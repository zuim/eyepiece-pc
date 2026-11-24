import Foundation
import ScreenCaptureKit
import VideoToolbox
import AppKit
import CoreImage
import Metal

@MainActor
final class AnimationView: NSView {
    var tick = 0
    var draws = 0
    override func draw(_ dirtyRect: NSRect) {
        draws += 1
        let context = NSGraphicsContext.current!.cgContext
        for index in 0..<16 {
            let shade = CGFloat((index + tick / 120) % 16) / 16
            context.setFillColor(CGColor(red: shade, green: 1 - shade, blue: 0.4, alpha: 1))
            let offset = CGFloat((tick * 3 + index * 60) % 960)
            context.fill(CGRect(x: offset, y: 0, width: 60, height: 540))
            context.fill(CGRect(x: offset - 960, y: 0, width: 60, height: 540))
        }
    }
}

func hostSeconds() -> Double {
    CMClockGetTime(CMClockGetHostTimeClock()).seconds
}

final class FrameTiming {
    let sample: CMSampleBuffer
    let arrived: Double
    let submitted: Double
    let index: Int

    init(sample: CMSampleBuffer, arrived: Double, submitted: Double, index: Int) {
        self.sample = sample
        self.arrived = arrived
        self.submitted = submitted
        self.index = index
    }
}

final class Benchmark: NSObject, SCStreamOutput, @unchecked Sendable {
    var session: VTCompressionSession?
    let lock = NSLock()
    let maxInFlight: Int
    let completeEachFrame: Bool
    var submitted = 0
    var completed = 0
    var dropped = 0
    var encoderDropped = 0
    var errors = 0
    var inFlight = 0
    var peakInFlight = 0
    var bytes = 0
    var encodeTimes: [Double] = []
    var callbackTimes: [Double] = []
    var packetTimes: [Double] = []
    var presentationTimes: [Double] = []
    var arrivalAges: [Double] = []
    var outputFile: FileHandle?
    let backgroundColor = CGColor(gray: 0, alpha: 1)
    let environment = ProcessInfo.processInfo.environment
    var heldSample: CMSampleBuffer?
    var keyframes = 0
    var largestFrame = 0
    var frameSizes: [Double] = []
    var invalidTimestamps = 0
    var negativeCaptureAges = 0
    var renderContext: CIContext?
    var renderPool: CVPixelBufferPool?
    let completedSignal = DispatchSemaphore(value: 0)
    let encodeQueue = DispatchQueue(label: "native-vt-encode", qos: .userInteractive)
    var queuedSample: (CMSampleBuffer, Double)?
    var workerActive = false

    init(maxInFlight: Int, completeEachFrame: Bool) {
        self.maxInFlight = maxInFlight
        self.completeEachFrame = completeEachFrame
    }

    func stream(_ stream: SCStream, didOutputSampleBuffer sample: CMSampleBuffer,
                of type: SCStreamOutputType) {
        guard type == .screen,
              let attachments = CMSampleBufferGetSampleAttachmentsArray(sample, createIfNecessary: false)
                as? [[SCStreamFrameInfo: Any]],
              attachments.first?[.status] as? Int == SCFrameStatus.complete.rawValue,
              sample.imageBuffer != nil else { return }
        let arrived = hostSeconds()
        if environment["VT_CALLBACK_WAIT"] == "1" {
            lock.lock()
            if queuedSample != nil { dropped += 1 }
            queuedSample = (sample, arrived)
            if workerActive { lock.unlock(); return }
            workerActive = true
            lock.unlock()
            encodeQueue.async {
                while true {
                    self.lock.lock()
                    let item = self.queuedSample
                    self.queuedSample = nil
                    if item == nil { self.workerActive = false }
                    self.lock.unlock()
                    guard let (sample, arrived) = item else { return }
                    self.submit(sample, arrived: arrived)
                }
            }
        } else {
            submit(sample, arrived: arrived)
        }
    }

    func submit(_ sample: CMSampleBuffer, arrived: Double) {
        guard let sourcePixels = sample.imageBuffer, let session else { return }
        lock.lock()
        if maxInFlight > 0 && inFlight >= maxInFlight {
            dropped += 1
            lock.unlock()
            return
        }
        let index = submitted
        submitted += 1
        inFlight += 1
        peakInFlight = max(peakInFlight, inFlight)
        if environment["VT_RETAIN_LAST"] == "1" { heldSample = sample }
        lock.unlock()
        if index == 0 {
            print("INPUT pixels=\(CVPixelBufferGetWidth(sourcePixels))x\(CVPixelBufferGetHeight(sourcePixels)) format=\(CVPixelBufferGetPixelFormatType(sourcePixels))")
        }
        var pixels = sourcePixels
        if let renderContext, let renderPool {
            var destination: CVPixelBuffer?
            guard CVPixelBufferPoolCreatePixelBuffer(nil, renderPool, &destination) == kCVReturnSuccess,
                  let destination else {
                lock.lock(); errors += 1; inFlight -= 1; lock.unlock()
                return
            }
            let image = CIImage(cvPixelBuffer: sourcePixels)
            renderContext.render(image, to: destination)
            pixels = destination
        }
        let timing = FrameTiming(sample: sample, arrived: arrived, submitted: hostSeconds(), index: index)
        let retained = Unmanaged.passRetained(timing).toOpaque()
        let idrInterval = Int(environment["VT_FORCE_IDR_EVERY"] ?? "0") ?? 0
        let status = VTCompressionSessionEncodeFrame(session, imageBuffer: pixels,
            presentationTimeStamp: sample.presentationTimeStamp, duration: sample.duration,
            frameProperties: index == 0 || (idrInterval > 0 && index % idrInterval == 0)
                ? [kVTEncodeFrameOptionKey_ForceKeyFrame: true] as CFDictionary : nil,
            sourceFrameRefcon: retained, infoFlagsOut: nil)
        if status != noErr {
            Unmanaged<FrameTiming>.fromOpaque(retained).release()
            lock.lock()
            errors += 1
            inFlight -= 1
            lock.unlock()
        } else if completeEachFrame {
            let status = VTCompressionSessionCompleteFrames(session, untilPresentationTimeStamp: sample.presentationTimeStamp)
            if status != noErr { print("CompleteFrames status=\(status)") }
        } else if environment["VT_CALLBACK_WAIT"] == "1" {
            if completedSignal.wait(timeout: .now() + 1) == .timedOut {
                lock.lock(); errors += 1; lock.unlock()
            }
        }
    }

    func output(_ timing: FrameTiming, status: OSStatus, sample: CMSampleBuffer?) {
        let now = hostSeconds()
        lock.lock()
        defer { lock.unlock() }
        defer {
            if environment["VT_SIMULATE_TIMEOUT"] != "1" { completedSignal.signal() }
        }
        inFlight -= 1
        guard status == noErr else {
            errors += 1
            return
        }
        guard let sample else { encoderDropped += 1; return }
        completed += 1
        let attachments = CMSampleBufferGetSampleAttachmentsArray(sample, createIfNecessary: false) as? [[CFString: Any]]
        if attachments?.first?[kCMSampleAttachmentKey_NotSync] as? Bool != true { keyframes += 1 }
        do {
            let packet = try annexB(sample)
            try outputFile?.write(contentsOf: packet)
        } catch {
            errors += 1
            print("bitstream packaging failed: \(error)")
        }
        let packetReady = hostSeconds()
        if timing.index >= 30 {
            let size = CMSampleBufferGetTotalSampleSize(sample)
            bytes += size
            largestFrame = max(largestFrame, size)
            frameSizes.append(Double(size))
            encodeTimes.append((now - timing.submitted) * 1000)
            callbackTimes.append((now - timing.arrived) * 1000)
            packetTimes.append((packetReady - timing.arrived) * 1000)
            let age = timing.arrived - timing.sample.presentationTimeStamp.seconds
            if age.isFinite && abs(age) < 10 {
                if age < 0 { negativeCaptureAges += 1 }
                arrivalAges.append(age * 1000)
                presentationTimes.append((packetReady - timing.sample.presentationTimeStamp.seconds) * 1000)
            } else {
                invalidTimestamps += 1
            }
        }
    }

    func annexB(_ sample: CMSampleBuffer) throws -> Data {
        guard let description = sample.formatDescription, let block = sample.dataBuffer else {
            throw NSError(domain: "missing compressed sample data", code: 1)
        }
        let hevc = CMFormatDescriptionGetMediaSubType(description) == kCMVideoCodecType_HEVC
        var data = Data()
        let prefix = Data([0, 0, 0, 1])
        let attachments = CMSampleBufferGetSampleAttachmentsArray(sample, createIfNecessary: false) as? [[CFString: Any]]
        let keyframe = attachments?.first?[kCMSampleAttachmentKey_NotSync] as? Bool != true
        var lengthBytes: Int32 = 4
        if keyframe {
            var count = 0
            var index = 0
            repeat {
                var pointer: UnsafePointer<UInt8>?
                var length = 0
                let status: OSStatus
                if hevc {
                    status = CMVideoFormatDescriptionGetHEVCParameterSetAtIndex(description,
                        parameterSetIndex: index, parameterSetPointerOut: &pointer,
                        parameterSetSizeOut: &length, parameterSetCountOut: &count,
                        nalUnitHeaderLengthOut: &lengthBytes)
                } else {
                    status = CMVideoFormatDescriptionGetH264ParameterSetAtIndex(description,
                        parameterSetIndex: index, parameterSetPointerOut: &pointer,
                        parameterSetSizeOut: &length, parameterSetCountOut: &count,
                        nalUnitHeaderLengthOut: &lengthBytes)
                }
                guard status == noErr, let pointer else { throw NSError(domain: "parameter sets", code: Int(status)) }
                data.append(prefix)
                data.append(pointer, count: length)
                index += 1
            } while index < count
        }
        let length = CMBlockBufferGetDataLength(block)
        var compressed = [UInt8](repeating: 0, count: length)
        let status = CMBlockBufferCopyDataBytes(block, atOffset: 0, dataLength: length, destination: &compressed)
        guard status == noErr else { throw NSError(domain: "compressed bytes", code: Int(status)) }
        var offset = 0
        while offset < length {
            guard offset + Int(lengthBytes) <= length else { throw NSError(domain: "NAL header", code: 1) }
            var nalSize = 0
            for byte in compressed[offset..<offset + Int(lengthBytes)] { nalSize = nalSize * 256 + Int(byte) }
            offset += Int(lengthBytes)
            guard nalSize > 0, offset + nalSize <= length else { throw NSError(domain: "NAL length", code: 1) }
            data.append(prefix)
            data.append(contentsOf: compressed[offset..<offset + nalSize])
            offset += nalSize
        }
        return data
    }

    func report(seconds: Double) {
        lock.lock()
        defer { lock.unlock() }
        func percentiles(_ values: [Double]) -> String {
            let sorted = values.sorted()
            guard !sorted.isEmpty else { return "n/a" }
            return String(format: "%.2f / %.2f", sorted[(sorted.count - 1) / 2], sorted[(sorted.count - 1) * 95 / 100])
        }
        print("submitted=\(submitted) completed=\(completed) dropped=\(dropped) errors=\(errors) peak-inflight=\(peakInFlight)")
        print(String(format: "fps=%.1f measured=%d", Double(completed) / seconds, encodeTimes.count))
        print("p50 / p95 ms: encode=\(percentiles(encodeTimes)) callback-to-output=\(percentiles(callbackTimes)) callback-to-packet=\(percentiles(packetTimes)) presentation-to-packet=\(percentiles(presentationTimes)) capture-age=\(percentiles(arrivalAges))")
        func values(_ data: [Double]) -> [Double] {
            let sorted = data.sorted()
            guard !sorted.isEmpty else { return [] }
            return [50, 95, 99].map { sorted[(sorted.count - 1) * $0 / 100] }
        }
        let deliveredFPS = Double(completed) / seconds
        let measuredDuration = max(seconds - 30.0 / max(deliveredFPS, 1.0), 0.01)
        let bitrate = Double(bytes) * 8.0 / measuredDuration / 1_000_000.0
        let result: [String: Any] = [
            "submitted": submitted, "completed": completed, "dropped": dropped, "errors": errors,
            "encoder_dropped": encoderDropped,
            "peak_inflight": peakInFlight, "keyframes": keyframes, "measured": packetTimes.count,
            "invalid_timestamps": invalidTimestamps, "fps": deliveredFPS,
            "negative_capture_ages": negativeCaptureAges,
            "packet_ms": values(packetTimes), "presentation_ms": values(presentationTimes),
            "capture_age_ms": values(arrivalAges), "frame_bytes": values(frameSizes),
            "largest_frame_bytes": largestFrame, "mbps": bitrate,
        ]
        if let data = try? JSONSerialization.data(withJSONObject: result, options: [.sortedKeys]),
           let text = String(data: data, encoding: .utf8) { print("RESULT \(text)") }
    }
}

@main
struct Main {
    static func main() async throws {
        setbuf(stdout, nil)
        let args = Array(CommandLine.arguments.dropFirst())
        let requestedDisplay = ProcessInfo.processInfo.environment["VT_DISPLAY_ID"].flatMap(UInt32.init)
        if args.first == "with-mode" {
            guard let requestedDisplay, let original = CGDisplayCopyDisplayMode(requestedDisplay),
                  let modes = CGDisplayCopyAllDisplayModes(requestedDisplay, nil) as? [CGDisplayMode],
                  let desired = modes.first(where: { $0.width == 1920 && $0.height == 1080 &&
                    $0.pixelWidth == 1920 && $0.pixelHeight == 1080 && abs($0.refreshRate - 100) < 0.1 }),
                  args.count > 1 else { throw NSError(domain: "native virtual 1080p mode or command unavailable", code: 1) }
            guard CGDisplaySetDisplayMode(requestedDisplay, desired, nil) == .success else {
                throw NSError(domain: "virtual display mode switch failed", code: 1)
            }
            defer {
                let result = CGDisplaySetDisplayMode(requestedDisplay, original, nil)
                print("RESTORE display=\(requestedDisplay) mode=\(original.width)x\(original.height) status=\(result.rawValue)")
            }
            print("MODE_SESSION display=\(requestedDisplay) mode=1920x1080@100 original=\(original.width)x\(original.height)")
            let process = Process()
            process.executableURL = URL(fileURLWithPath: "/usr/bin/env")
            process.arguments = Array(args.dropFirst())
            try process.run()
            process.waitUntilExit()
            guard process.terminationStatus == 0 else { throw NSError(domain: "benchmark child failed", code: Int(process.terminationStatus)) }
            return
        }
        if args.first == "list" {
            let content = try await SCShareableContent.excludingDesktopWindows(false, onScreenWindowsOnly: true)
            for display in content.displays {
                let current = CGDisplayCopyDisplayMode(display.displayID)
                print("DISPLAY id=\(display.displayID) logical=\(display.width)x\(display.height) backing=\(current?.pixelWidth ?? 0)x\(current?.pixelHeight ?? 0) refresh=\(current?.refreshRate ?? 0)")
                let modes = CGDisplayCopyAllDisplayModes(display.displayID, nil) as? [CGDisplayMode] ?? []
                for mode in modes {
                    print("MODE logical=\(mode.width)x\(mode.height) backing=\(mode.pixelWidth)x\(mode.pixelHeight) refresh=\(mode.refreshRate)")
                }
            }
            return
        }
        if args.first == "animate" {
            let app = NSApplication.shared
            app.setActivationPolicy(.accessory)
            let window = NSWindow(contentRect: NSRect(x: 40, y: 40, width: 960, height: 540),
                styleMask: [.titled], backing: .buffered, defer: false)
            if let requestedDisplay,
               let screen = NSScreen.screens.first(where: { ($0.deviceDescription[NSDeviceDescriptionKey("NSScreenNumber")] as? NSNumber)?.uint32Value == requestedDisplay }) {
                window.setFrameOrigin(NSPoint(x: screen.frame.minX + 40, y: screen.frame.minY + 40))
            }
            window.title = "Eyepiece benchmark motion"
            window.level = .floating
            let view = AnimationView(frame: NSRect(x: 0, y: 0, width: 960, height: 540))
            window.contentView = view
            window.orderFrontRegardless()
            let timer = Timer.scheduledTimer(withTimeInterval: 1.0 / 120, repeats: true) { _ in
                MainActor.assumeIsolated {
                    view.tick += 1
                    view.needsDisplay = true
                    if view.tick % 120 == 0 { print("ANIMATION ticks=\(view.tick) draws=\(view.draws)") }
                }
            }
            let activity = ProcessInfo.processInfo.beginActivity(options: [.userInitiatedAllowingIdleSystemSleep, .latencyCritical], reason: "Controlled capture benchmark")
            withExtendedLifetime((window, timer, activity)) { app.run() }
            return
        }
        let fps = args.indices.contains(0) ? Int(args[0])! : 60
        let seconds = args.indices.contains(1) ? Double(args[1])! : 6
        let frameDelay = args.indices.contains(2) ? Int(args[2])! : -1
        let queueDepth = args.indices.contains(3) ? Int(args[3])! : 3
        let lowLatency = args.indices.contains(4) ? args[4] != "0" : true
        let maxInFlight = args.indices.contains(5) ? Int(args[5])! : 0
        let completeEachFrame = args.indices.contains(6) ? args[6] != "0" : false
        let codec = args.indices.contains(7) && args[7] == "hevc" ? kCMVideoCodecType_HEVC : kCMVideoCodecType_H264
        let pixelFormat = args.indices.contains(8) && args[8] == "bgra" ? kCVPixelFormatType_32BGRA : kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange
        let width = args.indices.contains(9) ? Int(args[9])! : 1920
        let height = args.indices.contains(10) ? Int(args[10])! : 1080
        guard fps > 0, seconds > 0, queueDepth >= 1 && queueDepth <= 8 else {
            throw NSError(domain: "benchmark arguments", code: 1)
        }
        let benchmark = Benchmark(maxInFlight: maxInFlight, completeEachFrame: completeEachFrame)
        if args.indices.contains(11) {
            let path = args[11]
            guard FileManager.default.createFile(atPath: path, contents: nil) else {
                throw NSError(domain: "cannot create dump", code: 1)
            }
            benchmark.outputFile = try FileHandle(forWritingTo: URL(fileURLWithPath: path))
        }
        let start = hostSeconds()
        let specification = [
            kVTVideoEncoderSpecification_RequireHardwareAcceleratedVideoEncoder: true,
            kVTVideoEncoderSpecification_EnableLowLatencyRateControl: lowLatency,
        ] as CFDictionary
        let status = VTCompressionSessionCreate(allocator: nil, width: Int32(width), height: Int32(height),
            codecType: codec, encoderSpecification: specification, imageBufferAttributes: nil,
            compressedDataAllocator: nil, outputCallback: { context, source, status, _, sample in
                guard let context, let source else { return }
                let owner = Unmanaged<Benchmark>.fromOpaque(context).takeUnretainedValue()
                let timing = Unmanaged<FrameTiming>.fromOpaque(source).takeRetainedValue()
                owner.output(timing, status: status, sample: sample)
            }, refcon: Unmanaged.passUnretained(benchmark).toOpaque(), compressionSessionOut: &benchmark.session)
        guard status == noErr, let session = benchmark.session else {
            throw NSError(domain: "VTCompressionSessionCreate", code: Int(status))
        }
        defer { VTCompressionSessionInvalidate(session) }
        let properties: [(CFString, CFTypeRef)] = [
            (kVTCompressionPropertyKey_RealTime, kCFBooleanTrue),
            (kVTCompressionPropertyKey_AllowFrameReordering, kCFBooleanFalse),
            (kVTCompressionPropertyKey_AverageBitRate, NSNumber(value: 20_000_000)),
            (kVTCompressionPropertyKey_ExpectedFrameRate, NSNumber(value: fps)),
            (kVTCompressionPropertyKey_MaxKeyFrameInterval, NSNumber(value: fps * 2)),
            (kVTCompressionPropertyKey_ProfileLevel, codec == kCMVideoCodecType_H264 ? kVTProfileLevel_H264_High_AutoLevel : kVTProfileLevel_HEVC_Main_AutoLevel),
        ]
        for (key, value) in properties {
            let status = VTSessionSetProperty(session, key: key, value: value)
            guard status == noErr else { throw NSError(domain: "VT property \(key)", code: Int(status)) }
        }
        let environment = ProcessInfo.processInfo.environment
        let targetBitrate = Int(environment["VT_BITRATE"] ?? "20000000") ?? 20_000_000
        if targetBitrate != 20_000_000 {
            let status = VTSessionSetProperty(session, key: kVTCompressionPropertyKey_AverageBitRate, value: NSNumber(value: targetBitrate))
            print("PROPERTY VT_BITRATE=\(targetBitrate) status=\(status)")
        }
        let extraProperties: [(String, CFString)] = [
            ("VT_MAX_FPS", kVTCompressionPropertyKey_MaximumRealTimeFrameRate),
            ("VT_REFERENCES", kVTCompressionPropertyKey_ReferenceBufferCount),
            ("VT_POWER", kVTCompressionPropertyKey_MaximizePowerEfficiency),
            ("VT_SPEED", kVTCompressionPropertyKey_PrioritizeEncodingSpeedOverQuality),
            ("VT_LTR", kVTCompressionPropertyKey_EnableLTR),
            ("VT_LOOKAHEAD", kVTCompressionPropertyKey_SuggestedLookAheadFrameCount),
            ("VT_SPATIAL_AQ", kVTCompressionPropertyKey_SpatialAdaptiveQPLevel),
            ("VT_ALL_INTRA", kVTCompressionPropertyKey_AllowTemporalCompression),
            ("VT_GOP", kVTCompressionPropertyKey_MaxKeyFrameInterval),
            ("VT_QMAX", kVTCompressionPropertyKey_MaxAllowedFrameQP),
        ]
        var supported: CFDictionary?
        VTSessionCopySupportedPropertyDictionary(session, supportedPropertyDictionaryOut: &supported)
        let supportedDictionary = supported as? [String: Any] ?? [:]
        for (variable, key) in extraProperties {
            guard let text = environment[variable], let number = Int(text) else { continue }
            let boolean = ["VT_POWER", "VT_SPEED", "VT_LTR", "VT_ALL_INTRA"].contains(variable)
            let value: CFTypeRef = boolean ? (number != 0 ? kCFBooleanTrue : kCFBooleanFalse) : NSNumber(value: number)
            let status = VTSessionSetProperty(session, key: key, value: value)
            print("PROPERTY \(variable)=\(number) supported=\(supportedDictionary[key as String] != nil) status=\(status)")
        }
        if let text = environment["VT_RATE_WINDOW"], let duration = Double(text) {
            let limits = [NSNumber(value: Double(targetBitrate) * duration / 8), NSNumber(value: duration)] as CFArray
            let status = VTSessionSetProperty(session, key: kVTCompressionPropertyKey_DataRateLimits, value: limits)
            print("PROPERTY VT_RATE_WINDOW=\(duration) status=\(status)")
        }
        if frameDelay >= 0 {
            let status = VTSessionSetProperty(session, key: kVTCompressionPropertyKey_MaxFrameDelayCount, value: NSNumber(value: frameDelay))
            print("MaxFrameDelayCount=\(frameDelay) status=\(status)")
        }
        var identifier: UnsafeRawPointer?
        VTSessionCopyProperty(session, key: kVTCompressionPropertyKey_EncoderID, allocator: nil, valueOut: &identifier)
        let name = identifier.map { Unmanaged<CFString>.fromOpaque($0).takeRetainedValue() as String } ?? "unknown"
        print("encoder=\(name) fps=\(fps) delay=\(frameDelay) queue=\(queueDepth) low-latency=\(lowLatency) max-inflight=\(maxInFlight) complete=\(completeEachFrame) format=\(pixelFormat == kCVPixelFormatType_32BGRA ? "BGRA" : "NV12") size=\(width)x\(height)")
        let prepare = VTCompressionSessionPrepareToEncodeFrames(session)
        guard prepare == noErr else { throw NSError(domain: "VT prepare", code: Int(prepare)) }
        print(String(format: "encoder-open=%.1f ms", (hostSeconds() - start) * 1000))
        if environment["VT_DIRECT_GPU"] == "1" {
            guard let device = MTLCreateSystemDefaultDevice() else { throw NSError(domain: "no Metal device", code: 1) }
            let context = CIContext(mtlDevice: device)
            let attributes = [kCVPixelBufferWidthKey: width, kCVPixelBufferHeightKey: height,
                kCVPixelBufferPixelFormatTypeKey: kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange,
                kCVPixelBufferMetalCompatibilityKey: true, kCVPixelBufferIOSurfacePropertiesKey: [:]] as CFDictionary
            var pool: CVPixelBufferPool?
            guard CVPixelBufferPoolCreate(nil, nil, attributes, &pool) == kCVReturnSuccess, let pool else {
                throw NSError(domain: "direct GPU pool", code: 1)
            }
            let start = hostSeconds()
            var index = 0
            while hostSeconds() - start < seconds {
                let arrived = hostSeconds()
                var pixels: CVPixelBuffer?
                guard CVPixelBufferPoolCreatePixelBuffer(nil, pool, &pixels) == kCVReturnSuccess, let pixels else {
                    throw NSError(domain: "direct GPU pixels", code: 1)
                }
                let image = CIFilter(name: "CICheckerboardGenerator", parameters: [
                    "inputCenter": CIVector(x: Double(index * 4), y: 0), "inputWidth": 40.0])!.outputImage!
                    .cropped(to: CGRect(x: 0, y: 0, width: width, height: height))
                context.render(image, to: pixels)
                var description: CMVideoFormatDescription?
                CMVideoFormatDescriptionCreateForImageBuffer(allocator: nil, imageBuffer: pixels, formatDescriptionOut: &description)
                var timing = CMSampleTimingInfo(duration: CMTime(value: 1, timescale: CMTimeScale(fps)),
                    presentationTimeStamp: CMTime(seconds: arrived, preferredTimescale: 1_000_000_000), decodeTimeStamp: .invalid)
                var sample: CMSampleBuffer?
                guard let description,
                      CMSampleBufferCreateReadyWithImageBuffer(allocator: nil, imageBuffer: pixels,
                        formatDescription: description, sampleTiming: &timing, sampleBufferOut: &sample) == noErr,
                      let sample else { throw NSError(domain: "direct GPU sample", code: 1) }
                benchmark.submit(sample, arrived: arrived)
                index += 1
                let wait = start + Double(index) / Double(fps) - hostSeconds()
                if wait > 0 { try await Task.sleep(nanoseconds: UInt64(wait * 1_000_000_000)) }
            }
            VTCompressionSessionCompleteFrames(session, untilPresentationTimeStamp: .invalid)
            benchmark.report(seconds: hostSeconds() - start)
            try benchmark.outputFile?.close()
            guard benchmark.errors == 0 && benchmark.completed == benchmark.submitted else {
                throw NSError(domain: "direct GPU encoding failure", code: 1)
            }
            return
        }
        let content = try await SCShareableContent.excludingDesktopWindows(false, onScreenWindowsOnly: true)
        guard let display = content.displays.first(where: { requestedDisplay == nil || $0.displayID == requestedDisplay }) else {
            throw NSError(domain: "requested display unavailable", code: 1)
        }
        let refresh = CGDisplayCopyDisplayMode(display.displayID)?.refreshRate ?? 0
        let mode = CGDisplayCopyDisplayMode(display.displayID)
        print("DISPLAY id=\(display.displayID) size=\(display.width)x\(display.height) backing=\(mode?.pixelWidth ?? 0)x\(mode?.pixelHeight ?? 0) refresh=\(refresh)")
        let noScale = environment["VT_NO_SCALE"] == "1"
        if noScale && (mode?.pixelWidth != width || mode?.pixelHeight != height) {
            throw NSError(domain: "no-scale capture requires matching display backing pixels and output dimensions", code: 1)
        }
        var filter = SCContentFilter(display: display, excludingWindows: [])
        var sourceSize = CGSize(width: display.width, height: display.height)
        if environment["VT_WINDOW"] == "1" {
            guard let window = content.windows.first(where: { $0.title == "Eyepiece benchmark motion" }) else {
                throw NSError(domain: "benchmark animation window not found", code: 1)
            }
            filter = SCContentFilter(desktopIndependentWindow: window)
            sourceSize = window.frame.size
        }
        let configuration = SCStreamConfiguration()
        configuration.width = width
        configuration.height = height
        configuration.pixelFormat = pixelFormat
        configuration.minimumFrameInterval = CMTime(value: 1, timescale: CMTimeScale(fps))
        configuration.queueDepth = queueDepth
        configuration.showsCursor = false
        configuration.scalesToFit = !noScale
        print("CAPTURE scalesToFit=\(!noScale) output=\(width)x\(height)")
        if environment["VT_CROP"] == "1" {
            guard let window = content.windows.first(where: { $0.title == "Eyepiece benchmark motion" }) else {
                throw NSError(domain: "benchmark crop window not found", code: 1)
            }
            configuration.sourceRect = window.frame
            sourceSize = window.frame.size
        }
        let scale = min(Double(width) / sourceSize.width, Double(height) / sourceSize.height)
        let pictureWidth = floor(sourceSize.width * scale / 2) * 2
        let pictureHeight = floor(sourceSize.height * scale / 2) * 2
        configuration.destinationRect = CGRect(
            x: floor((Double(width) - pictureWidth) / 4) * 2,
            y: floor((Double(height) - pictureHeight) / 4) * 2,
            width: pictureWidth, height: pictureHeight)
        configuration.backgroundColor = benchmark.backgroundColor
        configuration.colorSpaceName = CGColorSpace.itur_709
        configuration.colorMatrix = CGDisplayStream.yCbCrMatrix_ITU_R_709_2
        if environment["VT_GPU_CONVERT"] == "1" {
            guard let device = MTLCreateSystemDefaultDevice() else { throw NSError(domain: "no Metal device", code: 1) }
            benchmark.renderContext = CIContext(mtlDevice: device)
            let attributes = [
                kCVPixelBufferWidthKey: width, kCVPixelBufferHeightKey: height,
                kCVPixelBufferPixelFormatTypeKey: kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange,
                kCVPixelBufferMetalCompatibilityKey: true,
                kCVPixelBufferIOSurfacePropertiesKey: [:],
            ] as CFDictionary
            let status = CVPixelBufferPoolCreate(nil, nil, attributes, &benchmark.renderPool)
            guard status == kCVReturnSuccess else { throw NSError(domain: "Metal destination pool", code: Int(status)) }
        }
        let stream = SCStream(filter: filter, configuration: configuration, delegate: nil)
        try stream.addStreamOutput(benchmark, type: .screen,
            sampleHandlerQueue: DispatchQueue(label: "native-vt-capture", qos: .userInteractive))
        try await stream.startCapture()
        let captureStart = hostSeconds()
        try await Task.sleep(nanoseconds: UInt64(seconds * 1_000_000_000))
        try await stream.stopCapture()
        benchmark.encodeQueue.sync {}
        let elapsed = hostSeconds() - captureStart
        let flush = VTCompressionSessionCompleteFrames(session, untilPresentationTimeStamp: .invalid)
        guard flush == noErr else { throw NSError(domain: "VT flush", code: Int(flush)) }
        benchmark.report(seconds: elapsed)
        try benchmark.outputFile?.close()
        guard benchmark.errors == 0 && benchmark.completed + benchmark.encoderDropped == benchmark.submitted && benchmark.inFlight == 0 else {
            throw NSError(domain: "incomplete or failed encoding", code: 1)
        }
    }
}

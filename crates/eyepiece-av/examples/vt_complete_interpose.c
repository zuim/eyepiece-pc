#include <VideoToolbox/VideoToolbox.h>

static OSStatus complete_encode(
    VTCompressionSessionRef session,
    CVImageBufferRef image,
    CMTime pts,
    CMTime duration,
    CFDictionaryRef properties,
    void *source,
    VTEncodeInfoFlags *flags)
{
    OSStatus result = VTCompressionSessionEncodeFrame(
        session, image, pts, duration, properties, source, flags);
    if (result == noErr)
        result = VTCompressionSessionCompleteFrames(session, pts);
    return result;
}

__attribute__((used)) static const struct {
    const void *replacement;
    const void *original;
} interpose __attribute__((section("__DATA,__interpose"))) = {
    (const void *)complete_encode,
    (const void *)VTCompressionSessionEncodeFrame,
};

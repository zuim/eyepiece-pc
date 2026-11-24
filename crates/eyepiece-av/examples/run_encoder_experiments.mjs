import { spawn } from 'node:child_process';
import fs from 'node:fs';

const duration = Number(process.env.BENCH_SECONDS ?? 4);
const repeats = Number(process.env.BENCH_REPEATS ?? 3);
let animation;
let animationLog = '';
if (process.env.BENCH_ANIMATE === '1') {
    animation = spawn('target/native-vt-bench', ['animate'], { env: process.env, stdio: ['ignore', 'pipe', 'pipe'] });
    animation.stdout.on('data', data => { animationLog += data; });
    process.on('exit', () => animation.kill('SIGTERM'));
    await new Promise(resolve => setTimeout(resolve, 700));
}
const variants = [
    ['virtual-scale', { VT_NO_SCALE: '0' }, []],
    ['virtual-no-scale', { VT_NO_SCALE: '1' }, []],
    ['virtual-wait', { VT_NO_SCALE: '1', VT_CALLBACK_WAIT: '1' }, []],
    ['virtual60', { VT_NO_SCALE: '1' }, ['60']],
    ['virtual-bgra', { VT_NO_SCALE: '1' }, ['100', '', '-1', '3', '1', '0', '0', 'h264', 'bgra']],
    ['native100', {}, []], ['native60', {}, ['60']],
    ['maxfps60', { VT_MAX_FPS: '60' }, []], ['maxfps100', { VT_MAX_FPS: '100' }, []],
    ['maxfps120', { VT_MAX_FPS: '120' }, []],
    ['callback-wait', { VT_CALLBACK_WAIT: '1' }, []], ['complete', {}, ['100', '', '-1', '3', '1', '0', '1']],
    ['retain-last', { VT_RETAIN_LAST: '1' }, []],
    ['queue1', {}, ['100', '', '-1', '1']], ['queue2', {}, ['100', '', '-1', '2']],
    ['queue4', {}, ['100', '', '-1', '4']], ['queue8', {}, ['100', '', '-1', '8']],
    ['inflight1', {}, ['100', '', '-1', '3', '1', '1']], ['inflight2', {}, ['100', '', '-1', '3', '1', '2']],
    ['rate-window1s', { VT_RATE_WINDOW: '1' }, []], ['rate-window40ms', { VT_RATE_WINDOW: '0.04' }, []],
    ['ltr-off', { VT_LTR: '0' }, []], ['ltr-on', { VT_LTR: '1' }, []],
    ['references1', { VT_REFERENCES: '1' }, []], ['references2', { VT_REFERENCES: '2' }, []],
    ['aq-off', { VT_SPATIAL_AQ: '0' }, []], ['aq-on', { VT_SPATIAL_AQ: '1' }, []],
    ['power-off', { VT_POWER: '0' }, []], ['power-on', { VT_POWER: '1' }, []],
    ['speed', { VT_SPEED: '1' }, []], ['frame-delay1', {}, ['100', '', '1']],
    ['lookahead0', { VT_LOOKAHEAD: '0' }, []],
    ['standard-rc', {}, ['100', '', '-1', '3', '0']],
    ['standard-lookahead0', { VT_LOOKAHEAD: '0' }, ['100', '', '-1', '3', '0']],
    ['all-intra', { VT_ALL_INTRA: '0' }, []], ['idr-every30', { VT_FORCE_IDR_EVERY: '30' }, []],
    ['gop600', { VT_GOP: '600' }, []], ['qmax20', { VT_QMAX: '20' }, []],
    ['window', { VT_WINDOW: '1' }, []], ['crop', { VT_CROP: '1' }, []],
    ['bgra', {}, ['100', '', '-1', '3', '1', '0', '0', 'h264', 'bgra']],
    ['gpu-convert', { VT_GPU_CONVERT: '1' }, ['100', '', '-1', '3', '1', '0', '0', 'h264', 'bgra']],
    ['direct-gpu', { VT_DIRECT_GPU: '1' }, []],
    ['hevc', {}, ['100', '', '-1', '3', '1', '0', '0', 'hevc']],
    ['720p', {}, ['100', '', '-1', '3', '1', '0', '0', 'h264', 'nv12', '1280', '720']],
    ['reference-aq-combined', { VT_REFERENCES: '1', VT_SPATIAL_AQ: '0' }, []],
    ['stress-target', { VT_BITRATE: '2000000', VT_FORCE_IDR_EVERY: '15' }, []],
    ['stress-window1s', { VT_BITRATE: '2000000', VT_FORCE_IDR_EVERY: '15', VT_RATE_WINDOW: '1' }, []],
    ['stress-window40ms', { VT_BITRATE: '2000000', VT_FORCE_IDR_EVERY: '15', VT_RATE_WINDOW: '0.04' }, []],
    ['forced-all-intra', { VT_FORCE_IDR_EVERY: '1' }, []],
    ['standard-all-intra', { VT_ALL_INTRA: '0' }, ['100', '', '-1', '3', '0']],
    ['timeout-check', { VT_CALLBACK_WAIT: '1', VT_SIMULATE_TIMEOUT: '1' }, []],
];
const filter = process.env.BENCH_FILTER ? new RegExp(process.env.BENCH_FILTER) : null;
const records = [];
const output = process.env.BENCH_OUTPUT ?? 'target/encoder-experiments.json';
const loadWorkers = [];
for (let index = 0; index < Number(process.env.BENCH_CPU_WORKERS ?? 0); index++) {
    const worker = spawn(process.execPath, ['-e', 'const crypto=require("node:crypto");while(true)crypto.pbkdf2Sync("benchmark","contention",10000,64,"sha256");'], { stdio: 'ignore' });
    loadWorkers.push(worker);
    process.on('exit', () => worker.kill('SIGTERM'));
}
if (process.env.BENCH_SESSION === '1') {
    for (let repeat = 0; repeat < repeats; repeat++) {
        for (const encoder of repeat % 2 ? ['videotoolbox', 'x264'] : ['x264', 'videotoolbox']) {
            let stdout = '', stderr = '';
            const args = ['--source', 'screencapturekit', '--no-connect', '--encoder', encoder,
                '--fps', process.env.BENCH_FPS ?? '120', '--duration', String(duration), '--stats-interval', '2'];
            if (process.env.VT_DISPLAY_ID) args.push('--display-id', process.env.VT_DISPLAY_ID);
            const child = spawn('/usr/bin/time', ['-l', 'target/release/eyepiece-pc', ...args], { env: process.env, stdio: ['ignore', 'pipe', 'pipe'] });
            child.stdout.on('data', data => { stdout += data; });
            child.stderr.on('data', data => { stderr += data; });
            const code = await new Promise(resolve => child.on('close', resolve));
            records.push({ encoder, repeat, args, code, stdout, stderr });
            fs.writeFileSync(output, JSON.stringify({ repeats, duration, animation_log: animationLog, records }, null, 2));
            console.log(`${encoder} session #${repeat + 1}: code=${code}\n${stderr.split('\n').filter(line => line.includes('sent ') || line.includes('pc:')).join('\n')}`);
        }
    }
    process.exit(records.some(record => record.code !== 0) ? 1 : 0);
}
if (process.env.BENCH_COMPARE === '1') {
    const cases = [];
    for (const fps of [60, 100, 120]) {
        for (const [encoder, mode, preset] of [
            ['x264', 'readback', 'superfast'], ['videotoolbox', 'readback', null],
            ['x264', 'bgra', 'superfast'], ['videotoolbox', 'bgra', null],
            ['x264', 'readback', 'ultrafast'], ['x264', 'readback', 'veryfast'],
        ]) cases.push({ name: `${encoder}-${mode}-${preset ?? 'default'}-${fps}`, encoder, mode, preset, fps });
    }
    for (let repeat = 0; repeat < repeats; repeat++) {
        const ordered = repeat % 2 ? [...cases].reverse() : cases;
        for (const test of ordered) {
            if (filter && !filter.test(test.name)) continue;
            const count = process.env.BENCH_FRAMES ?? '240';
            const args = [test.encoder, 'h264', String(test.fps), count, test.mode];
            let stdout = '', stderr = '';
            const env = { ...process.env };
            delete env.BENCH_PRESET;
            if (test.preset) env.BENCH_PRESET = test.preset;
            const child = spawn('/usr/bin/time', ['-l', 'target/release/examples/encoder_bench', ...args], {
                env, stdio: ['ignore', 'pipe', 'pipe'],
            });
            child.stdout.on('data', data => { stdout += data; });
            child.stderr.on('data', data => { stderr += data; });
            const timeout = setTimeout(() => child.kill('SIGTERM'), 60000);
            const code = await new Promise(resolve => child.on('close', resolve));
            clearTimeout(timeout);
            const line = stdout.split('\n').find(line => line.startsWith('RESULT '));
            const result = line ? JSON.parse(line.slice(7)) : null;
            const timing = stderr.match(/([\d.]+) real\s+([\d.]+) user\s+([\d.]+) sys/);
            const rss = stderr.match(/(\d+)\s+maximum resident set size/);
            const real = timing ? Number(timing[1]) : null;
            const cpu = timing ? Number(timing[2]) + Number(timing[3]) : null;
            records.push({
                ...test, repeat, count: Number(count), args, code, result, stdout, stderr,
                cpu_seconds: cpu, cpu_percent: cpu !== null && real ? cpu / real * 100 : null,
                max_rss_bytes: rss ? Number(rss[1]) : null
            });
            fs.writeFileSync(output, JSON.stringify({ repeats, cpu_workers: loadWorkers.length, display_id: process.env.VT_DISPLAY_ID ?? 'default', animation_log: animationLog, records }, null, 2));
            console.log(`${test.name} #${repeat + 1}: code=${code} fps=${result?.fps.toFixed(1)} capture=${result?.capture_ms.map(value => value.toFixed(2)).join('/')} cpu=${cpu !== null && real ? (cpu / real * 100).toFixed(1) : '-'}%`);
        }
    }
    if (animation) await new Promise(resolve => { animation.once('close', resolve); animation.kill('SIGTERM'); });
    process.exit(records.some(record => record.code !== 0 || !record.result || record.result.frames !== record.result.packets) ? 1 : 0);
}
if (process.env.BENCH_FFMPEG === '1') {
    const cases = [
        ['x264', ['x264', 'h264', '100', '600']], ['x265', ['x265', 'hevc', '100', '600']],
        ['vt-h264', ['videotoolbox', 'h264', '100', '600']], ['vt-hevc', ['videotoolbox', 'hevc', '100', '600']],
        ['ff-native60', ['videotoolbox', 'h264', '60', '120', 'native']],
        ['ff-native100', ['videotoolbox', 'h264', '100', '120', 'native']],
        ['ff-complete60', ['videotoolbox', 'h264', '60', '120', 'native'], true],
        ['ff-complete100', ['videotoolbox', 'h264', '100', '120', 'native'], true],
        ['ff-readback60', ['videotoolbox', 'h264', '60', '120', 'readback']],
        ['ff-readback100', ['videotoolbox', 'h264', '100', '120', 'readback']],
    ];
    for (let repeat = 0; repeat < repeats; repeat++) {
        for (const [name, args, interpose] of cases) {
            if (filter && !filter.test(name)) continue;
            let stdout = '', stderr = '';
            const environment = interpose ? { DYLD_INSERT_LIBRARIES: `${process.cwd()}/target/vt-complete-interpose.dylib` } : {};
            const command = interpose ? ['/usr/bin/env', `DYLD_INSERT_LIBRARIES=${environment.DYLD_INSERT_LIBRARIES}`, 'target/release/examples/encoder_bench', ...args]
                : ['target/release/examples/encoder_bench', ...args];
            const child = spawn('/usr/bin/time', ['-l', ...command], {
                env: { ...process.env, ...environment }, stdio: ['ignore', 'pipe', 'pipe'],
            });
            child.stdout.on('data', data => { stdout += data; });
            child.stderr.on('data', data => { stderr += data; });
            const timeout = setTimeout(() => child.kill('SIGTERM'), 30000);
            const code = await new Promise(resolve => child.on('close', resolve));
            clearTimeout(timeout);
            const metrics = stdout.match(/encode p50 ([\d.]+) p95 ([\d.]+) ms \| packet p50 ([\d.]+) p95 ([\d.]+) ms.*pipeline ([\d.]+) fps/);
            const capture = stdout.match(/capture-callback-to-packet p50 ([\d.]+) p95 ([\d.]+) ms/);
            records.push({
                name, repeat, args, environment, code, stdout, stderr,
                encode_ms: metrics ? [Number(metrics[1]), Number(metrics[2])] : [],
                packet_ms: metrics ? [Number(metrics[3]), Number(metrics[4])] : [],
                fps: metrics ? Number(metrics[5]) : null,
                capture_ms: capture ? [Number(capture[1]), Number(capture[2])] : []
            });
            fs.writeFileSync(output, JSON.stringify({ repeats, animation_log: animationLog, records }, null, 2));
            console.log(`${name} #${repeat + 1}: code=${code} ${metrics ? `encode=${metrics[1]}/${metrics[2]} packet=${metrics[3]}/${metrics[4]} fps=${metrics[5]}` : 'no metrics'}`);
        }
    }
    process.exit(records.some(record => record.code !== 0) ? 1 : 0);
}
for (let repeat = 0; repeat < repeats; repeat++) {
    for (const [name, environment, overrides] of variants) {
        if (name.startsWith('virtual') && process.env.BENCH_VIRTUAL !== '1') continue;
        if (filter && !filter.test(name)) continue;
        const args = ['100', String(duration), '-1', '3', '1', '0', '0', 'h264', 'nv12', '1920', '1080'];
        overrides.forEach((value, index) => { if (value) args[index] = value; });
        const started = Date.now();
        let stdout = '';
        let stderr = '';
        const child = spawn('/usr/bin/time', ['-l', 'target/native-vt-bench', ...args], {
            env: { ...process.env, ...environment }, stdio: ['ignore', 'pipe', 'pipe'],
        });
        child.stdout.on('data', data => { stdout += data; });
        child.stderr.on('data', data => { stderr += data; });
        const timeout = setTimeout(() => child.kill('SIGTERM'), (duration + 15) * 1000);
        const code = await new Promise(resolve => child.on('close', resolve));
        clearTimeout(timeout);
        const line = stdout.split('\n').find(line => line.startsWith('RESULT '));
        const result = line ? JSON.parse(line.slice(7)) : null;
        const timing = stderr.match(/([\d.]+) real\s+([\d.]+) user\s+([\d.]+) sys/);
        const rss = stderr.match(/(\d+)\s+maximum resident set size/);
        const record = {
            name, repeat, args, environment, code, elapsed_ms: Date.now() - started,
            cpu_seconds: timing ? Number(timing[2]) + Number(timing[3]) : null,
            max_rss_bytes: rss ? Number(rss[1]) : null, result,
            properties: stdout.split('\n').filter(line => line.startsWith('PROPERTY ')), stdout, stderr
        };
        records.push(record);
        fs.writeFileSync(output, JSON.stringify({ duration, repeats, animation_log: animationLog, records }, null, 2));
        console.log(`${name} #${repeat + 1}: code=${code} fps=${result?.fps.toFixed(1) ?? '-'} packet=${result?.packet_ms.map(value => value.toFixed(2)).join('/') ?? '-'} drops=${result?.dropped ?? '-'} errors=${result?.errors ?? '-'}`);
    }
}
console.log(`Saved ${records.length} runs to ${output}`);
if (animation) {
    await new Promise(resolve => {
        animation.once('close', resolve);
        animation.kill('SIGTERM');
    });
}

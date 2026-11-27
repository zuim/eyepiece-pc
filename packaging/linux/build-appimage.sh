#!/bin/sh
# Build an AppImage with eyepiece-pc (Linux x86_64).
#
#   packaging/linux/build-appimage.sh [--features pipewire]
#
# Portable build (what CI does): on an older base (Ubuntu 24.04, glibc 2.39) build FFmpeg with
# build-ffmpeg.sh, then
#   FFMPEG_INCLUDE_DIR=$P/include FFMPEG_LIB_DIR=$P/lib LD_LIBRARY_PATH=$P/lib build-appimage.sh
# Built against the system FFmpeg instead, the AppImage needs a glibc as new as the build
# machine's.
#
# Needs linuxdeploy (https://github.com/linuxdeploy/linuxdeploy, the x86_64 AppImage; it
# includes the AppImage output plugin): on PATH as linuxdeploy / linuxdeploy-x86_64.AppImage,
# or LINUXDEPLOY=/path/to/linuxdeploy-x86_64.AppImage. Without it the AppDir is still built
# (target/appimage/AppDir, runnable with its AppRun) and the script exits with status 3.
#
# The host supplies glibc, GPU drivers, libdrm and libpipewire. The AppImage cannot carry file
# capabilities, so KMS capture (CAP_SYS_ADMIN) does not work from it: use --source pipewire /
# x11, or a natively installed eyepiece-pc with setcap.
set -eu
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
cd "$ROOT"
TARGET="${CARGO_TARGET_DIR:-$ROOT/target}"

# the built web UI (cd web && pnpm build), served by every start
WEB="${WEB_DIR:-$ROOT/web/build}"
if [ ! -f "$WEB/index.html" ]; then
    echo "The web UI is not built: run pnpm build in web/ (or set WEB_DIR)" >&2
    exit 1
fi

cargo build --release --locked -p eyepiece-pc "$@"

OUT="$TARGET/appimage"
APPDIR="$OUT/AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin" "$APPDIR/usr/share/applications" "$APPDIR/usr/share/icons/hicolor/scalable/apps"
strip -o "$APPDIR/usr/bin/eyepiece-pc" "$TARGET/release/eyepiece-pc"
cp "$HERE/eyepiece-pc.desktop" "$APPDIR/usr/share/applications/"
cp "$HERE/eyepiece-pc.svg" "$APPDIR/usr/share/icons/hicolor/scalable/apps/"
cp README.md LICENSE "$APPDIR/"
cp -R "$WEB" "$APPDIR/usr/bin/web"

cat > "$OUT/AppRun" <<'EOF'
#!/bin/sh
HERE="$(dirname "$(readlink -f "$0")")"
exec "$HERE/usr/bin/eyepiece-pc" "$@"
EOF
chmod +x "$OUT/AppRun"

LD="${LINUXDEPLOY:-}"
if [ -z "$LD" ]; then
    LD="$(command -v linuxdeploy || command -v linuxdeploy-x86_64.AppImage || true)"
fi
if [ -z "$LD" ]; then
    ln -sf usr/share/applications/eyepiece-pc.desktop "$APPDIR/eyepiece-pc.desktop"
    ln -sf usr/share/icons/hicolor/scalable/apps/eyepiece-pc.svg "$APPDIR/eyepiece-pc.svg"
    cp "$OUT/AppRun" "$APPDIR/AppRun"
    echo "AppDir ready: $APPDIR (linuxdeploy not found, no AppImage built; set LINUXDEPLOY)" >&2
    exit 3
fi

cd "$OUT"
VERSION="${VERSION:-$(git -C "$ROOT" describe --tags --always 2>/dev/null || echo dev)}"
export VERSION
export LDAI_OUTPUT="eyepiece-pc-$VERSION-linux-x86_64.AppImage"
export APPIMAGE_EXTRACT_AND_RUN=1
"$LD" --appdir "$APPDIR" \
    --executable "$APPDIR/usr/bin/eyepiece-pc" \
    --desktop-file "$APPDIR/usr/share/applications/eyepiece-pc.desktop" \
    --icon-file "$APPDIR/usr/share/icons/hicolor/scalable/apps/eyepiece-pc.svg" \
    --custom-apprun "$OUT/AppRun" \
    --exclude-library 'libva*' --exclude-library 'libvdpau*' --exclude-library 'libOpenCL*' \
    --exclude-library 'libvulkan*' --exclude-library 'libcuda*' --exclude-library 'libnvidia*' \
    --exclude-library 'libdrm*' --exclude-library 'libpipewire*' --exclude-library 'libspa*' \
    --output appimage
echo "built $OUT/$LDAI_OUTPUT"

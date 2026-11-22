#!/bin/sh
# Build an AppImage with eyepiece-gui and eyepiece-pc (Linux x86_64).
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
# What is bundled: the two executables and the FFmpeg libraries they link, with their
# dependencies. What is NOT bundled (taken from the host, on purpose):
#   - glibc, libGL/libEGL, libwayland, libX11, libxkbcommon (linuxdeploy's exclude list;
#     the GUI loads the windowing/GL libraries at run time anyway)
#   - libva / libvdpau / libOpenCL / libvulkan / libcuda / libnvidia-*: they load the
#     host's GPU drivers and must match them
#   - libdrm, libpipewire / libspa (talk to the host kernel / PipeWire daemon)
# The AppImage cannot carry file capabilities, so KMS capture (CAP_SYS_ADMIN) does not work
# from it: use --source pipewire / x11, or a natively installed eyepiece-pc with setcap.
set -eu
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
cd "$ROOT"
TARGET="${CARGO_TARGET_DIR:-$ROOT/target}"

cargo build --release --locked -p eyepiece-gui -p eyepiece-pc "$@"

OUT="$TARGET/appimage"
APPDIR="$OUT/AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin" "$APPDIR/usr/share/applications" "$APPDIR/usr/share/icons/hicolor/scalable/apps"
for b in eyepiece-gui eyepiece-pc; do
    # keep symbols out of the AppImage (the release profile has debug = 1)
    strip -o "$APPDIR/usr/bin/$b" "$TARGET/release/$b"
done
cp "$HERE/eyepiece-gui.desktop" "$APPDIR/usr/share/applications/"
cp "$HERE/eyepiece-gui.svg" "$APPDIR/usr/share/icons/hicolor/scalable/apps/"
cp README.md LICENSE "$APPDIR/"
# a bridge to push (pcdisplay_bridge, legacy eyepiece_bridge): next to the executables if you have one
for b in pcdisplay_bridge eyepiece_bridge; do
    if [ -f "$HERE/$b" ]; then cp "$HERE/$b" "$APPDIR/usr/bin/"; fi
done

# one AppImage, two programs: a symlink named eyepiece-pc… runs the CLI (AppImage sets
# $ARGV0 to the name it was started as), everything else the GUI
cat > "$OUT/AppRun" <<'EOF'
#!/bin/sh
HERE="$(dirname "$(readlink -f "$0")")"
case "$(basename "${ARGV0:-$0}")" in
    eyepiece-pc*) exec "$HERE/usr/bin/eyepiece-pc" "$@" ;;
    *) exec "$HERE/usr/bin/eyepiece-gui" "$@" ;;
esac
EOF
chmod +x "$OUT/AppRun"

LD="${LINUXDEPLOY:-}"
if [ -z "$LD" ]; then
    LD="$(command -v linuxdeploy || command -v linuxdeploy-x86_64.AppImage || true)"
fi
if [ -z "$LD" ]; then
    # plain AppDir, runnable in place with the host's FFmpeg
    ln -sf usr/share/applications/eyepiece-gui.desktop "$APPDIR/eyepiece-gui.desktop"
    ln -sf usr/share/icons/hicolor/scalable/apps/eyepiece-gui.svg "$APPDIR/eyepiece-gui.svg"
    cp "$OUT/AppRun" "$APPDIR/AppRun"
    echo "AppDir ready: $APPDIR (linuxdeploy not found, no AppImage built; set LINUXDEPLOY)" >&2
    exit 3
fi

cd "$OUT"
VERSION="${VERSION:-$(git -C "$ROOT" describe --tags --always 2>/dev/null || echo dev)}"
export VERSION
export LDAI_OUTPUT="eyepiece-pc-$VERSION-linux-x86_64.AppImage"
# no FUSE needed (containers, CI)
export APPIMAGE_EXTRACT_AND_RUN=1
"$LD" --appdir "$APPDIR" \
    --executable "$APPDIR/usr/bin/eyepiece-gui" \
    --executable "$APPDIR/usr/bin/eyepiece-pc" \
    --desktop-file "$APPDIR/usr/share/applications/eyepiece-gui.desktop" \
    --icon-file "$APPDIR/usr/share/icons/hicolor/scalable/apps/eyepiece-gui.svg" \
    --custom-apprun "$OUT/AppRun" \
    --exclude-library 'libva*' --exclude-library 'libvdpau*' --exclude-library 'libOpenCL*' \
    --exclude-library 'libvulkan*' --exclude-library 'libcuda*' --exclude-library 'libnvidia*' \
    --exclude-library 'libdrm*' --exclude-library 'libpipewire*' --exclude-library 'libspa*' \
    --output appimage
echo "built $OUT/$LDAI_OUTPUT"

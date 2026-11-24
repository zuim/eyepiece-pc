#!/usr/bin/env bash
set -euo pipefail

version=${VERSION:?Set VERSION to a release tag or commit identifier}
app_version=${APP_VERSION:-$(sed -n 's/^version = "\([0-9][0-9.]*\)"$/\1/p' Cargo.toml | head -n 1)}
minimum_os=${MACOSX_DEPLOYMENT_TARGET:-13.0}
output_dir=${OUTPUT_DIR:-dist}
binary_dir=${BINARY_DIR:-target/release}
arch=$(uname -m)
name="eyepiece-pc-${version}-macos-${arch}"
staging="${output_dir}/${name}"
app="${staging}/Eyepiece.app"
dmg="${output_dir}/${name}.dmg"

if [[ -e "$staging" || -e "$dmg" ]]; then
    echo "Refusing to overwrite existing package: $staging or $dmg" >&2
    exit 1
fi
mkdir -p "$app/Contents/MacOS" "$app/Contents/Frameworks" "$app/Contents/Resources"
cp packaging/macos/Info.plist "$app/Contents/Info.plist"
cp "$binary_dir/eyepiece-gui" "$binary_dir/eyepiece-pc" "$app/Contents/MacOS/"
cp README.md LICENSE "$app/Contents/Resources/"
if [[ ${INCLUDE_HOMEBREW_METADATA:-0} == 1 ]]; then
    dependencies=$(brew deps --installed ffmpeg)
    formulae=(ffmpeg)
    while IFS= read -r dependency; do
        [[ -z "$dependency" ]] || formulae+=("$dependency")
    done <<< "$dependencies"
    brew info --json=v2 --installed "${formulae[@]}" > "$app/Contents/Resources/homebrew-formulae.json"
fi
/usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $app_version" "$app/Contents/Info.plist"
/usr/libexec/PlistBuddy -c "Set :CFBundleVersion $app_version" "$app/Contents/Info.plist"
/usr/libexec/PlistBuddy -c "Set :LSMinimumSystemVersion $minimum_os" "$app/Contents/Info.plist"

ruby --disable-gems packaging/macos/bundle-libraries.rb "$app"

while IFS= read -r -d '' library; do
    codesign --force --sign - "$library"
done < <(find "$app/Contents/Frameworks" -type f -name '*.dylib' -print0)
codesign --force --sign - "$app/Contents/MacOS/eyepiece-pc"
codesign --force --sign - "$app"
codesign --verify --deep --strict "$app"

"$app/Contents/MacOS/eyepiece-pc" --version
"$app/Contents/MacOS/eyepiece-gui" --version
"$app/Contents/MacOS/eyepiece-pc" --source test --no-connect --encoder x264 --codec h264 --fps 30 --duration 1
ln -s /Applications "$staging/Applications"
hdiutil create -quiet -volname Eyepiece -srcfolder "$staging" -format UDZO "$dmg"
hdiutil verify -quiet "$dmg"
echo "Created $dmg (ad-hoc signed; not notarized)"

#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
app="$PWD/dist-fish/大肥鱼文档.app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"
cp target/release/quiet_md "$app/Contents/MacOS/dafeiyu-documents"
cp assets/branding/app-icon.png "$app/Contents/Resources/app-icon.png"
cp LICENSE "$app/Contents/Resources/LICENSE-Quiet-md"
cp assets/editor-icons/LICENSE-lucide "$app/Contents/Resources/LICENSE-Lucide"
icons=$(mktemp -d)/fish.iconset
mkdir -p "$icons"
for size in 16 32 128 256 512; do
 sips -z "$size" "$size" assets/branding/app-icon.png --out "$icons/icon_${size}x${size}.png" >/dev/null
 twice=$((size * 2))
 sips -z "$twice" "$twice" assets/branding/app-icon.png --out "$icons/icon_${size}x${size}@2x.png" >/dev/null
done
iconutil -c icns "$icons" -o "$app/Contents/Resources/app-icon.icns"
cat > "$app/Contents/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleName</key><string>大肥鱼文档</string>
<key>CFBundleDisplayName</key><string>大肥鱼文档</string>
<key>CFBundleIdentifier</key><string>com.followhongyun.dafeiyu.documents</string>
<key>CFBundleExecutable</key><string>dafeiyu-documents</string>
<key>CFBundleIconFile</key><string>app-icon.icns</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleShortVersionString</key><string>0.1.0</string>
<key>CFBundleVersion</key><string>20261110.1</string>
<key>NSHighResolutionCapable</key><true/>
<key>LSMinimumSystemVersion</key><string>13.0</string>
<key>NSDocumentsFolderUsageDescription</key><string>读取和编辑大肥鱼的文档。</string>
<key>CFBundleDocumentTypes</key><array><dict>
<key>CFBundleTypeName</key><string>Markdown</string>
<key>CFBundleTypeRole</key><string>Editor</string>
<key>LSHandlerRank</key><string>Alternate</string>
<key>LSItemContentTypes</key><array><string>net.daringfireball.markdown</string></array>
<key>CFBundleTypeExtensions</key><array><string>md</string><string>markdown</string></array>
</dict></array>
</dict></plist>
PLIST
codesign --force --options runtime --timestamp --sign 'Developer ID Application: Shaoqi Wang (QD3H9VHB23)' "$app"
codesign --verify --deep --strict "$app"

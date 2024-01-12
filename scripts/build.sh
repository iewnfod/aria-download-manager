#!/bin/bash

set -e


MACOS_BIN_NAME=aria-download-manager
MACOS_APP_NAME=Aria\ Download\ Manager
MACOS_APP_DIR=target/$MACOS_APP_NAME.app

echo "Creating app directory structure"
rm -rf "$MACOS_APP_DIR"
mkdir -p "$MACOS_APP_DIR/Contents/MacOS"

cargo build \
    --verbose \
    --release \

echo "Copying binary"
MACOS_APP_BIN=$MACOS_APP_DIR/Contents/MacOS/$MACOS_BIN_NAME
cp "target/release/$MACOS_BIN_NAME" "$MACOS_APP_BIN"

echo "Linking binary with frameworks"
for old in `otool -L "$MACOS_APP_BIN" | grep @rpath | cut -f2 | cut -d' ' -f1`; do
    new=`echo $old | sed -e "s/@rpath/@executable_path\/..\/Frameworks/"`
    echo "Replacing '$old' with '$new'"
    install_name_tool -change $old $new "$MACOS_APP_BIN"
done

# echo "Copying resources directory"
# cp -r $RESOURCES $MACOS_APP_DIR/Contents/MacOS
# echo "Copying user directory"
# cp -r $USER $MACOS_APP_DIR/Contents/MacOS

echo "Copying icon"
RESOURCE_DIR=$MACOS_APP_DIR/Contents/Resources
mkdir "$RESOURCE_DIR"
cp -r assets/icon.icns "$RESOURCE_DIR"

echo "Copying Info.plist"
cp scripts/info.plist "$MACOS_APP_DIR/Contents"

# echo "Signaturing app"
# codesign --force --deep --sign - "$MACOS_APP_DIR"

echo "Creating dmg"
mkdir "$MACOS_APP_NAME"
cp -r "$MACOS_APP_DIR" "$MACOS_APP_NAME"
ln -s /Applications "$MACOS_APP_NAME/Applications"
rm -rf "$MACOS_APP_NAME/.Trashes"

FULL_NAME=Aria\ Download\ Manager

hdiutil create "target/$FULL_NAME.dmg" -srcfolder "$MACOS_APP_NAME" -ov
rm -rf "$MACOS_APP_NAME"

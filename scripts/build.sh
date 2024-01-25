#!/bin/bash

set -e

MACOS_BIN_NAME=aria-download-manager
MACOS_TRAY_NAME=adm-tray
MACOS_APP_NAME=Aria\ Download\ Manager
MACOS_APP_DIR=target/$MACOS_APP_NAME.app
RESOURCE_NAME=assets
RESOURCES=(assets/icon.icns assets/icon.iconset)

echo "Updating submodule"
/bin/bash scripts/update_submodule.sh

echo "Creating app directory structure"
rm -rf "$MACOS_APP_DIR"
mkdir -p "$MACOS_APP_DIR/Contents/MacOS"

echo "Building and copying main binary"
cargo build --release
MACOS_APP_BIN=$MACOS_APP_DIR/Contents/MacOS/$MACOS_BIN_NAME
cp "target/release/$MACOS_BIN_NAME" "$MACOS_APP_BIN"

echo "Building and copying tray binary"
/bin/bash scripts/build-tray.sh
MACOS_APP_TRAY_BIN=$MACOS_APP_DIR/Contents/MacOS/$MACOS_TRAY_NAME
cp "$MACOS_TRAY_NAME/target/release/$MACOS_TRAY_NAME" "$MACOS_APP_TRAY_BIN"

echo "Linking binary with frameworks"
for old in `otool -L "$MACOS_APP_BIN" | grep @rpath | cut -f2 | cut -d' ' -f1`; do
    new=`echo $old | sed -e "s/@rpath/@executable_path\/..\/Frameworks/"`
    echo "Replacing '$old' with '$new'"
    install_name_tool -change $old $new "$MACOS_APP_BIN"
done

echo "Copying resources directory"
MACOS_APP_RESOURCES_DIR=$MACOS_APP_DIR/Contents/MacOS/$RESOURCE_NAME
mkdir -p "$MACOS_APP_RESOURCES_DIR"
for resource in ${RESOURCES[@]}; do
    if [ -d "$resource" ]; then
        cp -r "$resource" "$MACOS_APP_RESOURCES_DIR"
    elif [ -f "$resource" ]; then
        cp "$resource" "$MACOS_APP_RESOURCES_DIR"
    fi
done

echo "Copying icon"
RESOURCE_DIR=$MACOS_APP_DIR/Contents/Resources
mkdir "$RESOURCE_DIR"
cp -r assets/icon.icns "$RESOURCE_DIR"

echo "Copying Info.plist"
cp scripts/info.plist "$MACOS_APP_DIR/Contents"

# echo "Signaturing app"
# codesign --force --deep --sign - "$MACOS_APP_DIR"

echo "Creating dmg"
if [ -e "$MACOS_APP_NAME" ]; then
    rm -rf "$MACOS_APP_NAME"
fi
mkdir "$MACOS_APP_NAME"
cp -r "$MACOS_APP_DIR" "$MACOS_APP_NAME"
ln -s /Applications "$MACOS_APP_NAME/Applications"
rm -rf "$MACOS_APP_NAME/.Trashes"

FULL_NAME=Aria\ Download\ Manager

hdiutil create "target/$FULL_NAME.dmg" -srcfolder "$MACOS_APP_NAME" -ov
rm -rf "$MACOS_APP_NAME"

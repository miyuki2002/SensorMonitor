#!/bin/bash
set -e

# Configuration
APP_NAME="sensor-monitor"
PACKAGE_NAME="com.example.sensormonitor"
MIN_SDK_VERSION=24
TARGET_SDK_VERSION=34
BUILD_TOOLS_VERSION="34.0.0"

# Build Rust libraries for all target architectures
echo "Building Rust libraries..."
cargo ndk -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 build --release

# Create directory structure
echo "Creating Android package structure..."
mkdir -p build/apk/lib/arm64-v8a
mkdir -p build/apk/lib/armeabi-v7a
mkdir -p build/apk/lib/x86
mkdir -p build/apk/lib/x86_64
mkdir -p build/apk/assets
mkdir -p build/apk/res/mipmap-hdpi
mkdir -p build/apk/res/mipmap-mdpi
mkdir -p build/apk/res/mipmap-xhdpi
mkdir -p build/apk/res/mipmap-xxhdpi

# Copy the shared libraries to the APK structure
echo "Copying native libraries..."
cp target/armv7-linux-androideabi/release/libsensor_monitor.so build/apk/lib/armeabi-v7a/
cp target/aarch64-linux-android/release/libsensor_monitor.so build/apk/lib/arm64-v8a/
cp target/i686-linux-android/release/libsensor_monitor.so build/apk/lib/x86/
cp target/x86_64-linux-android/release/libsensor_monitor.so build/apk/lib/x86_64/

# Copy Android manifest
echo "Copying manifest and resources..."
cp android/AndroidManifest.xml build/apk/

# Build the APK
echo "Building APK..."
ANDROID_HOME=${ANDROID_HOME:-"$HOME/Android/Sdk"}
AAPT="$ANDROID_HOME/build-tools/$BUILD_TOOLS_VERSION/aapt"
DX="$ANDROID_HOME/build-tools/$BUILD_TOOLS_VERSION/dx"
ZIPALIGN="$ANDROID_HOME/build-tools/$BUILD_TOOLS_VERSION/zipalign"
APKSIGNER="$ANDROID_HOME/build-tools/$BUILD_TOOLS_VERSION/apksigner"
PLATFORM="$ANDROID_HOME/platforms/android-$TARGET_SDK_VERSION/android.jar"

# Package the APK
cd build/apk
$AAPT package -f -M AndroidManifest.xml -I "$PLATFORM" -F "$APP_NAME-unaligned.apk" .

# Sign the APK (create a debug keystore if it doesn't exist)
if [ ! -f ~/.android/debug.keystore ]; then
    mkdir -p ~/.android
    keytool -genkey -v -keystore ~/.android/debug.keystore -storepass android -alias androiddebugkey -keypass android -keyalg RSA -keysize 2048 -validity 10000 -dname "CN=Android Debug,O=Android,C=US"
fi

echo "Signing APK..."
$APKSIGNER sign --ks ~/.android/debug.keystore --ks-pass pass:android --out "$APP_NAME.apk" "$APP_NAME-unaligned.apk"

echo "Done. APK is at build/apk/$APP_NAME.apk" 
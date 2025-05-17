# Configuration
$AppName = "sensor-monitor"
$PackageName = "com.example.sensormonitor"
$MinSdkVersion = 24
$TargetSdkVersion = 34
$BuildToolsVersion = "34.0.0"

# Set Android SDK path (modify as needed)
if (-not $env:ANDROID_HOME) {
    $env:ANDROID_HOME = "$env:USERPROFILE\AppData\Local\Android\Sdk"
}

# Build Rust libraries for all target architectures
Write-Host "Building Rust libraries..."
cargo ndk -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 build --release

# Create directory structure
Write-Host "Creating Android package structure..."
New-Item -ItemType Directory -Force -Path build\apk\lib\arm64-v8a | Out-Null
New-Item -ItemType Directory -Force -Path build\apk\lib\armeabi-v7a | Out-Null
New-Item -ItemType Directory -Force -Path build\apk\lib\x86 | Out-Null
New-Item -ItemType Directory -Force -Path build\apk\lib\x86_64 | Out-Null
New-Item -ItemType Directory -Force -Path build\apk\assets | Out-Null
New-Item -ItemType Directory -Force -Path build\apk\res\mipmap-hdpi | Out-Null
New-Item -ItemType Directory -Force -Path build\apk\res\mipmap-mdpi | Out-Null
New-Item -ItemType Directory -Force -Path build\apk\res\mipmap-xhdpi | Out-Null
New-Item -ItemType Directory -Force -Path build\apk\res\mipmap-xxhdpi | Out-Null

# Copy the shared libraries to the APK structure
Write-Host "Copying native libraries..."
Copy-Item -Path target\armv7-linux-androideabi\release\libsensor_monitor.so -Destination build\apk\lib\armeabi-v7a\
Copy-Item -Path target\aarch64-linux-android\release\libsensor_monitor.so -Destination build\apk\lib\arm64-v8a\
Copy-Item -Path target\i686-linux-android\release\libsensor_monitor.so -Destination build\apk\lib\x86\
Copy-Item -Path target\x86_64-linux-android\release\libsensor_monitor.so -Destination build\apk\lib\x86_64\

# Copy Android manifest
Write-Host "Copying manifest and resources..."
Copy-Item -Path android\AndroidManifest.xml -Destination build\apk\

# Set paths to Android tools
$Aapt = Join-Path $env:ANDROID_HOME "build-tools\$BuildToolsVersion\aapt.exe"
$ZipAlign = Join-Path $env:ANDROID_HOME "build-tools\$BuildToolsVersion\zipalign.exe"
$ApkSigner = Join-Path $env:ANDROID_HOME "build-tools\$BuildToolsVersion\apksigner.bat"
$Platform = Join-Path $env:ANDROID_HOME "platforms\android-$TargetSdkVersion\android.jar"

# Package the APK
Write-Host "Building APK..."
Push-Location build\apk
& $Aapt package -f -M AndroidManifest.xml -I $Platform -F "$AppName-unaligned.apk" .

# Sign the APK (create a debug keystore if it doesn't exist)
$DebugKeystore = "$env:USERPROFILE\.android\debug.keystore"
if (-not (Test-Path $DebugKeystore)) {
    Write-Host "Creating debug keystore..."
    New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.android" | Out-Null
    & keytool -genkey -v -keystore $DebugKeystore -storepass android -alias androiddebugkey -keypass android -keyalg RSA -keysize 2048 -validity 10000 -dname "CN=Android Debug,O=Android,C=US"
}

Write-Host "Signing APK..."
& $ApkSigner sign --ks $DebugKeystore --ks-pass pass:android --out "$AppName.apk" "$AppName-unaligned.apk"

Pop-Location
Write-Host "Done. APK is at build\apk\$AppName.apk" 
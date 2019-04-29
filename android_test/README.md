# ga.dive.example

```
# TODO: Copy resources into res.
mkdir res/
# Generate R.java
$HOME/.cargo-dist/android_sdk/build-tools/26.0.1/aapt package -f -m -J . -M AndroidManifest.xml -S res -I $HOME/.cargo-dist/android_sdk/platforms/android-18/android.jar
# Build APK
$HOME/.cargo-dist/android_sdk/build-tools/26.0.1/aapt package -f -M AndroidManifest.xml -S res -I $HOME/.cargo-dist/android_sdk/platforms/android-18/android.jar -F example.apk.unaligned
# Add libraries to APK
$HOME/.cargo-dist/android_sdk/build-tools/26.0.1/aapt add example.apk.unaligned lib/arm64-v8a/libnative-activity.so
$HOME/.cargo-dist/android_sdk/build-tools/26.0.1/aapt add example.apk.unaligned lib/armeabi-v7a/libnative-activity.so
$HOME/.cargo-dist/android_sdk/build-tools/26.0.1/aapt add example.apk.unaligned lib/x86/libnative-activity.so
$HOME/.cargo-dist/android_sdk/build-tools/26.0.1/aapt add example.apk.unaligned lib/x86_64/libnative-activity.so
# Sign APK
jarsigner -keystore ~/.android/debug.keystore -storepass 'android' example.apk.unaligned androiddebugkey
# ZipAlign APK
$HOME/.cargo-dist/android_sdk/build-tools/26.0.1/zipalign -f 4 example.apk.unaligned example.apk
# Install APK
$HOME/.cargo-dist/android_sdk/platform-tools/adb install -r example.apk
# Launch App
$HOME/.cargo-dist/android_sdk/platform-tools/adb shell am start -n ga.dive.example/android.app.NativeActivity
```

# ga.dive.example

```
# TODO: Copy resources into res.
mkdir res/
# Generate R.java
$HOME/.cargo-cala/android-sdk/build-tools/26.0.1/aapt package -f -m -J . -M AndroidManifest.xml -S res -I $HOME/.cargo-cala/android-sdk/platforms/android-18/android.jar
# Build APK
$HOME/.cargo-cala/android-sdk/build-tools/26.0.1/aapt package -f -M AndroidManifest.xml -S res -I $HOME/.cargo-cala/android-sdk/platforms/android-18/android.jar -F example.apk.unaligned
# Add libraries to APK
$HOME/.cargo-cala/android-sdk/build-tools/26.0.1/aapt add example.apk.unaligned lib/arm64-v8a/librs.so
$HOME/.cargo-cala/android-sdk/build-tools/26.0.1/aapt add example.apk.unaligned lib/armeabi-v7a/librs.so
$HOME/.cargo-cala/android-sdk/build-tools/26.0.1/aapt add example.apk.unaligned lib/x86/librs.so
$HOME/.cargo-cala/android-sdk/build-tools/26.0.1/aapt add example.apk.unaligned lib/x86_64/librs.so
# Sign APK
jarsigner -keystore ~/.android/debug.keystore -storepass 'android' example.apk.unaligned androiddebugkey
# ZipAlign APK
$HOME/.cargo-cala/android-sdk/build-tools/26.0.1/zipalign -f 4 example.apk.unaligned example.apk
# Install APK
$HOME/.cargo-cala/android-sdk/platform-tools/adb install -r example.apk
# Launch App
$HOME/.cargo-cala/android-sdk/platform-tools/adb shell am start -n cala.example/android.app.NativeActivity
```

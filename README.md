<img src="http://at.plopgrizzly.tech/gsp/banner.png" alt="Graphical Software Packager (cargo-gsp)" width="100%">

Cargo plugin for testing & building release packages that depend on cala and/or wasm.

## Linux: FlatPak

You'll need a terminal.

### Install: Fedora

	sudo dnf install flatpak

### Install: Ubuntu

	sudo add-apt-repository ppa:alexlarsson/flatpak
	sudo apt update
	sudo apt install flatpak

### Setup

	flatpak remote-add --from gnome https://sdk.gnome.org/gnome.flatpakrepo
	flatpak install gnome org.gnome.Platform//3.24

## Android: APK

### Use Cargo-Dist to Build APKs
Here is the script that is used, if you want to set it up manually:

```sh
rustup target add arm-linux-androideabi aarch64-linux-android armv7-linux-androideabi i686-linux-android thumbv7neon-linux-androideabi x86_64-linux-android
mkdir ~/.cargo-dist/
cd ~/.cargo-dist/
wget https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip
unzip sdk-tools-linux-4333796.zip
wget https://dl.google.com/android/repository/android-ndk-r19c-linux-x86_64.zip
unzip android-ndk-r19c-linux-x86_64.zip
rm android-ndk-r19c-linux-x86_64.zip
rm sdk-tools-linux-4333796.zip
mv android-ndk-r19c/ android_ndk/
./android_ndk/build/tools/make-standalone-toolchain.sh --toolchain=arm-linux-androideabi --install-dir=arm-linux-androideabi
./android_ndk/build/tools/make-standalone-toolchain.sh --toolchain=aarch64-linux-android --install-dir=aarch64-linux-android
./android_ndk/build/tools/make-standalone-toolchain.sh --toolchain=x86-linux-android --install-dir=x86-linux-android
./android_ndk/build/tools/make-standalone-toolchain.sh --toolchain=x86_64-linux-android --install-dir=x86_64-linux-android
mkdir android_sdk/
cd android_sdk/
mv ../tools .
./tools/bin/sdkmanager "platform-tools" "platforms;android-18" "build-tools;26.0.1"
```

Environment variables:

```sh
NDK_HOME=$HOME/.cargo-dive/android_ndk ANDROID_HOME=$HOME/.cargo-dive/android_sdk
```

## Windows: Installer.exe

TODO

## MacOS: .app

TODO

<img src="http://at.plopgrizzly.tech/gsp/banner.png" alt="Graphical Software Packager (cargo-gsp)" width="100%">

A cargo plugin that makes cargo generate release packages instead of binaries.

## Links
* [Website](http://at.plopgrizzly.tech/gsp/)
* [Cargo](https://crates.io/crates/cargo-gsp/)
* [Documentation](http://at.plopgrizzly.tech/docs/gsp/)
* [Tutorial](http://at.plopgrizzly.tech/demos/gsp)

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
mkdir ~/.cargo-dist/
cd ~/.cargo-dist/
rustup target add arm-linux-androideabi
wget https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip
unzip sdk-tools-linux-4333796.zip
wget https://dl.google.com/android/repository/android-ndk-r19c-linux-x86_64.zip
unzip android-ndk-r19c-linux-x86_64.zip
rm android-ndk-r19c-linux-x86_64.zip
rm sdk-tools-linux-4333796.zip
mkdir android_sdk/
cd android_sdk/
mv ../tools .
mv android-ndk-r19c/ android_ndk/
./tools/bin/sdkmanager "platform-tools" "platforms;android-18" "build-tools;26.0.1"
```

Environment variables:

```sh
NDK_HOME=$HOME/.cargo-dist/android_ndk ANDROID_HOME=$HOME/.cargo-dist/android_sdk
```

## Windows: Installer.exe

TODO

## MacOS: .app

TODO

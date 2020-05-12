# Notes
This file explains the process of making release packages for each platform.

## Linux
### Prerequistes
- Flatpak

```
# At the time of writing org.gnome.Platform/x86_64/3.36 is the newest but has
# bugs in Wayland, so we'll use the one before the bug was introduced.
sudo flatpak install -y org.gnome.Platform/x86_64/3.32
```

### Variables
```bash,no_run
{APP_COMPANY}       # Company developing application
{APP_NAME}          # Name of the application
{APP_DESCRIPTION}   # A short description (one sentence) describing the app
{APP_NAME_EN}       # English name for the app
```

```bash,no_run
{APP_DOMAIN} = cala.{APP-COMPANY}.{APP-NAME}
```

### Create the Repository
Enter these bash commands:

```bash
mkdir -p target/cargo-cala/flatpak/app/files/bin/
mkdir -p target/cargo-cala/flatpak/app/export/share/icons/hicolor/scalable/apps/
mkdir -p target/cargo-cala/flatpak/app/export/share/applications/
mkdir -p target/cargo-cala/flatpak/repo/
cp target/release/{APP_NAME} target/cargo-cala/flatpak/app/files/bin/{APP_DOMAIN}
cp res/logo.svg target/cargo-cala/flatpak/app/export/share/icons/hicolor/scalable/apps/
vim target/cargo-cala/flatpak/app/metadata
```

Then enter:

```
[Application]
name={APP_DOMAIN}
runtime=org.gnome.Platform/x86_64/3.32
command={APP_DOMAIN}

[Context]
shared=ipc;network;
sockets=x11;wayland;pulseaudio;
devices=dri;
filesystems=host;
```

Then in bash again, `vim target/cargo-cala/flatpak/app/export/share/applications/{APP_DOMAIN}.desktop`

```
[Desktop Entry]
Type=Application
Encoding=UTF-8
Name={APP_NAME}
Exec={APP_DOMAIN}
Icon={APP_DOMAIN}
Terminal=false

Name[en]={APP_NAME_EN}
Comment={APP_DESCRIPTION}
Comment[en]={APP_DESCRIPTION}
```

Then back in bash,
```bash
# This generates content in repo from app.
flatpak build-export target/cargo-cala/flatpak/repo/ target/cargo-cala/flatpak/app/
```

### Installing Locally
```
flatpak --user remote-add --no-gpg-verify --if-not-exists {APP_DOMAIN} target/cargo-cala/flatpak/repo/
flatpak --user install --reinstall --noninteractive {APP_DOMAIN} {APP_DOMAIN} -y
```

### Running Locally
```
flatpak run {APP_DOMAIN}
```

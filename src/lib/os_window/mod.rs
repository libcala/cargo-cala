// lib/os_window/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub type OSWindow = self::windows::WindowsWindow;

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
pub type OSWindow = self::android::AndroidWindow;

#[cfg(target_os = "ios")]
mod ios;
#[cfg(target_os = "ios")]
pub type OSWindow = self::ios::IosWindow;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd",
	target_os = "dragonfly", target_os = "bitrig", target_os = "openbsd",
	target_os = "netbsd"))]
mod unix;
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd",
	target_os = "dragonfly", target_os = "bitrig", target_os = "openbsd",
	target_os = "netbsd"))]
pub use self::unix::{ UnixWindow as OSWindow, key };

// Platforms that don't have standard libary support.

#[cfg(target_os = "aldarons_os")]
mod aldarons_os;
#[cfg(target_os = "aldarons_os")]
pub type OSWindow = self::aldarons_os::AldaronsWindow;

#[cfg(target_os = "arduino")]
mod arduino;
#[cfg(target_os = "arduino")]
pub type OSWindow = self::arduino::ArduinoWindow;

#[cfg(target_os = "nintendo_switch")]
mod nintendo_switch;
#[cfg(target_os = "nintendo_switch")]
pub type OSWindow = self::nintendo_switch::SwitchWindow;

#[cfg(target_os = "web_assembly")]
mod web_assembly;
#[cfg(target_os = "web_assembly")]
pub type OSWindow = self::web_assembly::WebWindow;

#[cfg(target_os = "none")]
mod no_os;
#[cfg(target_os = "none")]
pub type OSWindow = self::no_os::OSWindow;

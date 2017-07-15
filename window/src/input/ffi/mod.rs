// lib/input/ffi/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "android"))]
mod unix;
#[cfg(any(target_os = "macos", target_os = "linux", target_os = "android"))]
pub use self::unix::Joystick;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::Joystick;

#[cfg(not(any(target_os = "macos",target_os = "linux",target_os = "windows",target_os = "android")))]
mod emulated;
#[cfg(not(any(target_os = "macos",target_os = "linux",target_os = "windows",target_os = "android")))]
pub use self::emulated::Joystick;

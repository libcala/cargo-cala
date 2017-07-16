// lib/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

#![doc(html_logo_url = "http://at.plopgrizzly.tech/window/icon.png",
       html_favicon_url = "http://at.plopgrizzly.tech/window/icon.png",
       html_root_url = "http://at.plopgrizzly.tech/window/")]

pub(crate) extern crate ami;

pub(crate) mod input;
pub(crate) mod os_window;
pub(crate) mod window_connection;
pub(crate) mod window;
pub(crate) mod window_ops;

pub use input::Input;
pub use input::Key;
pub use input::Click;
pub use input::Joystick;
pub use input::Button;
pub use input::InputQueue;
pub use input::{ Msg, Emphasis, Align };
pub use window_connection::WindowConnection;
pub use window::Window;
pub use window_ops::WindowOps;

// Default Width and Height for a window.
pub(crate) const MWW : u32 = 640;
pub(crate) const MWH : u32 = 360;

// Main
/*#[cfg(target_os = "android")]
#[allow(unused)]
#[no_mangle]
pub extern "C" fn gsp_main(activity: *mut ANativeActivity) -> () {
	println!("Got Start");
}*/

/*#[cfg(not(target_os = "android"))]
#[no_mangle]
pub extern "C" fn gsp_main() -> () {
	println!("Got Start");
}*/

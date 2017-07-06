// lib/os_window/unix/wayland/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

#[link(name = "wayland-client")]
pub mod window {
	use std::ptr::null as null;

	pub enum WaylandDisplay { }
	pub enum WaylandEventQueue { }

	extern {
		pub fn wl_display_connect(name: *const i8) -> *mut WaylandDisplay;
		pub fn wl_display_disconnect(display: *mut WaylandDisplay) -> ();
		pub fn wl_display_get_error(display: *mut WaylandDisplay) -> i32;
	}

	pub fn init() {
		// Call wayland libary's init.
		let wayland_display:*mut _;
		unsafe {
			wayland_display = wl_display_connect(null());
		};
		if wayland_display as *const _ == null() {
			panic!("Couldn't Connect: Wayland server not running?");
		}
	}

	pub fn kill() {
		
	}
}

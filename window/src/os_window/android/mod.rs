// lib/os_window/android/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

mod input;
mod glue;

pub use self::input::key;
pub use self::glue::gsp_main;

pub struct AndroidWindow {
}

impl ::WindowOps for AndroidWindow {
	fn create(title: &str, icon: (u32, u32, &[u8])) -> Self {
		AndroidWindow { }
	}

	fn show(&self) -> () {

	}

	fn update(&self) -> () {

	}

	fn poll_event(&self, input: &mut ::input::InputQueue, wh: &mut(u32,u32))
		-> bool
	{
		false
	}

	fn fullscreen(&self) -> () {

	}

	fn get_connection(&self) -> ::WindowConnection {
		::WindowConnection::Android
	}
}

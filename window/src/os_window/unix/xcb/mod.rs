// lib/os_window/unix/xcb/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

mod ffi;
mod event;
mod keyboard;
mod native_connection;
mod property;

use self::native_connection::NativeConnection;
use self::property::Property;
use self::event::Event;

pub struct XcbWindow {
	native: NativeConnection,
	fullscreen: Property,
}

impl ::WindowOps for XcbWindow {
	fn create(title: &str, _: (u32, u32, &[u8])) -> Self {
		let xcb_dl = unsafe { ffi::load_dl() };
		let native = NativeConnection::create(xcb_dl);

		if native.failed() {
			return XcbWindow {
				fullscreen: Property::dummy(), native: native
			}
		}

		let native = native.title(title);

		XcbWindow {
			fullscreen: Property::create(native.connection(),
				b"_NET_WM_STATE", b"_NET_WM_STATE_FULLSCREEN"),
			native: native,
		}
	}

	fn show(&self) -> () {
		// Make sure 'X' button works before showing!
		Property::create(self.native.connection(), b"WM_PROTOCOLS",
				b"WM_DELETE_WINDOW")
			.catch(self.native.connection(), self.native.1);
		self.native.show()
	}

	fn update(&self) -> () {
		self.native.update()
	}

	fn poll_event(&self, input: &mut ::input::InputQueue, wh: &mut(u32,u32))
		-> bool
	{
		let connection = self.native.connection();
		let keyboard_state = self.native.keyboard_state();

		Event::create(connection, keyboard_state).poll(input, wh)
	}

	fn fullscreen(&self) -> () {
		self.fullscreen.apply(self.native.connection(), self.native.1)
	}

	fn get_connection(&self) -> ::WindowConnection {
		::WindowConnection::Xcb(self.native.0 .0, self.native.1)
	}
}

impl XcbWindow {
	pub fn failed(&self) -> bool {
		self.native.failed()
	}
}

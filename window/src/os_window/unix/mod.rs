// lib/os_window/unix/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

mod input;

mod xcb;
// mod wayland; // TODO: link to in runtime only if it's installed.
// mod direct_fb;

pub use self::input::key;

pub enum UnixWindow {
//	Wayland(),
	Xcb(xcb::XcbWindow),
//	DirectFb(direct_fb::FBWindow),
}

impl ::WindowOps for UnixWindow {
	fn create(title: &str, icon: (u32, u32, &[u8])) -> Self {
		let xcb = xcb::XcbWindow::create(title, icon);

		if xcb.failed() {
			println!("WARNING: Either XCB is uninstalled or no X.");
		} else {
			return UnixWindow::Xcb(xcb);
		}

		println!("WARNING: No wayland support yet.");
		println!("WARNING: No directfb support yet.");
		panic!("None of the unix backends [xcb,] found!");
	}

	fn show(&self) -> () {
		match *self {
//			UnixWindow::Wayland(w) => w.show(),
			UnixWindow::Xcb(ref w) => w.show(),
//			UnixWindow::DirectFb(ref w) => w.show(),
		}
	}

	fn update(&self) -> () {
		match *self {
//			UnixWindow::Wayland(w) => w.update(),
			UnixWindow::Xcb(ref w) => w.update(),
//			UnixWindow::DirectFb(ref w) => w.update(),
		}
	}

	fn poll_event(&self, input: &mut ::input::InputQueue, wh: &mut(u32,u32))
		-> bool
	{
		let r = match *self {
//			UnixWindow::Wayland(w) => w.poll_event(input, wh),
			UnixWindow::Xcb(ref w) => w.poll_event(input, wh),
//			UnixWindow::DirectFb(ref w) => w.poll_event(input, wh),
		};

		r
	}

	fn fullscreen(&self) -> () {
		match *self {
//			UnixWindow::Wayland(w) => w.fullscreen(),
			UnixWindow::Xcb(ref w) => w.fullscreen(),
//			UnixWindow::DirectFb(ref w) => w.fullscreen(),
		}
	}

	fn get_connection(&self) -> ::WindowConnection {
		match *self {
//			UnixWindow::Wayland(w) => w.get_connection(),
			UnixWindow::Xcb(ref w) => w.get_connection(),
//			UnixWindow::DirectFb(ref w) => w.get_connection(),
		}
	}
}

// lib/window_connection/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use ami::void_pointer::VoidPointer;

/// Connection is listed first, then window.
pub enum WindowConnection {
	Xcb(VoidPointer, u32),
	Wayland,
	DirectFB,
	Windows,
	Android,
	IOS,
	AldaronsOS,
	Arduino,
	Switch,
	Web,
	NoOS,
}

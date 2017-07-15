// lib/window_ops/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

pub trait WindowOps {
	// Create the window.
	fn create(title: &str, icon: (u32, u32, &[u8])) -> Self;
	// Show the window.
	fn show(&self) -> ();
	// Re-draw the window.
	fn update(&self) -> ();
	// Poll for events, returns true if there's more.  Adds 1+ to input.
	fn poll_event(&self, input: &mut ::input::InputQueue, wh: &mut(u32,u32))
		-> bool;
	// Toggle fullscreen.
	fn fullscreen(&self) -> ();
	// Get connection details
	fn get_connection(&self) -> ::WindowConnection;
}

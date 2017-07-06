// lib/input/mod.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

pub(crate) mod cursor;
pub(crate) mod keyboard;
mod joystick;
mod ffi;

pub use input::keyboard::key::Key;
pub use input::keyboard::msg::Msg;
pub use input::keyboard::msg::Align;
pub use input::keyboard::msg::Emphasis;
pub use input::cursor::Click;
pub use input::joystick::Joystick;
pub use input::joystick::Button;

/// Input to the window, that's put into the input queue, when an event has
/// occurred.
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Input {
	/// One of the following has happenned,
	///
	/// - The window has just been resized
	/// - The window has just been created
	Resize,
	/// The user has switched to this window (in focus).
	Resume,
	/// The user has switched to a different window (out of focus).
	Pause,
	/// One of the following has happenned,
	///
	/// - A key has been pressed on a physical keyboard.
	/// - A key has been pressed on an on-screen keyboard.
	KeyPress(Key),
	/// One of the following has happenned,
	///
	/// - A key has been released on a physical keyboard.
	/// - A key has been released on an on-screen keyboard.
	KeyRelease(Key),
	/// The user has inputted text.
	Text(char),
	/// One of the following has happenned,
	///
	/// - A keyboard shortcut has been used.
	/// - A graphical shortcut has been used.
	Msg(Msg),
	/// `Cursor(x, y)`: One of the following has happenned,
	///
	/// - The user moves the cursor with the mouse.
	/// - The user moves the cursor with the touchpad.
	/// - The last place that the user touched the touchscreen changes.
	Cursor(Option<(f32,f32)>),
	/// `CursorPress(click, x, y)`: A mouse button has been pressed.
	CursorPress(Click, (f32,f32)),
	/// `CursorRelease(click, x, y)`: A mouse button has been released.
	CursorRelease(Click, Option<(f32,f32)>),
	/// One of the following has happenned,
	///
	/// - The mouse wheel has been scrolled up.
	/// - The touchpad has been used to scroll up.
	ScrollUp(f32,f32),
	/// One of the following has happenned,
	///
	/// - The mouse wheel has been scrolled down.
	/// - The touchpad has been used to scroll down.
	ScrollDown(f32,f32),
	/// One of the following has happenned,
	///
	/// - The mouse wheel has been scrolled left.
	/// - The touchpad has been used to scroll left.
	ScrollLeft(f32,f32),
	/// One of the following has happenned,
	///
	/// - The mouse wheel has been scrolled right.
	/// - The touchpad has been used to scroll right.
	ScrollRight(f32,f32),
	/// One of the following has happenned,
	///
	/// - The joystick has moved to a different position.
	/// - The C-pad has moved.
	/// - The on-screen joystick 1 has moved.
	JoystickMove(f32, f32),
	/// One of the following has happenned,
	///
	/// - The joystick's POV hat has moved.
	/// - The POV-Joystick has moved.
	/// - The on-screen joystick 2 has moved.
	JoystickPov(f32, f32),
	/// One of the following has happenned,
	///
	/// - The joystick's throttle has moved.
	/// - The on-screen throttle has moved.
	JoystickThrottle(f32),
	/// One of the following has happenned,
	///
	/// - One of the joystick's buttons has been pressed.
	/// - An on-screen button has been pressed.
	JoystickButtonDown(Button),
	/// One of the following has happenned,
	///
	/// - One of the joystick's buttons has been released.
	/// - An on-screen button has been released.
	JoystickButtonUp(Button),
}

pub enum ScrollWheel {
	Up,
	Down,
	Left,
	Right,
}

trait CoordToFloat {
	fn to_f32(self) -> f32;
}

impl CoordToFloat for u32 {
	fn to_f32(self) -> f32 { self as f32 }
}

impl CoordToFloat for i16 {
	fn to_f32(self) -> f32 { self as f32 }
}

fn cursor_coordinates<T, U>(wh: (T, T), xy: (U, U)) -> Option<(f32, f32)>
	where U: CoordToFloat, T: CoordToFloat
{
	let x = xy.0.to_f32();
	let y = xy.1.to_f32();
	let w = wh.0.to_f32();
	let h = wh.1.to_f32();
	let xy = (x * 2.0 / w - 1.0, y * 2.0 / h - 1.0);

	if xy.0 > 1.0 || xy.0 < -1.0 || xy.1 > 1.0 || xy.1 < -1.0 {
		None
	} else {
		Some(xy)
	}
}

pub struct InputQueue {
	queue: Vec<Input>,
	mods: keyboard::modifiers::Modifiers,
	resized: bool,
	fullscreen: bool,
}

impl InputQueue {
	#[inline(always)]
	pub fn create() -> InputQueue {
		let queue = Vec::new();
		let mods = keyboard::modifiers::Modifiers::create();
		let resized = false;
		let fullscreen = false;

		InputQueue { queue, mods, resized, fullscreen }
	}

	/// Returns an iterator over the InputQueue.
	#[inline(always)]
	pub fn iter(&self) -> ::std::slice::Iter<Input> {
		self.queue.iter()
	}

	/// Returns true if the InputQueue indicates that the window has been
	/// resized, returns false otherwise.
	#[inline(always)]
	pub fn get_resized(&self) -> bool {
		self.resized
	}

	#[inline(always)]
	pub fn get_fullscreen(&self) -> bool {
		self.fullscreen
	}

	#[inline(always)]
	pub fn clear(&mut self) {
		self.resized = false;
		self.fullscreen = false;
		self.queue.clear()
	}

	#[inline(always)]
	pub fn len(&self) -> usize {
		self.queue.len()
	}

	#[inline(always)]
	pub fn pop(&mut self) {
		self.queue.pop();
	}

	#[inline(always)]
	pub fn last(&self) -> Input {
		self.queue[self.queue.len() - 1]
	}

	#[inline(always)]
	pub fn resize(&mut self, wh: &mut (u32,u32), d: (u32,u32)) {
		// Only if new dimensions differ from old.
		if *wh != d {
			*wh = d;
			self.input(Input::Resize);
			self.resized = true;
		}
	}

	#[inline(always)]
	pub fn fullscreen(&mut self) {
		self.fullscreen = true
	}

	#[inline(always)]
	pub fn key_down(&mut self, key: Key) {
		match key {
			_ => self.input(Input::KeyPress(key)),
		}
	}

	#[inline(always)]
	pub fn key_up(&mut self, key: Key) {
		self.input(Input::KeyRelease(key));
	}

	#[inline(always)]
	pub fn scroll(&mut self, wh: (u32, u32), c: (i16, i16),
		direction: ScrollWheel, times: usize)
	{
		let xy = cursor_coordinates(wh, c);

		if let Some((x, y)) = xy {
			match direction {
				ScrollWheel::Up => self.push(Input::ScrollUp(x,y),times),
				ScrollWheel::Down => self.push(Input::ScrollDown(x,y),times),
				ScrollWheel::Left => self.push(Input::ScrollLeft(x,y),times),
				ScrollWheel::Right => self.push(Input::ScrollRight(x,y),times),
			}
		}
	}

	#[inline(always)]
	pub fn button_down(&mut self, wh: (u32, u32), c: (i16, i16),
		button_id: Click)
	{
		let xy = cursor_coordinates(wh, c);

		if let Some(xy) = xy {
			self.input(Input::CursorPress(button_id, xy));
		}
	}

	#[inline(always)]
	pub fn button_up(&mut self, wh: (u32, u32), c: (i16, i16),
		button_id: Click)
	{
		let xy = cursor_coordinates(wh, c);

		self.input(Input::CursorRelease(button_id, xy));
	}

	#[inline(always)]
	pub fn cursor_move(&mut self, wh: (u32,u32), c: (i16,i16)) {
		let xy = cursor_coordinates(wh, c);

		self.input(Input::Cursor(xy));
	}

	#[inline(always)]
	pub fn cursor_leave(&mut self) {
		self.input(Input::Cursor(None));
	}

	#[inline(always)]
	pub fn pause(&mut self) {
		self.input(Input::Pause);
	}

	#[inline(always)]
	pub fn resume(&mut self) {
		self.input(Input::Resume);
	}

	#[inline(always)]
	pub fn back(&mut self) {
		self.input(Input::Msg(Msg::Back));
	}

	#[inline(always)]
	pub fn text(&mut self, string: String) {
		let chars = string.char_indices();

		for c in chars {
			self.input(Input::Text(c.1));
		}
	}	

	#[inline(always)]
	fn push(&mut self, input: Input, repeat: usize) {
		for _ in 0..repeat {
			self.input(input);
		}
	}

	#[inline(always)]
	fn input(&mut self, input: Input) -> () {
		self.mods.update(&mut self.queue, input)
	}
}

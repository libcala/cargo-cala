// lib/os_window/unix/xcb/poll_event.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use ami::void_pointer::*;

use Input;
use Key;
use input;
use super::ffi as xcb;

const KEY_DOWN: u8 = 2;
const KEY_UP: u8 = 3;
const BUTTON_DOWN: u8 = 4;
const BUTTON_UP: u8 = 5;
const CURSOR_MOVE: u8 = 6;
const CURSOR_LEAVE: u8 = 8;
const GAIN_FOCUS: u8 = 9;
const LOSE_FOCUS: u8 = 10;
const WINDOW_RESIZE: u8 = 22;
const WINDOW_SELECT: u8 = 31;
const WINDOW_CLOSE: u8 = 128 | 33;

pub struct EventDetails {
	id: u8,
	detail: u32,
	xy: (i16, i16),
	wh: (u32, u32),
	utf8: Option<String>,
}

pub enum Event {
	Event(EventDetails),
	Stop,
}

impl Event {
	pub fn create(connection: xcb::Connection, state: VoidPointer) -> Event{
		let event = unsafe { xcb::poll_for_event(connection, state) };

		if let Some(e) = event {
			Event::Event(EventDetails {
				id: e.0,
				detail: e.1,
				xy: e.2,
				wh: (e .3 .0 as u32, e .3 .1 as u32),
				utf8: e.4
			})
		} else {
			Event::Stop
		}
	}

	pub fn poll(self, queue: &mut input::InputQueue, wh: &mut (u32, u32))
		-> bool
	{
		let e : EventDetails = if let Event::Event(details) = self {
			details
		} else {
			return false;
		};

		match e.id {
			KEY_DOWN => if let Some(key) = Key::create(e.detail) {
				if queue.len() == 0 {
					queue.key_down(key);
				} else {
					let input2 = queue.last();

					if let Input::KeyRelease(_) = input2 {
						queue.pop();
					} else {
						queue.key_down(key);
					}
				}
			} else if e.detail == 9 {
				queue.back();
			} else if e.detail == 95 {
				queue.fullscreen();
			},
			KEY_UP => if let Some(key) = Key::create(e.detail) {
				queue.key_up(key);
			},
			BUTTON_DOWN => match e.detail {
				bt @ 1...3 => queue.button_down(*wh, e.xy,
					mouse_button(bt)),
				sc @ 4...7 => queue.scroll(*wh, e.xy,
					scroll_wheel(sc), 1),
				uc => panic!("awi: Unknown Click {}!", uc)
			},
			BUTTON_UP => match e.detail {
				bt @ 1...3 => queue.button_up(*wh, e.xy,
					mouse_button(bt)),
				_ => {}
			},
			CURSOR_MOVE => queue.cursor_move(*wh, e.xy),
			CURSOR_LEAVE => queue.cursor_leave(),
			GAIN_FOCUS => queue.resume(),
			LOSE_FOCUS => queue.pause(),
			WINDOW_RESIZE => queue.resize(wh, e.wh),
			WINDOW_SELECT => println!("!SELECT!"),
			WINDOW_CLOSE => queue.back(),
			_ => { }, // ignore all other messages
		}

		if let Some(string) = e.utf8 {
			queue.text(string);
		}

		true
	}
}

fn scroll_wheel(button: u32) -> input::ScrollWheel {
	match button {
		4 => input::ScrollWheel::Up,
		5 => input::ScrollWheel::Down,
		6 => input::ScrollWheel::Left,
		7 => input::ScrollWheel::Right,
		_ => unreachable!(),
	}
}

fn mouse_button(button: u32) -> ::Click {
	match button {
		1 => ::Click::Left,
		2 => ::Click::Middle,
		3 => ::Click::Right,
		_ => unreachable!(),
	}
}

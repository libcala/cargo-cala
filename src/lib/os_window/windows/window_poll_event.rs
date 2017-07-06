// lib/os_window/windows/window_poll_event.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use ami::void_pointer::*;
use Input;
use input::keyboard::{ english, FSC, ESC };
use Key;
use super::{ convert_mouse_pos, should_resize };

static mut ADI_WNDPROCMSG : u8 = 0b0000_0000;

const RESIZED : u8 = 0b1000_0000;
const PAUSED : u8 = 0b0100_0000;
const RESUMED : u8 = 0b0010_0000;

// const KEY_DOWN: u32 = 2;
// const KEY_UP: u32 = 3;
const CURSOR_MOVE: u32 = 0x0200;
// const CURSOR_ENTER: u32 = 7;
// const CURSOR_LEAVE: u32 = 8;
// const GAIN_FOCUS: u32 = 9;
// const LOSE_FOCUS: u32 = 10;
// const WINDOW_RESIZE: u32 = 22;
const WINDOW_CLOSE: u32 = 0x0012;

const LBUTTON_DOWN: u32 = 0x0201;
const LBUTTON_UP: u32 = 0x0202;
const MBUTTON_DOWN: u32 = 0x0207;
const MBUTTON_UP: u32 = 0x0208;
const RBUTTON_DOWN: u32 = 0x0204;
const RBUTTON_UP: u32 = 0x0205;
const SCROLL: u32 = 0x020A;

#[repr(C)]
struct Point {
	x: isize, // long
	y: isize, // long
}

#[repr(C)]
struct Msg {
	hwnd: VoidPointer,
	message: u32,
	w_param: VoidPointer,
	l_param: VoidPointer,
	time: u32,
	pt: Point,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Rect {
	left: isize,
	top: isize,
	right: isize,
	bottom: isize,
}

extern "system" {
	fn PeekMessageW(lpMsg: *mut Msg, h_wnd: VoidPointer, msg_filter_min: u32,
		msg_filter_max: u32, remove_msg: u32) -> i32;
	fn TranslateMessage(lpMsg: *const Msg) -> i32;
	fn DispatchMessageW(lpMsg: *const Msg) -> usize;
	fn GetCursorPos(point: *mut Point) -> i32;
	fn ScreenToClient(h_wnd: VoidPointer, point: *mut Point) -> i32;
	fn GetClientRect(h_wnd: VoidPointer, out: *mut Rect) -> i32;
	fn PostQuitMessage(exit_code: i32) -> ();
	fn DefWindowProcW(hw: VoidPointer, uMsg: u32, wParam: VoidPointer,
		lParam: VoidPointer) -> isize;
}

pub extern "C" fn wnd_proc(h_wnd: VoidPointer, u_msg: u32, w_param: VoidPointer,
	l_param: VoidPointer) -> isize
{
	match u_msg {
		0x0007 => unsafe { ADI_WNDPROCMSG |= RESUMED },
		0x0008 => unsafe { ADI_WNDPROCMSG |= PAUSED },
		0x0010 => {
			unsafe { PostQuitMessage(0) }; // Successful exit
			return 1; // TRUE = Don't Close Window Yet
		},
		0x0024 => {
			unsafe { ADI_WNDPROCMSG |= RESIZED };
			return 0;
		},
		_ => {},
	}

	unsafe {
		DefWindowProcW(h_wnd, u_msg, w_param, l_param)
	}
}

fn get_mouse(window: VoidPointer, wh: &(u32, u32), is_miw: &mut bool)
	-> (f32, f32, bool)
{
	let mut pos = Point { x: 0, y: 0 };
	unsafe {
		GetCursorPos(&mut pos);
		ScreenToClient(window, &mut pos);
	}

	let pos = convert_mouse_pos(wh, (pos.x as i16, pos.y as i16));

	let miw = pos.0 >= -1.0 && pos.0 <= 1.0 && pos.1 >= -1.0
		&& pos.1 <= 1.0;

	let miw_changed = if *is_miw != miw {
		*is_miw = miw;
		true
	} else {
		false
	};
	(pos.0, pos.1, miw_changed)
}

pub fn window_poll_event(window: VoidPointer, input: &mut Vec<Input>,
	miw: &mut bool, wh: &mut (u32, u32)) -> bool
{
	let mut msg = Msg { hwnd: NULL, message: 0, w_param: NULL, l_param: NULL,
		time: 0, pt: Point { x: 0, y: 0 } };

	if unsafe { ADI_WNDPROCMSG & RESIZED != 0 } {
		let clia = {
			let mut clia = Rect { left: 0, right: 0, top: 0, bottom: 0 };
			unsafe {
				GetClientRect(window, &mut clia);
			}
			println!("LT: {} {}", clia.left, clia.top);
			(clia.right as u32, clia.bottom as u32)
		};

		if should_resize(wh, clia) {
			input.push(Input::Resize);
		}
		unsafe { ADI_WNDPROCMSG &= !RESIZED };
		return true;
	}
	
	if unsafe { ADI_WNDPROCMSG & PAUSED != 0 } {
		input.push(Input::Pause);
		unsafe { ADI_WNDPROCMSG &= !PAUSED };
		return true;
	}
	
	if unsafe { ADI_WNDPROCMSG & RESUMED != 0 } {
		input.push(Input::Resume);
		unsafe { ADI_WNDPROCMSG &= !RESUMED };
		return true;
	}

	let (x, y, miw_changed) = get_mouse(window, wh, miw);
	if miw_changed {
		input.push(if *miw {
			Input::EnterWindow
		} else {
			Input::LeaveWindow
		});
		return true;
	}

	if unsafe {
		PeekMessageW(&mut msg, NULL, 0, 0, 0x0001)
	} == 0 { // no messages available
		return false;
	}
	match msg.message {
		WINDOW_CLOSE => {
			input.push(Input::Back)
		},
		CURSOR_MOVE => {
			input.push(Input::Cursor(x,y))
		},
		LBUTTON_DOWN => {
			input.push(Input::LeftDown(x,y))
		},
		LBUTTON_UP => {
			input.push(Input::LeftUp(x,y))
		},
		MBUTTON_DOWN => {
			input.push(Input::MiddleDown(x,y))
		},
		MBUTTON_UP => {
			input.push(Input::MiddleUp(x, y))
		},
		RBUTTON_DOWN => {
			input.push(Input::RightDown(x, y))
		},
		RBUTTON_UP => {
			input.push(Input::RightUp(x, y))
		},
		0x0100 | 0x0104 => {
			let scan = ((msg.l_param
				& 0b00000001_11111111_00000000_00000000) >> 16)
					.as_int() as u16;
			let chr = english(msg.w_param.as_int() as u16, scan);
			if msg.l_param & 0b01000000_00000000_00000000_00000000
				!= NULL
			{
				match chr {
					// These keys shouldn't repeat.
					Key::Char(ESC) | Key::Char(FSC) |
						Key::Insert | Key::Compose |
						Key::NumLock | Key::Shift(_) |
						Key::Ctrl(_) | Key::Alt(_)
					=> { } // ignore
					_ => input.push(Input::KeyRepeat(chr))
				}
			} else {
				input.push(Input::KeyDown(chr))
			}
		}
		0x0101 | 0x0105 => {
			let scan = ((msg.l_param
				& 0b00000001_11111111_00000000_00000000) >> 16)
					.as_int() as u16;
			let chr = english(msg.w_param.as_int() as u16, scan);
			input.push(Input::KeyUp(chr))
		}
		SCROLL => {
			let a = (((msg.w_param.as_int() as u32) >> 16) & 0xFFFF)
				as i16;

			if a > 0 {
				let reps = a / 120;
				for _ in 0..reps {
					input.push(Input::ScrollUp(x, y))
				}
			} else {
				let reps = a / -120;
				for _ in 0..reps {
					input.push(Input::ScrollDown(x, y))
				}
			}
		}
		// ignore all other messages
		_ => {
			unsafe {
				TranslateMessage(&msg);
				DispatchMessageW(&msg);
			}
//			println!("Unknown event: {0:x}", x);
		}
	};

	true
}

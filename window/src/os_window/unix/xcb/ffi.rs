// lib/os_window/unix/xcb/ffi.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use ami::void_pointer::*;

#[repr(C)]
struct XcbClientMessageEvent {
	response_type: u8,
	format: u8,
	sequence: u16,
	window: u32,
	stype: u32,
	data32: [u32; 5],
}

#[repr(C)]
struct XcbGenericEvent {
	response_type: u8,
	detail: u8,
	sequence: u16,
	timestamp: u32,
	root: u32,
	event: u32,
	child: u32,
	root_x: i16,
	root_y: i16,
	event_x: i16,
	event_y: i16,
	state: u16,
	same_screen: u8,
	pad0: u8,
}

#[derive(Copy, Clone)]
pub struct Dl {
	pub dl_handle: VoidPointer,
	pub dl_handl2: VoidPointer,
	xcb_send_event: unsafe extern "C" fn(c: VoidPointer, p: u8, d: u32,
		m: u32, e: *const XcbClientMessageEvent) -> (),
	xcb_poll_for_event: unsafe extern "C" fn(c: VoidPointer)
		-> *mut XcbGenericEvent,
	xcb_flush : unsafe extern "C" fn(c: VoidPointer) -> i32,
	
}

pub type Connection = (VoidPointer, Dl);

pub unsafe fn load_dl() -> Dl {
	extern { fn dlopen(name: *const u8, flags: i32) -> VoidPointer; }

	let xcb = b"libxcb.so.1\0";
	let xcb_keysyms = b"libxkbcommon-x11.so.0\0";

	let dl_handle = dlopen(&xcb[0], 1);
	let dl_handl2 = dlopen(&xcb_keysyms[0], 1);

	Dl {
		dl_handle: dl_handle,
		dl_handl2: dl_handl2,
		xcb_send_event: dlsym(dl_handle, b"xcb_send_event\0"),
		xcb_poll_for_event: dlsym(dl_handle, b"xcb_poll_for_event\0"),
		xcb_flush: dlsym(dl_handle, b"xcb_flush\0"),
	}
}

unsafe fn dlsym<T>(lib: VoidPointer, name: &[u8]) -> T {
	extern {
		fn dlsym(handle: VoidPointer, symbol: *const u8) -> VoidPointer;
	}

	let function_ptr = dlsym(lib, &name[0]);
	::std::mem::transmute_copy::<VoidPointer, T>(&function_ptr)
}

unsafe fn intern_atom(connection: Connection, name: &[u8]) -> u32 {
	let xcb_intern_atom : unsafe extern "C" fn(c: VoidPointer, e: u8,
		l: u16, n: *const u8) -> u32
		= dlsym(connection.1.dl_handle, b"xcb_intern_atom\0");

	xcb_intern_atom(connection.0, 0, name.len() as u16, &name[0])
}

unsafe fn intern_atom_reply(connection: Connection, atom: u32) -> u32 {
	#[repr(C)]
	struct XcbInternAtomReply {
		response_type: u8,
		pad0: u8,
		sequence: u16,
		length: u32,
		atom: u32,
	}

	extern { fn free(this: *mut XcbInternAtomReply) -> (); }
	let xcb_intern_atom_reply : unsafe extern "C" fn(c: VoidPointer,
		cookie: u32, e: VoidPointer) -> *mut XcbInternAtomReply
		= dlsym(connection.1.dl_handle, b"xcb_intern_atom_reply\0");

	let reply = xcb_intern_atom_reply(connection.0, atom, NULL);
	let atom = (*reply).atom;

	free(reply);

	atom
}

pub unsafe fn get_atom(connection: Connection, name: &[u8]) -> u32 {
	intern_atom_reply(connection, intern_atom(connection, name))
}

pub unsafe fn change_property(connection: Connection, window: u32, t: u32,
	a: u32, data: &[u32])
{
	let xcb_change_property : unsafe extern "C" fn(c: VoidPointer, mode: u8,
		window: u32, property: u32, t: u32, format: u8, data_len: u32,
		data: *const u32) -> u32
		= dlsym(connection.1.dl_handle, b"xcb_change_property\0");

	let len = data.len() as u32;
	let ptr = &data[0];

	xcb_change_property(connection.0, 0, window, a, t, 32, len, ptr);
}

pub unsafe fn change_property_title(connection: Connection, window: u32,
	title: &[u8])
{
	let xcb_change_property : unsafe extern "C" fn(c: VoidPointer, mode: u8,
		window: u32, property: u32, t: u32, format: u8, data_len: u32,
		data: *const u8) -> u32
		= dlsym(connection.1.dl_handle, b"xcb_change_property\0");

	let atom2 = get_atom(connection, b"_NET_WM_NAME");
	let atom = get_atom(connection, b"UTF8_STRING");
	let len = title.len() as u32;
	let ptr = &title[0];

	xcb_change_property(connection.0, 0, window, atom2, atom, 8, len, ptr);
//	xcb_change_property(connection.0, 0, window, 37, atom, 8, len, ptr);
}

pub unsafe fn send_event(connection: Connection, window: u32, a: (u32,u32)) {
	let xcb_send_event = connection.1.xcb_send_event;

	let event = XcbClientMessageEvent {
		response_type: 33, // Client Message
		format: 32,
		sequence: 0,
		window: window,
		stype: a.0,
		data32: [2, a.1, 0, 0, 0],
	};

	xcb_send_event(connection.0, 1, window, 1048576 | 524288, &event);
}

pub unsafe fn map_window(connection: Connection, window: u32) {
	let xcb_map_window : unsafe extern "C" fn(c: VoidPointer, w: u32) -> u32
		= dlsym(connection.1.dl_handle, b"xcb_map_window\0");

	xcb_map_window(connection.0, window);
}

pub unsafe fn screen_root(connection: Connection) -> (u32, u32, u32) {
	#[repr(C)]
	pub struct XcbScreen {
		root: u32,
		default_colormap: u32,
		white_pixel: u32,
		black_pixel: u32,
		current_input_masks: u32,
		width_in_pixels: u16,
		height_in_pixels: u16,
		width_in_millimeters: u16,
		height_in_millimeters: u16,
		min_installed_maps: u16,
		max_installed_maps: u16,
		root_visual: u32,
		backing_stores: u8,
		save_unders: u8,
		root_depth: u8,
		allowed_depths_len: u8,
	}

	#[repr(C)]
	struct XcbScreenIterator {
		data: *mut XcbScreen,
		rem: i32,
		index: i32,
	}

	let xcb_get_setup : unsafe extern "C" fn(c: VoidPointer) -> VoidPointer
		= dlsym(connection.1.dl_handle, b"xcb_get_setup\0");
	let xcb_setup_roots_iterator : unsafe extern "C" fn(setup: VoidPointer)
		-> XcbScreenIterator
		= dlsym(connection.1.dl_handle, b"xcb_setup_roots_iterator\0");

	let setup = xcb_get_setup(connection.0);
	let screen = xcb_setup_roots_iterator(setup).data;

	((*screen).root, (*screen).root_visual, (*screen).black_pixel)
}

pub unsafe fn generate_id(connection: Connection) -> u32 {
	let xcb_generate_id : unsafe extern "C" fn(c: VoidPointer) -> u32
		= dlsym(connection.1.dl_handle, b"xcb_generate_id\0");

	xcb_generate_id(connection.0)
}

pub unsafe fn create_window(connection: Connection, window: u32,
	rvb: (u32, u32, u32)) -> ()
{
	let xcb_create_window : unsafe extern "C" fn(c: VoidPointer, depth: u8,
		id: u32, parent: u32, x: i16, y: i16, w: u16, h: u16,
		border_width: u16, class: u16, visual: u32, vmask: u32,
		vlist: *mut u32) -> u32
		= dlsym(connection.1.dl_handle, b"xcb_create_window\0");

	let (root, visual, black) = rvb;
	let mut value_list = [black, 0b01000100000000001101111];
	xcb_create_window(connection.0, 0, window, root, 0, 0, ::MWW as u16,
		::MWH as u16, 0, 1, visual, 2|2048, &mut value_list[0]);
}

pub unsafe fn connect(so: VoidPointer) -> VoidPointer {
	let xcb_connect : unsafe extern "C" fn(displayname: VoidPointer,
		s: VoidPointer) -> VoidPointer = dlsym(so, b"xcb_connect\0");

	let connection = xcb_connect(NULL, NULL);

	if connection == NULL {
		panic!("Couldn't connect to X Server.");
	}

	connection
}

pub unsafe fn flush(connection: Connection) -> () {
	let xcb_flush = connection.1.xcb_flush;

	xcb_flush(connection.0);
}

pub unsafe fn disconnect(connection: Connection) -> () {
	extern { fn dlclose(handle: VoidPointer) -> i32; }
	let xcb_disconnect : unsafe extern "C" fn(c: VoidPointer) -> () =
		dlsym(connection.1.dl_handle, b"xcb_disconnect\0");

	xcb_disconnect(connection.0);
	dlclose(connection.1.dl_handle);
}

pub unsafe fn poll_for_event(connection: Connection, state: VoidPointer)
	-> Option<(u8, u32, (i16, i16), (i16, i16), Option<String>)>
{
	use super::super::input::key;
	use std::string::String;

	extern { fn free(event: *mut XcbGenericEvent) -> (); }
	let xcb_poll_for_event = connection.1.xcb_poll_for_event;

	let event = xcb_poll_for_event(connection.0);

	if event.is_null() {
		return None;
	}

	let response_type = (*event).response_type;
	let detail = (*event).detail as u32;
	let event_xy = ((*event).event_x, (*event).event_y);
	let root_xy = ((*event).root_x, (*event).root_y);

	free(event);

	let string = match response_type {
		2 => {
			Some(match detail {
				key::LEFT => String::from("\u{91}"),
				key::RIGHT => String::from("\u{92}"),
				key::UP => String::from("\u{9E}"),
				key::DOWN => String::from("\u{9F}"),
				key::ENTER | key::ext::NUM_PAD_ENTER
					=> String::from("\n"),
				key::LEFT_SHIFT | key::RIGHT_SHIFT |
					key::ext::ALT_GR | key::ext::NUMLOCK |
					9 =>
				{
					xkb_state_update_key(connection, state,
						detail, true);
					String::from("")
				},
				_ => {
					xkb_state_key_get_utf8(connection,
						state, detail)
				}
			})
		},
		3 => {
			xkb_state_update_key(connection, state, detail, false);
			None
		},
		_ => None
	};

	Some((response_type, detail, event_xy, root_xy, string))
}

pub unsafe fn xkb_get_core_keyboard_device_id(connection: Connection) -> i32 {
	let xkb_x11_get_core_keyboard_device_id : unsafe extern "C"
		fn(c: VoidPointer) -> i32 = dlsym(connection.1.dl_handl2,
		b"xkb_x11_get_core_keyboard_device_id\0");

	xkb_x11_get_core_keyboard_device_id(connection.0)
}

pub unsafe fn xkb_context_new(connection: Connection) -> VoidPointer {
	#[repr(C)]
	enum ContextFlags { NoFlags = 0 }

	let xkb_context_new : unsafe extern "C" fn(f: ContextFlags)
		-> VoidPointer
		= dlsym(connection.1.dl_handl2, b"xkb_context_new\0");

	xkb_context_new(ContextFlags::NoFlags)
}

pub unsafe fn xkb_x11_keymap_new_from_device(connection: Connection,
	xkbctx: VoidPointer, device_id: i32) -> VoidPointer
{
	#[repr(C)]
	enum CompileFlags { NoFlags = 0 }

	let xkb_x11_keymap_new_from_device : unsafe extern "C"
		fn(context: VoidPointer, connection: VoidPointer,
			device_id: i32, flags: CompileFlags) -> VoidPointer
		= dlsym(connection.1.dl_handl2,
			b"xkb_x11_keymap_new_from_device\0");

	xkb_x11_keymap_new_from_device(xkbctx, connection.0, device_id,
		CompileFlags::NoFlags)
}

pub unsafe fn xkb_x11_state_new_from_device(connection: Connection,
	keymap: VoidPointer, device_id: i32) -> VoidPointer
{
	let xkb_x11_state_new_from_device : unsafe extern "C" fn(
		keymap: VoidPointer, connection: VoidPointer, device_id: i32)
		-> VoidPointer = dlsym(connection.1.dl_handl2,
			b"xkb_x11_state_new_from_device\0");

	xkb_x11_state_new_from_device(keymap, connection.0, device_id)
}

unsafe fn xkb_state_update_key(connection: Connection, state: VoidPointer,
	keycode: u32, dn: bool)
{
	#[allow(dead_code)]
	#[repr(C)]
	enum StateComponent { None }

	#[repr(C)]
	enum KeyDirection {
		Up,
		Down,
	}

	let xkb_state_update_key : unsafe extern "C" fn(state: VoidPointer,
		key: u32, direction: KeyDirection) -> StateComponent
		= dlsym(connection.1.dl_handl2, b"xkb_state_update_key\0");

	xkb_state_update_key(state, keycode, if dn {
		KeyDirection::Down
	} else {
		KeyDirection::Up
	});
}

unsafe fn xkb_state_key_get_utf8(connection: Connection, state: VoidPointer,
	key: u32) -> String
{
	let xkb_state_key_get_utf8 : unsafe extern "C" fn(state: VoidPointer,
		keycode: u32, buffer: *mut u8, size: usize) -> i32
		= dlsym(connection.1.dl_handl2, b"xkb_state_key_get_utf8\0");

	let size = xkb_state_key_get_utf8(state, key, ::std::ptr::null_mut(), 0)
		as usize + 1;
	let mut utf8 = Vec::new();

	utf8.resize(size, b'\0'); // Size + 1 to include NULL byte from XKB.

	let buffer = utf8.as_mut_ptr();

	xkb_state_key_get_utf8(state, key, buffer, size);

	utf8.pop();

	// TODO: Validate that is valid
	::std::string::String::from_utf8(utf8).unwrap()
}

pub unsafe fn use_xkb_extension(connection: Connection) {
	let xcb_xkb_use_extension : unsafe extern "C" fn(
		connection: VoidPointer, major: u16, minor: u16) -> u32
		= dlsym(connection.1.dl_handl2, b"xcb_xkb_use_extension\0");

	xcb_xkb_use_extension(connection.0, 1, 0);
}

pub unsafe fn xkb_state_unref(connection: Connection, state: VoidPointer) {
	let xkb_state_unref : unsafe extern "C" fn(state: VoidPointer) -> ()
		= dlsym(connection.1.dl_handl2, b"xkb_state_unref\0");

	xkb_state_unref(state);
}

pub unsafe fn xkb_keymap_unref(connection: Connection, keymap: VoidPointer) {
	let xkb_keymap_unref : unsafe extern "C" fn(state: VoidPointer) -> ()
		= dlsym(connection.1.dl_handl2, b"xkb_keymap_unref\0");

	xkb_keymap_unref(keymap);
}

pub unsafe fn xkb_context_unref(connection: Connection, context: VoidPointer) {
	let xkb_context_unref : unsafe extern "C" fn(state: VoidPointer) -> ()
		= dlsym(connection.1.dl_handl2, b"xkb_context_unref\0");

	xkb_context_unref(context);
}

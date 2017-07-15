// lib/os_window/android/glue.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use ami::void_pointer::*;

/*const LOOPER_ID_MAIN: i32 = 1;
const LOOPER_ID_INPUT: i32 = 2;
const LOOPER_ID_USER: i32 = 3;

// HM... https://developer.android.com/ndk/reference/looper_8h.html

// from https://developer.android.com/ndk/reference/configuration_8h.html
enum AConfiguration {
	ACONFIGURATION_ORIENTATION_ANY = 0x0000,
	// ...
}

enum AppCmd {
	InputChanged,
	InitWindow,
	TermWindow,
	WindowResized,
	WindowRedrawNeeded,
	ContentRectChanged,
	GainFocus,
	LoseFocus,
	ConfigChanged,
	LowMemory,
	Start,
	Resume,
	SaveState,
	Pause,
	Stop,
	Destroy,
}

struct AndroidApp {
	user_data: VoidPointer,
	on_app_cmd: fn(*mut AndroidApp, cmd: i32) -> (),
	// return 1 if event handled, 0 otherwise
	on_input_event: fn(*mut AndroidApp, *mut VoidPointer) -> i32,
	activity: *mut ANativeActivity,
	config: *mut AConfiguration,
	saved_state: VoidPointer,
	saved_state_size: usize,
	looper: VoidPointer, // Pointer To ALooper
	input_queue: VoidPointer,
	window: VoidPointer,
	content_rect: ARect,
	activity_state: i32,
	destroy_requested: i32,
	// private implementation of glue code
}

struct AndroidPollSource {
	id: i32, // LOOPER_ID_MAIN or LOOPER_ID_INPUT
	app: *mut AndroidApp,
	process: fn(*mut AndroidApp, *mut AndroidPollSource) -> (),
}

extern "C" fn ANativeActivity_onCreate(activity: *mut ANativeActivity,
	saved_state: VoidPointer, saved_state_size: usize) -> ()
{
	log_write("Creating Activity Ya\n");
	::std::process::exit(0);
}*/

pub fn log_write(what: &str) -> () {
	extern "C" {
		fn __android_log_write(prio: i32, tag: *const u8, text: *const u8) -> i32;
	}

	unsafe {
		__android_log_write(3, b"GSP".as_ptr(), what.as_ptr());
	}
}

struct ARect {
	left: i32,
	top: i32,
	right: i32,
	bottom: i32,
}

// from https://developer.android.com/ndk/reference/struct_a_native_activity.html
#[repr(C)]
struct ANativeActivityCallbacks {
	start: fn(*mut ANativeActivity) -> (),
	resume: fn(*mut ANativeActivity) -> (),
	save_instance_state: fn(*mut ANativeActivity, *mut usize) -> (),
	pause: fn(*mut ANativeActivity) -> (),
	stop: fn(*mut ANativeActivity) -> (),
	destroy: fn(*mut ANativeActivity) -> (),
	window_focus_change: fn(*mut ANativeActivity, has_focus: i32) -> (),
	native_window_created: fn(*mut ANativeActivity, VoidPointer) -> (),
	native_window_resized: fn(*mut ANativeActivity, VoidPointer) -> (),
	native_window_redraw_needed: fn(*mut ANativeActivity, VoidPointer)->(),
	native_window_destroyed: fn(*mut ANativeActivity, VoidPointer) -> (),
	input_queue_created: fn(*mut ANativeActivity, VoidPointer) -> (),
	input_queue_destroyed: fn(*mut ANativeActivity, VoidPointer) -> (),
	content_rect_changed: fn(*mut ANativeActivity, *mut ARect) -> (),
	configuration_changed: fn(*mut ANativeActivity) -> (),
	low_memory: fn(*mut ANativeActivity) -> (),
}

#[repr(C)]
pub struct ANativeActivity {
	callbacks: *mut ANativeActivityCallbacks,
	vm: VoidPointer,
	env: VoidPointer,
	activity: VoidPointer, // jobject
	internal_data_path: *const i8,
	external_data_path: *const i8,
	sdk_version: i32,
	instance: VoidPointer,
	asset_manager: VoidPointer,
	obb_path: *const i8,
}

#[allow(unused)]
#[no_mangle]
pub extern "C" fn gsp_main(activity: *mut ANativeActivity) -> () {
	println!("Got Start");
	log_write("Creating Activity Ya\n");

/*	// loop waiting for stuff to do.
	while (1) {
	// Read all pending events.
	int ident;
	int events;
	struct android_poll_source* source;
	
	while ((ident=ALooper_pollAll(0, NULL, &events, (void**)&source)) >= 0) {
	if (state->destroyRequested != 0) {
	return;
	}
	}
	}*/
}

// bin/res/main.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

#[macro_use]
extern crate window;

mod app;

pub use app::*;

#[no_mangle] pub extern "C" fn stn_main() -> () {
	main()
}

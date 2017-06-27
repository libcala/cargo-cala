// main.rs
// Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

extern crate rand;

use std::path::Path;
use std::env;
use std::ffi::OsString;

mod cli;
mod file;
mod password;
mod program;

const KEYSTORE: &'static str = "app/android-release-key.keystore";
const UNSIGNED: &'static str = "app/build/outputs/apk/app-release-unsigned.apk";
const APK_OUT: &'static str = "app/build/outputs/apk/todo.apk"; // TODO
const PASSWORD: &'static str = ".packager/password.text";

/// Deploy the apk.
fn deploy(adb: &str) {
	// Push apk to phone.
	program::execute_log(adb,
		vec![
			"install",
			"-r",
			APK_OUT,
		],
		"couldn't run adb install", "adb install failed");
}

fn execute(adb: &str) {
	// Start app from adb.
	program::execute_log(adb,
		vec![
			"shell",
			"am",
			"start",
			"-n", "com.android.gl2jni/com.android.gl2jni.GL2JNIActivity"
		],
		"couldn't run adb shell", "adb failed to start app");

	// Log output from adb.
//	program::execute_log(adb,
//		vec![
//			"logcat",
//			"-b", "main",
//		],
//		"couldn't run adb logcat", "adb failed to start log");
}

fn run(android_home: OsString) {
	let mut adb = android_home.clone();

	// Get adb
	let adb : &str = {
		adb.push("/platform-tools/adb");
		adb.to_str().unwrap()
	};

	// Install APK on the Phone
	deploy(adb);

	// Run the APK on the Phone
	execute(adb);
}

fn main() {
	// Start of program
	cli::print("*Packager* - Aldaron's Tech");

	// Get Android Home.
	let android_home = match env::var_os("ANDROID_HOME") {
		Some(val) => val,
		None => {
			program::exit("ANDROID_HOME is not defined!");
			unreachable!()
		},
	};
	let mut zipalign = android_home.clone();
	
	// Get zipalign
	let zipalign : &str = {
		zipalign.push("/build-tools/25.0.2/zipalign");
		zipalign.to_str().unwrap()
	};

	// Generate the keytool
	if Path::new(KEYSTORE).exists() == false {
		// Generate the password
		let password = password::new();

		// Save the password.
		file::save(PASSWORD, password.as_slice());

		// Password as string.
		let password = String::from_utf8(password).unwrap();

		// Generate the Keystore
		program::execute("Keytool", "keytool",
			vec![
				"-sigalg", "SHA1withRSA",
				"-keyalg", "RSA",
				"-keysize", "1024",
				"-genkey",
				"-keystore", KEYSTORE,
// TODO: Read Program Name For This
//				"-alias", "daliasle",
				"-validity", "36500", // 100 years.
				"-keypass", password.as_str(),
				"-storepass", password.as_str(),
			],
			"keytool not installed!", "keytool failed!");
	}

	// Load Password.
	let password = String::from_utf8(file::load(PASSWORD)).unwrap();

	// Compile Java Code.
	program::execute("Gradle", "./gradlew", vec!["assembleRelease"],
		"Gradle failed to start ( is ./gradlew in the current directory\
			? )", "java compile failed!");

	// Sign the APK
	program::execute("Jarsigner", "jarsigner",
		vec![
			"-verbose",
			"-tsa", "http://timestamp.digicert.com",
			"-sigalg", "SHA1withRSA",
			"-digestalg", "SHA1",
			"-keystore", KEYSTORE,
			"-storepass", &password,
			UNSIGNED, "mykey" // TODO: read program name daliasle
		],
		"jarsigner not installed!", "couldn't sign apk!");

	// Align The APK
	program::execute("Zipalign", zipalign,
		vec![
			"-f",
			"-v",
			"4",
			UNSIGNED,
			APK_OUT,
		],
		"zipalign for 25.0.2 is not installed", "zipalign failed");

	// Install & Run APK on the Phone
	run(android_home);
}

// bin/android.rs
// Graphical Software Packager
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use RES_ICON;
use TARGET;
use TARGET_DIR;
use PASSWORD;

use std::env;
use std::path::Path;
use std::ffi::OsString;

use parse;
use resources;
use program;
use file;
use password;

/// Deploy the apk.
fn deploy(adb: &str, apk_out: &str) {
	println!("DEPLOY");
	// Push apk to phone.
	program::execute_log(adb,
		vec![
			"install",
			"-r",
			apk_out,
		]);
}

fn execute_app(adb: &str) {
	// Start app from adb.
	println!("SHELL START IT");
	program::execute_log(adb,
		vec![
			"shell",
			"am",
			"start",
			"-n", "com.company.app/android.app.NativeActivity"
		]);

	println!("LOG");
	// Log output from adb.
	program::execute_log(adb, vec!["-c"]);
	program::execute_log(adb,
		vec![
			"logcat", "-b", "main", "*:s", "GSP:i"
		]);
}

fn run(android_home: OsString, apk_out: &str) {
	let mut adb = android_home.clone();

	// Get adb
	let adb : &str = {
		adb.push("/platform-tools/adb");
		adb.to_str().unwrap()
	};

	// Install APK on the Phone
	deploy(adb, apk_out);

	// Run the APK on the Phone
	execute_app(adb);
}

fn cargo(cargo_toml: &parse::CargoToml, target: &str) {
	program::execute_log("cargo", vec![
		"build", "--release", "--lib", "--target", target,
	]);
}

fn inject_exe(cargo_toml: &parse::CargoToml, apk: &String, target: &str,
	folder: &str)
{
	file::copy(&format!("target/{}/release/libgsp_program.a", target),
		&format!("target/apk/src/main/jniLibs/{}/libgsp.a", folder));
}

pub fn execute(cargo_toml: parse::CargoToml, translations: resources::Lang) {
	let apk = "target/apk/".to_string();

	let keystore = {
		let mut x = apk.clone();

		x.push_str("android-release-key.keystore");
		x
	};

//	let target_dir = TARGET_DIR.to_string();

	// Get Android Home.
	let android_home = match env::var_os("ANDROID_HOME") {
		Some(val) => val,
		None => {
			program::exit("ANDROID_HOME is not defined!");
			unreachable!()
		},
	};
	let mut zipalign = android_home.clone();
	let mut android = android_home.clone();
	
	// Get zipalign
	let zipalign : &str = {
		zipalign.push("/build-tools/25.0.2/zipalign");
		zipalign.to_str().unwrap()
	};

	// App Domain
	let app_domain = format!("{}.{}", &cargo_toml.domain,
		&cargo_toml.subdomain).to_string();

	// Build the program
	cargo(&cargo_toml, "arm-linux-androideabi");
	cargo(&cargo_toml, "armv7-linux-androideabi");
	cargo(&cargo_toml, "aarch64-linux-android");
	cargo(&cargo_toml, "i686-linux-android");

	// Copy Executable Into APK
	inject_exe(&cargo_toml, &apk, "arm-linux-androideabi", "armeabi");
	inject_exe(&cargo_toml, &apk, "armv7-linux-androideabi", "armeabi-v7a");
	inject_exe(&cargo_toml, &apk, "i686-linux-android", "x86");
	inject_exe(&cargo_toml, &apk, "aarch64-linux-android", "arm64-v8a");

	// Copy Icon Into APK
	{
		let dst = format!("{}src/main/res/mipmap/app_icon.png", &apk);

		file::copy(RES_ICON, &dst).unwrap();
	}

	// Create AndroidManifest.xml
	file::save(&format!("{}src/main/AndroidManifest.xml", &apk),
		format!("\
			<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
			<manifest xmlns:android=\"http://schemas.android.com/apk/res/android\"\n\
			\tpackage=\"com.company.app\"\n\
			\tandroid:versionCode=\"10\"\n\
			\tandroid:versionName=\"0.1.0\">\n\
			\n\
			\t<application\n\
			\t\tandroid:allowBackup=\"false\"\n\
			\t\tandroid:fullBackupContent=\"false\"\n\
			\t\tandroid:icon=\"@mipmap/app_icon\"\n\
			\t\tandroid:label=\"@string/app_name\"\n\
			\t\tandroid:hasCode=\"false\">\n\
			\n\
			\t\t<activity android:name=\"android.app.NativeActivity\"\n\
			\t\t\tandroid:configChanges=\"orientation\">\n\
			\n\
			\t\t\t<intent-filter>\n\
			\t\t\t\t<action android:name=\n\
			\t\t\t\t\t\"android.intent.action.MAIN\" />\n\
			\t\t\t\t<category android:name=\n\
			\t\t\t\t\t\"android.intent.category.LAUNCHER\" />\n\
			\t\t\t</intent-filter>\n\
			\t\t</activity>\n\
			\t</application>\n\
			</manifest>\n\
		").as_bytes()
	);

	// Create strings.xml
	file::save(&format!("{}src/main/res/values/strings.xml", &apk),
		format!("\
			<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
			<resources>\n\
			\t<string name=\"app_name\">Simple</string>\n\
			</resources>\n\
		").as_bytes()
	);

	// build.gradle
	let build_gradle = format!("{}build.gradle", &apk);

	// GRADLEW
	let gradlew = format!("{}gradlew", &apk);

	// Generate GRADLEW if doesn't exist.
	if Path::new(&gradlew).exists() == false {
		file::save(&build_gradle,
			format!("\
				task wrapper(type: Wrapper) {{\n\
					gradleVersion = '3.3'\n\
				}}\n"
			).as_bytes()
		);

		program::execute("Gradle", "gradle",
			vec!["wrapper", "--build-file", &build_gradle],
			"Gradle was not found (Install it!)", "gradlew failed");
	}

	// Create build.gradle
	file::save(&build_gradle,
		format!("\
			apply plugin: 'com.android.application'\n\
			\n\
			buildscript {{\n\
			\trepositories {{\n\
			\t\tjcenter()\n\
			\t}}\n\
			\tdependencies {{\n\
			\t\tclasspath 'com.android.tools.build:gradle:2.3.0'\n\
			\t}}\n\
			}}\n\
			\n\
			allprojects {{\n\
			\trepositories {{\n\
			\t\tjcenter()\n\
			\t}}\n\
			}}\n\
			\n\
			android {{\n\
			\tcompileSdkVersion = 25\n\
			\tbuildToolsVersion = '25.0.2'\n\
			\n\
			\tdefaultConfig {{\n\
			\t\tapplicationId = 'com.company.app'\n\
			\t\tminSdkVersion   9\n\
			\t\ttargetSdkVersion  25\n\
			\t\tndk {{\n\
			\t\t\tabiFilters 'x86', 'armeabi', 'armeabi-v7a', 'arm64-v8a'\n\
			\t\t}}\n\
			\t\texternalNativeBuild {{\n\
			\t\t\tcmake {{\n\
			\t\t\t\targuments '-DANDROID_PLATFORM=android-9',\n\
			\t\t\t\t\t'-DANDROID_TOOLCHAIN=clang', '-DANDROID_STL=gnustl_static'\n\
			\t\t\t}}\n\
			\t\t}}\n\
			\t}}\n\
			\tbuildTypes {{\n\
			\t\trelease {{\n\
			\t\t\tminifyEnabled false\n\
			\t\t\t\tproguardFiles getDefaultProguardFile('proguard-android.txt'),\n\
			\t\t\t\t\t'proguard-rules.pro'\n\
			\t\t}}\n\
			\t}}\n\
			\texternalNativeBuild {{\n\
			\t\tcmake {{\n\
			\t\t\tpath 'c/CMakeLists.txt'\n\
			\t\t}}\n\
			\t}}\n\
			}}\n\
		").as_bytes()
	);

	// CMakeLists.txt
	{
		let data = "\
			cmake_minimum_required(VERSION 3.4.1)\n\
			set(${CMAKE_C_FLAGS}, \"${CMAKE_C_FLAGS} -v\")\n\
			add_library(native_app_glue STATIC ${ANDROID_NDK}/sources/android/native_app_glue/android_native_app_glue.c)\n\
			set(CMAKE_CXX_FLAGS \"${CMAKE_CXX_FLAGS} -std=gnu++11 -v\")\n\
			set(CMAKE_SHARED_LINKER_FLAGS \"${CMAKE_SHARED_LINKER_FLAGS} -u ANativeActivity_onCreate -v\")\n\
			add_library(gsp STATIC IMPORTED)\n\
			set_target_properties(gsp PROPERTIES IMPORTED_LOCATION\n\
			\t/home/aldaron/aldarons_tech/cargo-gsp/demo/target/apk/src/main/jniLibs/${ANDROID_ABI}/libgsp.a)\n\
			add_library(main SHARED main.c)\n\
			target_include_directories(main PRIVATE ${ANDROID_NDK}/sources/android/native_app_glue)\n\
			target_link_libraries(main gsp android native_app_glue EGL GLESv1_CM dl log)\
		";

		file::save(&format!("{}c/CMakeLists.txt", &apk), data.as_bytes());
	}

	// main.c
	{
		let data = "\
			#include <android/log.h>\n\
			#include <android_native_app_glue.h>\n\
			\n\
			#define LOGI(...) ((void)__android_log_print(ANDROID_LOG_INFO, \"GSP\", __VA_ARGS__))\n\
			#define LOGW(...) ((void)__android_log_print(ANDROID_LOG_WARN, \"GSP\", __VA_ARGS__))\n\
			\n\
			void stn_main(void);\n\
			\n\
			struct android_app* gsp_state = NULL;\n\
			\n\
			void android_main(struct android_app* state) {\n\
			\tLOGI(\"Starting....\\n\");\n\
			\tgsp_state = state;\n\
			\tstn_main();\n\
			}\n\
		";

		file::save(&format!("{}c/main.c", &apk), data.as_bytes());
	}

	// Compile Java Code.
	program::execute("Gradle", &gradlew,
		vec!["assembleRelease", "--build-file", &build_gradle, "--stacktrace"],
		"Gradle was not found (install IT!)", "java compile failed!");

	// ....

	// Generate the keytool
	if Path::new(&keystore).exists() == false {
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
				"-keystore", &keystore,
				"-alias", &cargo_toml.subdomain,
				"-validity", "36500", // 100 years.
				"-keypass", password.as_str(),
				"-storepass", password.as_str(),
			],
			"keytool not installed!", "keytool failed!");
	}

	// Load Password.
	let password = String::from_utf8(file::load(PASSWORD)).unwrap();

	let unsigned = format!("{}build/outputs/apk/apk-release-unsigned.apk",
		&apk);

	let apk_out = format!("{}build/outputs/apk/todo.apk", &apk);

	// Sign the APK
	program::execute("Jarsigner", "jarsigner",
		vec![
			"-verbose",
			"-tsa", "http://timestamp.digicert.com",
			"-sigalg", "SHA1withRSA",
			"-digestalg", "SHA1",
			"-keystore", &keystore,
			"-storepass", &password,
			&unsigned, &cargo_toml.subdomain
		],
		"jarsigner not installed!", "couldn't sign apk!");

	// Align The APK
	program::execute("Zipalign", zipalign,
		vec![
			"-f",
			"-v",
			"4",
			&unsigned,
			&apk_out,
		],
		"zipalign for 25.0.2 is not installed", "zipalign failed");

	// Install & Run APK on the Phone
	run(android_home, &apk_out);
}

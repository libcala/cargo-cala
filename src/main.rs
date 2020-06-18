#[macro_use]
extern crate serde_derive;

mod apk;
mod run;

/// Start a program.
fn run(prg: &str, args: &[&str]) -> std::process::ExitStatus {
    std::process::Command::new(prg)
        .args(args)
        .status()
        .expect(&format!("Couldn't Start {}", prg))
}

fn apk() {
    println!("Building for Android…");

    use std::env::var;

    // Find SDK & NDK directories if they exist, otherwise create them.
    let home = var("HOME").unwrap();
    let cala = format!("{}/.cargo-cala", home);
    std::fs::create_dir_all(&cala).unwrap();
    let asdk = match var("ANDROID_SDK_ROOT")
        .or(var("ANDROID_HOME"))
        .or(var("ANDROID_SDK"))
    {
        Ok(s) => {
            println!("Found Android SDK (env)!");
            s
        }
        Err(_) => {
            let sdk = format!("{}/android-sdk", &cala);
            if std::path::Path::new(&sdk).exists() == false {
                println!("No Android SDK found!");
                // Create SDK folder.
                std::fs::create_dir_all(&sdk).unwrap();
                // Download the SDK.
                run(
                    "wget",
                    &[
                        "-P",
                        &sdk,
                        "https://github.com/JeronAldaron/cargo-cala/releases/download/android/sdk-linux.zip",
//                        "https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip",
                    ],
                );
                run("touch", &[&format!("{}/sdk-linux.zip", sdk)]);
                // Extract the SDK.
                run("unzip", &[&format!("{}/sdk-linux.zip", sdk), "-d", &format!("{}/", sdk)]);
                // Delete the ZIP file.
                run("rm", &[&format!("{}/sdk-linux.zip", sdk)]);
            } else {
                println!("Found Android SDK (cala)!");
            }
            sdk
        }
    };

    let asat = format!("{}/.cargo-cala/android-ndk", home);
    let andk = match var("ANDROID_NDK_HOME")
        .or(var("NDK_HOME"))
        .or(var("ANROID_NDK"))
    {
        Ok(s) => {
            println!("Found Android NDK (env)!");
            s
        }
        Err(_) => {
            let ndk = asat.clone();
            if std::path::Path::new(&ndk).exists() == false {
                println!("No Android NDK found!");
                // Create NDK folder.
                std::fs::create_dir_all(&ndk).unwrap();
                // Download the SDK.
                run(
                    "wget",
                    &[
                        "-P",
                        &ndk,
                        "https://github.com/JeronAldaron/cargo-cala/releases/download/android/ndk-linux.zip",
                    ],
                );
                // Extract the NDK.
                run("unzip", &[&format!("{}/ndk-linux.zip", ndk), "-d", &ndk]);
                // Move the NDK.
                run(
                    "mv",
                    &[
                        &format!("{}/android-ndk-r19c", ndk),
                        &format!("{}/ndk", ndk),
                    ],
                );
                // Delete the ZIP file.
                run("rm", &[&format!("{}/ndk-linux.zip", ndk)]);
            } else {
                println!("Found Android NDK (cala)!");
            }
            format!("{}/ndk", ndk)
        }
    };

    // Set up target directory…
    println!("Setting up target directory…");
    std::fs::create_dir_all("target/cala/android/lib/arm64-v8a").unwrap();
    std::fs::create_dir_all("target/cala/android/lib/armeabi-v7a").unwrap();
    std::fs::create_dir_all("target/cala/android/lib/x86").unwrap();
    std::fs::create_dir_all("target/cala/android/lib/x86_64").unwrap();
    std::fs::create_dir_all("target/cala/android/res/values").unwrap();

    // Make sure the 4 targeted toolchains are installed.
    run(
        "rustup",
        &[
            "target",
            "add",
            //            "arm-linux-androideabi",
            "aarch64-linux-android",
            //            "armv7-linux-androideabi",
            "i686-linux-android",
            "thumbv7neon-linux-androideabi",
            "x86_64-linux-android",
        ],
    );

    // Make sure the android platform tools are installed.
    if std::path::Path::new(&format!("{}/build-tools/", &asdk)).exists() == false {
        println!("Installing Android platform tools…");
        run(
            &format!("{}/tools/bin/sdkmanager", asdk),
            &[
                "platform-tools",
                "platforms;android-18",
                "build-tools;26.0.1",
            ],
        );
    }

    // Make sure Android toolchains are installed.
    if std::path::Path::new(&format!("{}/arm-linux-androideabi", &asat)).exists() == false {
        run(
            &format!("{}/build/tools/make-standalone-toolchain.sh", &andk),
            &[
                "--toolchain=arm-linux-androideabi",
                &format!("--install-dir={}/arm-linux-androideabi", &asat),
            ],
        );
    }
    if std::path::Path::new(&format!("{}/aarch64-linux-android", &asat)).exists() == false {
        run(
            &format!("{}/build/tools/make-standalone-toolchain.sh", &andk),
            &[
                "--toolchain=aarch64-linux-android",
                &format!("--install-dir={}/aarch64-linux-android", &asat),
            ],
        );
    }
    if std::path::Path::new(&format!("{}/x86-linux-android", &asat)).exists() == false {
        run(
            &format!("{}/build/tools/make-standalone-toolchain.sh", &andk),
            &[
                "--toolchain=x86-linux-android",
                &format!("--install-dir={}/x86-linux-android", &asat),
            ],
        );
    }
    if std::path::Path::new(&format!("{}/x86_64-linux-android", &asat)).exists() == false {
        run(
            &format!("{}/build/tools/make-standalone-toolchain.sh", &andk),
            &[
                "--toolchain=x86_64-linux-android",
                &format!("--install-dir={}/x86_64-linux-android", &asat),
            ],
        );
    }

    // Build Rust code for all 4 targets.
    println!("Building Rust code…");
    // 32-bit ARM
    run(
        "cargo",
        &[
            "rustc",
            "--target",
            "thumbv7neon-linux-androideabi", // "armv7-linux-androideabi" // "arm-linux-androideabi"
            "--release",
            "--",
            "-C",
            &format!(
                "linker={}/arm-linux-androideabi/bin/clang",
                &asat,
            ),
        ],
    );
    // 64-bit ARM
    run(
        "cargo",
        &[
            "rustc",
            "--target",
            "aarch64-linux-android",
            "--release",
            "--",
            "-C",
            &format!(
                "linker={}/aarch64-linux-android/bin/clang",
                &asat,
            ),
        ],
    );
    // 32-bit AMD
    run(
        "cargo",
        &[
            "rustc",
            "--target",
            "i686-linux-android",
            "--release",
            "--",
            "-C",
            &format!("linker={}/x86-linux-android/bin/clang", &asat),
        ],
    );
    // 64-bit AMD
    run(
        "cargo",
        &[
            "rustc",
            "--target",
            "x86_64-linux-android",
            "--release",
            "--",
            "-C",
            &format!("linker={}/x86_64-linux-android/bin/clang", &asat),
        ],
    );
    // Copy into android's lib folder.
}

fn help() {
    println!("USAGE: cargo cala [OPTION]");
    println!();
    println!("[OPTION]:");
    println!("      run      Debug application on this computer");
    println!("      android  Debug application on connected Android device");
    println!("      wasm     Debug application in web browser");
    println!();
    println!("      flatpak  Build FlatPak release package");
    println!("      aab      Build Android release package (*.abb - Android App Bundle)");
    println!("      website  Build static website");
    println!();
    // println!("      dmg      Deploy distribution package For Mac (.dmg) [TODO]");
    // println!("      msi      Deploy distribution installer For Windows (.msi) [TODO]");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 3 {
        if args.len() == 2 {
            if args[1] != "cala" {
                eprintln!("Please use `cargo cala` instead of `cargo-cala`");
            } else {
                help(); // TODO: build for native.
            }
        } else {
            // 1
            eprintln!("Please use `cargo cala` instead of `cargo-cala`");
            help();
        }
    } else {
        match args[2].as_str() {
            "run" => run::run(),
            "android" => apk(), // apk::apk(),
            "wasm" => run::web(),

            "flatpak" => todo!(),
            "aab" => todo!(),
            "website" => todo!(),
            a => {
                eprintln!("Unknown Distribution Format: {}", a);
                help()
            }
        }
    }
}

/// Start a program.
fn run(prg: &str, args: Vec<&str>) -> std::process::ExitStatus {
    std::process::Command::new(prg)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect(&format!("Couldn't Start {}", prg))
        .wait()
        .expect("failed to wait on child")
}

fn apk() {
    let home = std::env::var("HOME").unwrap();
    // 32-bit ARM
    run("cargo",
        vec![
            "rustc",
            "--",
            "-C",
            &format!("linker={}/.cargo-dive/arm-linux-androideabi/bin/clang", home),
            "--target",
            "thumbv7neon-linux-androideabi" // "armv7-linux-androideabi" // "arm-linux-androideabi"
        ]);
    // 64-bit ARM
    run("cargo",
        vec![
            "rustc",
            "--",
            "-C",
            &format!("linker={}/.cargo-dive/aarch64-linux-android/bin/clang", home),
            "--target",
            "aarch64-linux-android"
        ]);
    // 32-bit AMD
    run("cargo",
        vec![
            "rustc",
            "--",
            "-C",
            &format!("linker={}/.cargo-dive/x86-linux-android/bin/clang", home),
            "--target",
            "i686-linux-android"
        ]);
    // 64-bit AMD
    run("cargo",
        vec![
            "rustc",
            "--",
            "-C",
            &format!("linker={}/.cargo-dive/x86_64-linux-android/bin/clang", home),
            "--target",
            "x86_64-linux-android"
        ]);
}

fn help() {
    println!("USAGE:");
    println!("      cargo dive apk      # Deploy distribution package for Android (APK)");
    println!("      cargo dive pak      # Deploy distribution package For Linux (FlatPak) [TODO]");
    println!("      cargo dive app      # Deploy distribution package For Mac (.app) [TODO]");
    println!("      cargo dive msi      # Deploy distribution installer For Windows (.msi) [TODO]");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 3 {
        if args.len() == 2 {
            if args[1] != "dive" {
                eprintln!("Please use `cargo dive` instead of `cargo-dive`");
            } else {
                help(); // TODO: build for native.
            }
        } else { // 1
            eprintln!("Please use `cargo dive` instead of `cargo-dive`");
            help();
        }
    } else {
        match args[2].as_str() {
            "apk" => apk(),
            "pak" => unimplemented!(),
            "app" => unimplemented!(),
            "msi" => unimplemented!(),
            a => {
                eprintln!("Unknown Distribution Format: {}", a);
                help()
            }
        }
    }
}

mod apk;
mod web;

/// Start a program.
fn run(prg: &str, args: &[&str]) -> std::process::ExitStatus {
    std::process::Command::new(prg)
        .args(args)
        .status()
        .expect(&format!("Couldn't Start {}", prg))
}

fn help() {
    println!("USAGE:");
    println!("      cargo cala web      # Deploy distribution for web");
    println!("      cargo cala apk      # Deploy distribution package for Android (APK)");
    println!("      cargo cala pak      # Deploy distribution package For Linux (FlatPak) [TODO]");
    println!("      cargo cala dmg      # Deploy distribution package For Mac (.dmg) [TODO]");
    println!("      cargo cala msi      # Deploy distribution installer For Windows (.msi) [TODO]");
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
            "apk" => apk::apk(),
            "web" => web::web(),
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

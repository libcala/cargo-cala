mod apk;
mod web;
mod run;

/// Start a program.
fn run(prg: &str, args: &[&str]) -> std::process::ExitStatus {
    std::process::Command::new(prg)
        .args(args)
        .status()
        .expect(&format!("Couldn't Start {}", prg))
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
            "android" => apk::apk(),
            "wasm" => web::web(),
            
            "flatpak" => unimplemented!(),
            "aab" => unimplemented!(),
            "website" => unimplemented!(),
            a => {
                eprintln!("Unknown Distribution Format: {}", a);
                help()
            }
        }
    }
}

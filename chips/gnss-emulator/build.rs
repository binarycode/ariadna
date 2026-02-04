use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=chip.json");

    let target = env::var("TARGET").unwrap();
    if !target.contains("wasm32") {
        return;
    }

    let from = env::var("CARGO_MANIFEST_DIR").unwrap();
    let from = Path::new(&from).join("chip.json");

    let to = env::var("OUT_DIR").unwrap();
    let mut to = Path::new(&to);
    loop {
        if to.file_name().unwrap() == "target" {
            let profile = env::var("PROFILE").unwrap();
            let chip_json = format!("{}.json", env::var("CARGO_PKG_NAME").unwrap()).replace("-", "_");

            let to = &to.join(target).join(profile).join(chip_json);

            if let Err(e) = std::fs::copy(&from, to) {
                println!("cargo:warning=Failed to copy chip.json from {from:?} to {to:?}: {e}");
                std::process::exit(1);
            }

            break;
        }

        to = to.parent().unwrap();
    }
}

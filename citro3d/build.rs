use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=examples/assets");

    let mut asset_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    asset_dir.push("examples");
    asset_dir.push("assets");

    println!("Checking dir {:?}", asset_dir.display());

    for entry in asset_dir.read_dir().unwrap().flatten() {
        println!("Checking {:?}", entry.path().display());
        if let Some("pica") = entry.path().extension().and_then(OsStr::to_str) {
            println!("cargo:rerun-if-changed={}", entry.path().display());

            let mut out_path = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
            out_path.push("examples");
            out_path.push("assets");
            out_path.push(entry.path().with_extension("shbin").file_name().unwrap());

            std::fs::create_dir_all(out_path.parent().unwrap()).unwrap();

            println!("Compiling {:?}", out_path.display());

            let mut cmd = Command::new("picasso");
            cmd.arg(entry.path()).arg("--out").arg(out_path);

            let status = cmd.spawn().unwrap().wait().unwrap();
            assert!(
                status.success(),
                "Command {cmd:#?} failed with code {status:?}"
            );
        }
    }
}

use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::fs;

const REBUILD_BLOCKLIST: &[&str] = &[".git", "build"];

fn visit_dir(dir: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if !REBUILD_BLOCKLIST.contains(&path.file_name().unwrap().to_str().unwrap()) {
                visit_dir(&path)?;
            }
        } else {
            println!("cargo:rerun-if-changed={}", fs::canonicalize(path)?.display());
        }
    }
    Ok(())
}

fn main() {
    let dst = cmake::build("stefan-code");

    // Tell cargo to tell rustc where to find libstefan
    println!(
            "cargo:rustc-link-search={}", dst.display()
            );
    // Tell cargo to tell rustc to link the static libstefan
    println!("cargo:rustc-link-lib=static=stefan");
    // Also link the c++ standard library under Linux
    if cfg!(unix) {
        println!("cargo:rustc-link-lib=stdc++");
    }

    // Tell cargo to invalidate the built crate whenever the C++ code changed
    visit_dir(Path::new("./stefan-code/")).unwrap();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("stefan-code/include/stefan.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

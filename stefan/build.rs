use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc where to find the stuff
    println!(
        "cargo:rustc-link-search={}/stefan-code/build/",
        env::current_dir().unwrap().to_str().unwrap()
    );
    // Tell cargo to tell rustc to link the static libstefan
    println!("cargo:rustc-link-lib=stefan");
    // Also link the c++ standard library
    println!("cargo:rustc-link-lib=stdc++");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=stefan-code/include/stefan.h");
    println!("cargo:rerun-if-changed=stefan-code/build/libstefan.a");

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
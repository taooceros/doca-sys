use std::{env, path::PathBuf};
extern crate pkg_config;

fn main() {
    let doca = pkg_config::Config::new()
        .atleast_version("2.7")
        .probe("doca")
        .unwrap();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_args(
            doca.include_paths
                .iter()
                .map(|p| format!("-I{}", p.to_string_lossy())),
        )
        .clang_arg("-DDOCA_ALLOW_EXPERIMENTAL_API")
        .header("include/wrapper.h")
        // Tell cargo to invalidate the built ucx_sys whenever any of the
        // included header files changed.
        .prepend_enum_name(false)
        .bitfield_enum("doca_.*_flag")
        .rustified_enum("doca_.*")
        .bitfield_enum("doca_.*_flag")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // let cargo knows if wrapper.h is changed
    println!("cargo:rerun-if-changed=include/wrapper.h");


    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

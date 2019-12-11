extern crate bindgen;
extern crate pkg_config;

use std::env;
use pkg_config::Library;

fn main() {
    if cfg!(target_os = "macos") {
        return;
    }

    let out_file = env::current_dir().unwrap().join("src").join("bindings.rs");

    // Skip building if bindings already exist.
    // If you want to rebuild, delete the bindings file.
    if out_file.exists() {
        return;
    }


    let lib = pkg_config::Config::new()
        .atleast_version("0.8.1")
        .probe("libzfs")
        .unwrap();
    println!("cargo:rustc-link-lib=zfs");
    println!("cargo:version={}", lib.version);

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(clang_include_paths(&lib))
        .clang_args(clang_library_paths(&lib))
        .whitelist_function("dmu.*")
        .whitelist_type("dmu.*")
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to src.
    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}

fn clang_include_paths(library: &Library) -> Vec<String> {
    library
        .include_paths
        .iter()
        .map(|p| format!("-I{}", p.display()))
        .collect()
}

fn clang_library_paths(library: &Library) -> Vec<String> {
    library
        .link_paths
        .iter()
        .map(|p| format!("-L{}", p.display()))
        .collect()
}



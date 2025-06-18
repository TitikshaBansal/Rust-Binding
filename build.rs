// build.rs
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=cpdb");
    println!("cargo:rustc-link-lib=cpdb-frontend");

    let cpdb = pkg_config::Config::new()
        .atleast_version("2.0")
        .probe("cpdb")
        .expect("Failed to find cpdb using pkg-config");

    // Tell cargo to tell rustc to link the cpdb library
    for lib in cpdb.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    // Tell cargo to tell rustc to link the cpdb library search path
    for path in cpdb.link_paths {
        println!("cargo:rustc-link-search=native={}", path.display());
    }
    // Generate bindings for the C API
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(cpdb.include_paths.iter().map(|p| format!("-I{}", p.display())))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("cpdb_.*")
        .allowlist_type("cpdb_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
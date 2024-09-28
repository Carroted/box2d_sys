use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=box2d");
    println!("cargo:rerun-if-changed=box2d/");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    let box2d_path = build_box2d();

    let box2d_include_path = box2d_path.join(std::path::PathBuf::from("include"));
    //println!("cargo:rustc-link-lib=static=box2");
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg(format!("-I{}", box2d_include_path.display()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("__mingw_ldbl_type_t")
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

fn build_box2d() -> PathBuf {
    let mut box2d_config = cmake::Config::new("box2d");

    let mut box2d_config = box2d_config
        .define("BOX2D_BUILD_UNIT_TESTS", "OFF")
        .define("BOX2D_UNIT_TESTS", "OFF")
        .define("BOX2D_BUILD_TESTBED", "OFF")
        .define("BOX2D_BUILD_DOCS", "OFF")
        .define("BOX2D_USER_SETTINGS", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BOX2D_SAMPLES", "OFF")
        .define("BOX2D_BENCHMARKS", "OFF")
        .define("BOX2D_DOCS", "OFF")
        .define("BOX2D_PROFILE", "OFF")
        .define("BOX2D_VALIDATE", "ON")
        .define("ENKITS_BUILD_EXAMPLES", "OFF")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("CMAKE_INSTALL_BINDIR", "bin")
        .define("CMAKE_INSTALL_INCLUDEDIR", "include")
        .define("CMAKE_BUILD_TYPE", "Release");

    #[cfg(feature = "no_avx2")]
    {
        // for compatibility we can do this, itll make it slower tho so its not default
        box2d_config = box2d_config.define("BOX2D_AVX2", "OFF");
    }

    // the rest are fine
    let box2d_path = box2d_config.build();

    println!(
        "cargo:rustc-link-search=native={}/lib",
        box2d_path.display()
    );

    println!("cargo:include={}/include", box2d_path.display());

    return box2d_path;
}

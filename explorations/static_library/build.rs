use std::env;
use std::path::PathBuf;


fn build_vex_binding() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/vex_headers/wrapper.hpp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        // .header_contents("chrono", chrono_contents())
        .header("src/vex_headers/wrapper.hpp") // minimal vex headers to import to get everything to wrok
        .clang_arg("-std=c++11") // use the right c++ version
        .layout_tests(false) // we don't want tests because they rely on std library
        .use_core()
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

fn main() {
    build_vex_binding();
    // let sdk_loc = "/home/richie/.config/Code/User/globalStorage/vexrobotics.vexcode/sdk/";

}

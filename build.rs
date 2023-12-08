const VERSION: &str = "V5_20220726_10_00_00";
const SDK_ZIP_PATH: &str = "sdk.zip";

use curl::easy::Easy;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::{env, vec};
use zip::read::ZipArchive;

fn make_vex_bindings(sdk_path: &String, out_dir: &String) {
    let header_path = |fname: &str| format!("{}/include/{}", sdk_path, fname);
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg("-std=c++11") // use the right c++ version
        .clang_arg("-xc++") // despite the fact that these say .h, theyre really cpp files
        .clang_arg("-DNULL=nullptr")
        .layout_tests(false) // we don't want tests because they rely on std library
        .use_core() // since we're no_std we need definitions for ffi types
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .header(header_path("v5.h"))
        .header(header_path("vex_mevent.h"))
        .header(header_path("vex_units.h"))
        .header(header_path("vex_color.h"))
        .header(header_path("vex_timer.h"))
        .header(header_path("vex_device.h"))
        // Do the silly with vex_brain. bindgen does not like it's templates
        .header_contents(
            header_path("vex_brain.h").as_str(),
            remove_unbindables(sdk_path).as_str(),
        )
        .header(header_path("vex_competition.h"))
        // Finish the builder and generate the bindings.
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(out_dir);
    println!(
        "cargo:warning=Writing bindings to {}",
        out_path.join("bindings.rs").to_str().unwrap()
    );
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

// bindgen really does not like the templates in vex_brain.h
fn remove_unbindables(sdk_path: &String) -> String {
    let ranges: Vec<(usize, usize)> = vec![
        (131, 134),
        (155, 160),
        (195, 209),
        (238, 241),
        (266, 278),
        (334, 346),
        (391, 403),
    ];
    let in_range = |i, &(lo, hi)| i >= lo && i <= hi;

    let path = format!("{}/include/vex_brain.h", sdk_path);
    let str = fs::read_to_string(&path).expect("Couldn't open vex_brain.h to fix template stuff");

    let oglines: Vec<&str> = str.lines().collect();
    let mut src_wout: String = "".into();

    for line_num in 1..=oglines.len() {
        let mut still_in = true;
        for range in &ranges {
            if in_range(line_num, range) {
                still_in = false;
            }
        }
        if still_in {
            src_wout += (oglines[line_num - 1].to_string() + "\n").as_str();
        }
    }

    return src_wout;
}

#[derive(Clone, Copy)]

fn find_sdk_path() -> Option<String> {
    let windows_sdk_path: String = "C:/Program Files (x86)/VEX Robotics/VEXcode V5/sdk".into();
    let linux_sdk_path: String =
        "/.config/Code/User/globalStorage/vexrobotics.vexcode/sdk/cpp/V5/V5_20220726_10_00_00/"
            .into();
    if cfg!(windows) {
        Some(windows_sdk_path)
    } else {
        Some(format!(
            "{}/{}",
            env::var("HOME").expect("Need HOME env variable for linux"),
            linux_sdk_path
        ))
    }
}

fn main() {
    let out_dir: String =
        env::var("OUT_DIR").expect("Need OUT_DIR defined to know where to build this stuff to");

    let bindings_path = format!("{}/bindings.rs", out_dir);

    let sdk_path = find_sdk_path().expect("Couldn't find the sdk path");

    // have to rebuild
    if !Path::new(bindings_path.as_str()).exists() {
        make_vex_bindings(&sdk_path, &out_dir);
    }

    println!("cargo:rustc-link-search={}/vexv5", sdk_path);
    println!("cargo:rustc-link-search={}/vexv5/gcc/libs", sdk_path);
    println!("cargo:rustc-link-arg=-R{}/vexv5/stdlib_0.lib", sdk_path);

    println!("cargo:rustc-link-lib=static=v5rt");
    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=m");
    println!("cargo:rustc-link-lib=static=gcc");
}

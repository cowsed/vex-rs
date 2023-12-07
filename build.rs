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
        .header_contents(
            "extra",
            r#"
        typedef unsigned int size_t; // found by static asserting in real vex project
            void free(void*);
            void *malloc(size_t size);
        "#,
        )
        // .header("extra")
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

fn download_sdk(out_dir: &String) {
    let full_zip_path = format!("{}/{}", out_dir, SDK_ZIP_PATH);
    let full_zip_path = full_zip_path.as_str();

    let mut dst = Vec::new();
    let mut easy = Easy::new();
    easy.url(
        format!(
            "https://content.vexrobotics.com/vexos/public/V5/vscode/sdk/cpp/{}.zip",
            VERSION
        )
        .as_str(),
    )
    .expect("Couldn't download sdk zip");
    let _redirect = easy.follow_location(true);

    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    {
        let mut file = File::create(full_zip_path).expect("Downloaded sdk not there");
        file.write_all(dst.as_slice())
            .expect(format!("Couldnt write file to {}", full_zip_path).as_str());
    }
}

fn unzip_sdk(out_dir: &String) {
    let full_zip_path = format!("{}/{}", out_dir, SDK_ZIP_PATH);

    let f = File::open(full_zip_path).expect("Couldn't open sdk zip path");
    let reader = BufReader::new(f);
    ZipArchive::new(reader)
        .expect("Failed to decode sdk zip")
        .extract(out_dir)
        .expect("Couldn't decode sdk zip");
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
enum PathFindError {
    NoBuiltinPath,
}

fn find_sdk_path(out_dir: &String) -> Result<String, PathFindError> {
    // If we have them from a previous install
    let target_path = format!("{}/{}/vexv5", out_dir, VERSION);
    if Path::new(target_path.as_str()).exists() {
        return Ok(target_path);
    }

    return Err(PathFindError::NoBuiltinPath);
}

fn main() {
    let out_dir: String =
        env::var("OUT_DIR").expect("Need OUT_DIR defined to know where to build this stuff to");

    let bindings_path = format!("{}/bindings.rs", out_dir);

    // have to rebuild
    let builtin_path = find_sdk_path(&out_dir);
    let mut sdk_path: String = "".into();
    if let Ok(builtin_p) = builtin_path {
        sdk_path = builtin_p;
    } else {
        // need to download sdk
        println!("cargo:warning=Couldn't find system headers. Specify in the VEX_SDK_PATH environment variable to use those headers. Downloading them now");

        download_sdk(&out_dir);
        unzip_sdk(&out_dir);
        sdk_path += out_dir.as_str();
        sdk_path += "/";
        sdk_path += VERSION;
        sdk_path += "/vexv5";
    }
    if !Path::new(bindings_path.as_str()).exists() {
        make_vex_bindings(&sdk_path, &out_dir);
    }
    println!("cargo:rustc-link-search={}", sdk_path);
    println!("cargo:rustc-link-search={}/gcc/libs", sdk_path);
    println!("cargo:rustc-link-arg=-R{}/vexv5/stdlib_0.lib", sdk_path);

    println!("cargo:rustc-link-lib=static=v5rt");
    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=m");
    println!("cargo:rustc-link-lib=static=gcc");
}

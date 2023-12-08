fn find_sdk_path() -> String {
    let windows_sdk_path: String = "C:/Program Files (x86)/VEX Robotics/VEXcode V5/sdk".into();
    let linux_sdk_path: String = "/.config/Code/User/globalStorage/vexrobotics.vexcode/sdk/cpp/V5/V5_20220726_10_00_00/".into();
    if cfg!(windows) {
        windows_sdk_path
    } else {
        format!("{}/{}", env!("HOME"), linux_sdk_path)
    }
}


fn main() {
    let lscript_arg = format!(
        "-Wl,-T{}/vexv5/lscript.ld",
        find_sdk_path()
    );
    let stdlib_arg = format!(
        "-Wl,-R{}/vexv5/stdlib_0.lib",
        find_sdk_path()
    );

    println!("cargo:rustc-link-arg={}", lscript_arg);
    println!("cargo:rustc-link-arg={}", stdlib_arg);
}


const TARGET_FILE:&str = r#"
{
    "arch": "arm",
    "data-layout": "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64",
    "disable-redzone": true,
    "emit-debug-gdb-scripts": false,
    "env": "newlib",
    "executables": true,
    "features": "+v7,+thumb2,+soft-float,-neon,+strict-align",
    "linker": "arm-none-eabi-gcc",
    "post-link-args": {
      "gcc": [
        "-nostdlib",
        "-nodefaultlibs",
        "-Wl,--gc-sections",
        "-Wl,-T/home/richie/.config/Code/User/globalStorage/vexrobotics.vexcode/sdk/cpp/V5/V5_20220726_10_00_00/vexv5/lscript.ld",
        "-Wl,-R\"/home/richie/.config/Code/User/globalStorage/vexrobotics.vexcode/sdk/cpp/V5/V5_20220726_10_00_00/vexv5/stdlib_0.lib\""
      ]
    },
    "llvm-target": "armv7a-none-eabi",
    "max-atomic-width": 64,
    "os": "none",
    "panic-strategy": "abort",
    "relocation-model": "static",
    "target-c-int-width": "32",
    "target-family": "unix",
    "target-endian": "little",
    "target-pointer-width": "32",
    "vendor": "vex"
  }
"#;

const HALF_FINSHED = r#"
{
    "arch": "arm",
    "data-layout": "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64",
    "disable-redzone": true,
    "emit-debug-gdb-scripts": false,
    "env": "newlib",
    "executables": true,
    "features": "+v7,+thumb2,+soft-float,-neon,+strict-align",
    "linker": "arm-none-eabi-gcc",
    "post-link-args": {
      "gcc": [
        "-nostdlib",
        "-nodefaultlibs", 
        "-Wl,--gc-sections"
      ]
    },
    "llvm-target": "armv7a-none-eabi",
    "max-atomic-width": 64,
    "os": "none",
    "panic-strategy": "abort",
    "relocation-model": "static",
    "target-c-int-width": "32",
    "target-family": "unix",
    "target-endian": "little",
    "target-pointer-width": "32",
    "vendor": "vex"
  }"#;

use std::{fs::File, io::Write};
fn main(){
    let mut file = File::create("build/armv7a-vex-eabi.json").expect("msg");
    // file.write_all(target_file.)?;
    file.write(target_file.as_bytes()).expect("asdsadas");
}
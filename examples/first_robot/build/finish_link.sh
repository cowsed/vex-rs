name=$(basename $(pwd))
arm-none-eabi-ld target/armv7a-vex-eabi/release/libfirst_robot.a -T$HOME/.config/Code/User/globalStorage/vexrobotics.vexcode/sdk/cpp/V5/V5_20220726_10_00_00/vexv5/lscript.ld -o target/${name}.elf
arm-none-eabi-objcopy -O binary target/${name}.elf target/${name}.bin

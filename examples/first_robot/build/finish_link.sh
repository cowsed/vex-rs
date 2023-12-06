echo "RUNNING FINISH LINK.SH"
name=$(basename $(pwd))
arm-none-eabi-ld target/armv7a-vex-eabi/debug/libfirst_robot.a -T/home/richie/.config/Code/User/globalStorage/vexrobotics.vexcode/sdk/cpp/V5/V5_20220726_10_00_00/vexv5/lscript.ld -o target/${name}.elf
arm-none-eabi-objcopy -O binary target/${name}.elf target/${name}.bin

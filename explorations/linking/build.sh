rm objs/*.o
cp ../static_library/target/armv7a-vex-eabi/release/libstatic_library.a ./
ar x libstatic_library.a
mv *.o objs
arm-none-eabi-ld -nostdlib -T "/home/richie/.config/Code/User/globalStorage/vexrobotics.vexcode/sdk/cpp/V5/V5_20220726_10_00_00/vexv5/lscript.ld" -R "/home/richie/.config/Code/User/globalStorage/vexrobotics.vexcode/sdk/cpp/V5/V5_20220726_10_00_00/vexv5/stdlib_0.lib" --gc-section -L"/home/richie/.config/Code/User/globalStorage/vexrobotics.vexcode/sdk/cpp/V5/V5_20220726_10_00_00/vexv5" -L"/home/richie/.config/Code/User/globalStorage/vexrobotics.vexcode/sdk/cpp/V5/V5_20220726_10_00_00/vexv5/gcc/libs" -o RustRelease.elf  objs/*.o --start-group -lv5rt -lstdc++ -lc -lm -lgcc  --end-group
arm-none-eabi-objcopy -O binary RustRelease.elf  RustRelease.bin

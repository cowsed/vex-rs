#!/usr/bin/env bash

# output of rust compiler
elf_name=$1
# binary file we're flashing to the brain
bin_name="${elf_name%.*}.bin"

# create binary file
arm-none-eabi-objcopy -O binary ${elf_name} ${bin_name} 

# Show size
du -h ${bin_name}

# flash the guy
$HOME/.vscode/extensions/vexrobotics.vexcode-0.5.0/resources/tools/vexcom/linux-x64/vexcom --slot 1 --write ${bin_name}
#!/usr/bin/env bash

# output of rust compiler
elf_path=$1
# binary file we're flashing to the brain
bin_path="${elf_path%.*}.bin"
bin_name=$(basename ${bin_path})

# create binary file
arm-none-eabi-objcopy -O binary ${elf_path} ${bin_path} 

size=$(du -h ${bin_path} | cut -f1)
echo -e "\n========    Flash - ${bin_name} - ${size}    =======\n"

# flash the guy
$HOME/.vscode/extensions/vexrobotics.vexcode-0.5.0/resources/tools/vexcom/linux-x64/vexcom --slot 1 --write ${bin_path}
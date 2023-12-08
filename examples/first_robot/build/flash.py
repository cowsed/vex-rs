import os
import subprocess
import sys

def format_bytes(size):
    # Define the size units
    units = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"]
    for unit in units:
        if size < 1024:
            return f"{size:.2f} {unit}"
        size /= 1024

def main():
    # Check if an argument is provided
    if len(sys.argv) < 2:
        print("Usage: script.py <elf_path>")
        sys.exit(1)

    # output of rust compiler
    elf_path = sys.argv[1]
    # binary file we're flashing to the brain
    bin_path = f"{os.path.splitext(elf_path)[0]}.bin"
    bin_name = os.path.basename(bin_path)

    # create binary file
    subprocess.run(["arm-none-eabi-objcopy", "-O", "binary", elf_path, bin_path])

    # get file size of bin_path
    size = os.path.getsize(bin_path)

    print(f"\n========    Flash - {bin_name} - {format_bytes(size)}    ========\n")

    # flash the guy
    vexcom_path = f"C:/Users/{os.getlogin()}/.vscode/extensions/vexrobotics.vexcode-0.5.0/resources/tools/vexcom/win32/vexcom.exe"
    if os.name != "nt": # if not windows
        vexcom_path = os.path.join(os.environ['HOME'], ".vscode/extensions/vexrobotics.vexcode-0.5.0/resources/tools/vexcom/linux-x64/vexcom")
    subprocess.run([vexcom_path, "--slot", "1", "--write", bin_path, "--progress", "--timer"])

if __name__ == "__main__":
    main()

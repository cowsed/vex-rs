name=$(basename $(pwd))
$HOME/.vscode/extensions/vexrobotics.vexcode-0.5.0/resources/tools/vexcom/linux-x64/vexcom --slot $1 --write target/${name}.bin
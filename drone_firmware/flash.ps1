$ErrorActionPreference = "Stop"

cargo build --release

$projectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$buildElf    = Join-Path $projectRoot "target\thumbv7em-none-eabihf\release\drone_firmware"
$flashElf    = Join-Path $projectRoot "target\thumbv7em-none-eabihf\release\drone_firmware.elf"
$cubeProg    = "C:\Program Files\STMicroelectronics\STM32Cube\STM32CubeProgrammer\bin\STM32_Programmer_CLI.exe"

if (-not (Test-Path $buildElf)) {
    throw "Built ELF not found at: $buildElf"
}

if (Test-Path $flashElf) {
    Remove-Item $flashElf -Force
}

Copy-Item $buildElf $flashElf -Force

& $cubeProg `
  --connect port=usb1 `
  --download $flashElf `
  --start 0x08000000

if ($LASTEXITCODE -ne 0) {
    throw "Flashing failed with exit code $LASTEXITCODE"
}
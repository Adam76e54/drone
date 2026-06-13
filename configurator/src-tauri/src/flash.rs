use std::path::{Path, PathBuf}; 
use std::process::Command;
use serde::{Deserialize};
use std::fs;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionMode {
    Usb1, 
    Usb2,
    // we could add more later or in a different project (trying to keep things somewhat reusable even though this is a once-off project)
}

impl ConnectionMode {
    /// returns the correct argument to give STM32CubeProgrammer CLI's --connect arg
    pub fn as_stm32_cube_cli_arg(&self) -> &'static str {
        // NOTE: 'static str is basically like a C string literal. Lives in the binary, very cheap
        match self {
            ConnectionMode::Usb1 => "port=usb1",
            ConnectionMode::Usb2 => "port=usb2",
        }
    }
}

/// Uses stm32CubeProgrammer.exe to flash a firmware file given:
/// 1. the path to the firmware (cargo does add .elf so we add it ourselves)
/// 2. the connected port
/// 3. the path to stm32CubeProgrammer on the machine
#[tauri::command]
pub async fn flash_firmware(raw_firmware: PathBuf, port: ConnectionMode, stm32_cube_path: PathBuf) -> Result<String, String> {
    if !stm32_cube_path.is_file() {
        return Err(format!("STM32 cube programmer executable was not found at: {stm32_cube_path:?}"));
    }

    if !raw_firmware.is_file() {
        return Err(format!("Firmware file was not found at: {raw_firmware:?}"))
    }

    let firmware = attach_elf(&raw_firmware)?;
    // This is just copying what I've done in flash.ps1 but it's fragile
    let output = Command::new(stm32_cube_path)
        .arg("--connect")
        .arg(port.as_stm32_cube_cli_arg())
        .arg("--download")
        .arg(&firmware)
        .output()
        .map_err(|error| format!("STM32 cube programmer failed to complete flash: {error}"))?;

    // If the child process returned success, send stdout back to the UI.
    if output.status.success() {
        let stdout_text = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout_text)
    } else {
        // If flashing failed, stderr usually contains the useful explanation.
        let stderr_text = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Flashing failed:\n{stderr_text}"))
    }
}

fn attach_elf(raw_file: &Path) -> Result<PathBuf, String> {
    if !raw_file.is_file() {
        return Err(format!("Firmware artifact was not found at: {raw_file:?}"));
    }

    let elf_file = raw_file.with_extension("elf");

    if !elf_file.exists() {
        if !elf_file.is_file() {
            return Err(format!("Expected .elf file is not a file: {elf_file:?}"));
        }
        return Ok(elf_file);
    }

    fs::copy(raw_file, &elf_file)
        .map_err(|e| format!("Failed to create file with .elf artifact: {e}"))?;

    Ok(elf_file)
}


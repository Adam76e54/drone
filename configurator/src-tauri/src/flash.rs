#[tauri::command]
pub async fn flash_firmware(port: String, path: String) -> Result<String, String> {
    println!("placeholder flash started with parameters '{port}' and '{path}'");
    Ok(format!(
      "placeholder flash started with parameters '{port}' and '{path}'"
    ))
}



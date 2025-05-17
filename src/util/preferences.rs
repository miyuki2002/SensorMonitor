use anyhow::Result;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

const DEFAULT_ESP32_URL: &str = "localhost";
const PREFERENCES_FILE: &str = "sensor_monitor_preferences.json";

// Lấy đường dẫn đến tệp cài đặt
fn get_preferences_path() -> PathBuf {
    let mut path = match std::env::var("ANDROID_DATA") {
        Ok(data_dir) => PathBuf::from(format!("{}/data/com.example.sensormonitor/files", data_dir)),
        Err(_) => {
            if let Some(dir) = dirs_next::data_local_dir() {
                dir.join("sensor_monitor")
            } else {
                PathBuf::from(".")
            }
        }
    };
    
    // Tạo thư mục nếu chưa tồn tại
    if !path.exists() {
        let _ = fs::create_dir_all(&path);
    }
    
    path.join(PREFERENCES_FILE)
}

pub fn load_preferences() -> Result<serde_json::Value> {
    let path = get_preferences_path();
    
    if !path.exists() {
        return Ok(serde_json::json!({
            "esp32_url": DEFAULT_ESP32_URL,
        }));
    }
    
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let json: serde_json::Value = serde_json::from_str(&contents)?;
    Ok(json)
}

pub fn save_preferences(prefs: &serde_json::Value) -> Result<()> {
    let path = get_preferences_path();
    let json_string = serde_json::to_string_pretty(prefs)?;
    
    let mut file = fs::File::create(path)?;
    file.write_all(json_string.as_bytes())?;
    
    Ok(())
}

// Lấy URL ESP32
pub fn load_esp32_url() -> Result<String> {
    let prefs = load_preferences()?;
    
    let url = match &prefs["esp32_url"] {
        serde_json::Value::String(s) => s.clone(),
        _ => DEFAULT_ESP32_URL.to_string(),
    };
    
    Ok(url)
}

// Lưu URL ESP32
pub fn save_esp32_url(url: &str) -> Result<()> {
    let mut prefs = load_preferences()?;
    prefs["esp32_url"] = serde_json::Value::String(url.to_string());
    save_preferences(&prefs)
}

pub fn get_boolean(key: &str, default_value: bool) -> Result<bool> {
    let prefs = load_preferences()?;
    
    let value = match &prefs[key] {
        serde_json::Value::Bool(b) => *b,
        _ => default_value,
    };
    
    Ok(value)
}

pub fn set_boolean(key: &str, value: bool) -> Result<()> {
    let mut prefs = load_preferences()?;
    prefs[key] = serde_json::Value::Bool(value);
    save_preferences(&prefs)
} 
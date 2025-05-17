use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::Value;
use tokio::runtime::Runtime;
use crate::model::sensor_data::ESP32SensorData;

pub fn fetch_data_from_esp32(url: &str) -> Result<String> {
    // Create a new tokio runtime for async calls
    let rt = Runtime::new()?;
    
    // Execute the async function in the runtime
    rt.block_on(async {
        let client = Client::new();
        let response = client.get(url)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch data from ESP32: {}", e))?;
        
        let status = response.status();
        if !status.is_success() {
            return Err(anyhow!("ESP32 API returned error status: {}", status));
        }
        
        let json = response.text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;
        
        Ok(json)
    })
}

pub fn parse_esp32_data(json: &str) -> Result<ESP32SensorData> {
    let data: ESP32SensorData = serde_json::from_str(json)?;
    Ok(data)
} 
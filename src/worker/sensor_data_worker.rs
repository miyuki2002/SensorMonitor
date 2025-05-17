use anyhow::Result;
use crate::api::{esp32_api, firebase_api};
use crate::repository::sensor_repository;
use std::time::Duration;
use tokio::time;

pub async fn start_worker_loop(esp32_url: String) {
    log::info!("Starting sensor data worker loop");
    
    loop {
        if let Err(e) = fetch_and_process_data(&esp32_url).await {
            log::error!("Error in worker: {}", e);
        }
        
        // Wait 15 minutes before next update
        time::sleep(Duration::from_secs(15 * 60)).await;
    }
}

/// Fetch data from ESP32 and process it
async fn fetch_and_process_data(url: &str) -> Result<()> {
    log::info!("Fetching data from ESP32: {}", url);
    
    // Fetch data from ESP32
    let json = esp32_api::fetch_data_from_esp32(url)?;
    
    // Parse data
    let data = esp32_api::parse_esp32_data(&json)?;
    
    // Process and save data
    sensor_repository::process_esp32_data(&data)?;
    
    log::info!("Successfully processed sensor data");
    
    Ok(())
} 
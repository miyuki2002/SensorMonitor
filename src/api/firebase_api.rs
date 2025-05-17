use anyhow::{Result, anyhow};
use firebase_rs::*;
use serde_json::{Value, json};
use tokio::runtime::Runtime;
use crate::model::sensor_data::{ESP32SensorData, SensorReading};
use std::collections::HashMap;

const FIREBASE_URL: &str = "https://your-project-id.firebaseio.com";
const DATABASE_PATH: &str = "sensor_readings";

pub fn initialize() -> Result<Firebase> {
    let firebase = Firebase::new(FIREBASE_URL)?;
    Ok(firebase)
}

pub fn fetch_latest_readings() -> Result<Vec<ESP32SensorData>> {
    let firebase = initialize()?;
    let db_path = firebase.at(DATABASE_PATH);
    
    // Create a new tokio runtime for async calls
    let rt = Runtime::new()?;
    
    // Execute the async function in the runtime
    rt.block_on(async {
        // Fetch only the last 5 readings ordered by timestamp
        let result = db_path
            .order_by("timestamp")
            .limit_to_last(5)
            .get::<HashMap<String, ESP32SensorData>>()
            .await
            .map_err(|e| anyhow!("Failed to fetch data from Firebase: {}", e))?;
        
        // Convert the HashMap to a Vec
        let mut readings: Vec<ESP32SensorData> = result.values().cloned().collect();
        
        // Sort by timestamp in descending order
        readings.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        Ok(readings)
    })
}

pub fn push_sensor_reading(reading: &SensorReading) -> Result<()> {
    let firebase = initialize()?;
    let db_path = firebase.at(DATABASE_PATH);
    
    // Create a new tokio runtime for async calls
    let rt = Runtime::new()?;
    
    // Execute the async function in the runtime
    rt.block_on(async {
        let json_data = json!({
            "sensor_type": reading.sensor_type,
            "value": reading.value,
            "timestamp": reading.timestamp,
            "is_alert": reading.is_alert
        });
        
        db_path
            .push(&json_data)
            .await
            .map_err(|e| anyhow!("Failed to push data to Firebase: {}", e))?;
        
        Ok(())
    })
}

pub fn setup_realtime_updates<F>(callback: F) -> Result<()>
where
    F: Fn(ESP32SensorData) + Send + 'static,
{
    // This is a placeholder for setting up Firebase realtime updates
    // Firebase's Rust SDK doesn't have direct support for realtime updates
    // In a real implementation, you might use a different approach or library
    
    Ok(())
} 
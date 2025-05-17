use anyhow::{Result, anyhow};
use serde_json::json;
use std::collections::HashMap;
use crate::api::{esp32_api, firebase_api};
use crate::data::dao::{sensor_reading_dao, sensor_threshold_dao};
use crate::model::sensor_data::{ESP32SensorData, SensorReading, SensorThreshold};
use crate::model::sensor_types;

/// Fetch latest readings from database, Firebase, and ESP32
pub fn fetch_latest_readings() -> Result<String> {
    // First, try to fetch from database
    let db_readings = sensor_reading_dao::get_latest_readings()?;
    
    // Convert to a map by sensor type
    let mut readings_map: HashMap<String, SensorReading> = db_readings
        .into_iter()
        .map(|r| (r.sensor_type.clone(), r))
        .collect();
    
    // Try to fetch from Firebase as well
    match firebase_api::fetch_latest_readings() {
        Ok(firebase_readings) => {
            if let Some(latest) = firebase_readings.first() {
                // Process the latest reading from Firebase
                process_esp32_data(latest)?;
                
                // Update the map with new readings
                for reading in sensor_reading_dao::get_latest_readings()? {
                    readings_map.insert(reading.sensor_type.clone(), reading);
                }
            }
        }
        Err(e) => {
            log::warn!("Failed to fetch from Firebase: {}", e);
        }
    }
    
    // Convert to JSON
    let json = serde_json::to_string(&readings_map)?;
    Ok(json)
}

/// Fetch reading history for a specific sensor type
pub fn fetch_reading_history(sensor_type: &str, limit: i64) -> Result<String> {
    let readings = sensor_reading_dao::get_history_by_type(sensor_type, limit)?;
    let json = serde_json::to_string(&readings)?;
    Ok(json)
}

/// Process ESP32 sensor data and save to database
pub fn process_esp32_data(data: &ESP32SensorData) -> Result<()> {
    // Convert ESP32 data to sensor readings
    let readings = SensorReading::from_esp32_data(data);
    
    // Check thresholds and set alerts
    let readings_with_alerts = check_thresholds(readings)?;
    
    // Save to database
    sensor_reading_dao::insert_batch(&readings_with_alerts)?;
    
    Ok(())
}

/// Check sensor thresholds and set alerts
fn check_thresholds(readings: Vec<SensorReading>) -> Result<Vec<SensorReading>> {
    let mut result = Vec::new();
    
    for mut reading in readings {
        let threshold = sensor_threshold_dao::get_threshold(&reading.sensor_type)?;
        
        // Check if value is outside threshold
        let is_alert = reading.value < threshold.min_value || reading.value > threshold.max_value;
        reading.is_alert = is_alert;
        
        result.push(reading);
    }
    
    Ok(result)
}

/// Get threshold for a sensor type
pub fn get_threshold(sensor_type: &str) -> Result<f32> {
    let threshold = sensor_threshold_dao::get_threshold(sensor_type)?;
    Ok(threshold.max_value) // For simplicity, just return max value
}

/// Set threshold for a sensor type
pub fn set_threshold(sensor_type: &str, value: f32) -> Result<()> {
    let mut threshold = sensor_threshold_dao::get_threshold(sensor_type)?;
    threshold.max_value = value;
    sensor_threshold_dao::set_threshold(&threshold)?;
    Ok(())
} 
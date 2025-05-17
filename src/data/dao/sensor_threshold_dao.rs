use anyhow::{Result, anyhow};
use rusqlite::{Connection, params, Error as SqlError};
use crate::data::get_database;
use crate::model::sensor_data::SensorThreshold;
use crate::model::sensor_types;

pub fn get_threshold(sensor_type: &str) -> Result<SensorThreshold> {
    let db = get_database().ok_or_else(|| anyhow!("Database not initialized"))?;
    let conn = db.lock().map_err(|_| anyhow!("Failed to lock database"))?;
    
    let mut stmt = conn.prepare(
        "SELECT sensor_type, min_value, max_value 
         FROM sensor_thresholds 
         WHERE sensor_type = ?"
    )?;
    
    let result = stmt.query_row(params![sensor_type], |row| {
        Ok(SensorThreshold {
            sensor_type: row.get(0)?,
            min_value: row.get(1)?,
            max_value: row.get(2)?,
        })
    });
    
    match result {
        Ok(threshold) => Ok(threshold),
        Err(SqlError::QueryReturnedNoRows) => {
            // Use default thresholds if not set
            let (min_value, max_value) = sensor_types::get_default_threshold(sensor_type);
            
            let threshold = SensorThreshold {
                sensor_type: sensor_type.to_string(),
                min_value,
                max_value,
            };
            
            // Save default threshold
            set_threshold(&threshold)?;
            
            Ok(threshold)
        }
        Err(e) => Err(anyhow!(e)),
    }
}

pub fn set_threshold(threshold: &SensorThreshold) -> Result<()> {
    let db = get_database().ok_or_else(|| anyhow!("Database not initialized"))?;
    let conn = db.lock().map_err(|_| anyhow!("Failed to lock database"))?;
    
    conn.execute(
        "INSERT OR REPLACE INTO sensor_thresholds (sensor_type, min_value, max_value) VALUES (?, ?, ?)",
        params![
            threshold.sensor_type,
            threshold.min_value,
            threshold.max_value
        ],
    )?;
    
    Ok(())
}

pub fn get_all_thresholds() -> Result<Vec<SensorThreshold>> {
    let db = get_database().ok_or_else(|| anyhow!("Database not initialized"))?;
    let conn = db.lock().map_err(|_| anyhow!("Failed to lock database"))?;
    
    let mut stmt = conn.prepare(
        "SELECT sensor_type, min_value, max_value FROM sensor_thresholds"
    )?;
    
    let rows = stmt.query_map([], |row| {
        Ok(SensorThreshold {
            sensor_type: row.get(0)?,
            min_value: row.get(1)?,
            max_value: row.get(2)?,
        })
    })?;
    
    let mut thresholds = Vec::new();
    for row in rows {
        thresholds.push(row?);
    }
    
    Ok(thresholds)
} 
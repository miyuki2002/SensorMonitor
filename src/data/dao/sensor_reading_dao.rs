use anyhow::{Result, anyhow};
use rusqlite::{Connection, params};
use crate::model::sensor_data::SensorReading;
use crate::data::get_database;

pub fn insert(reading: &SensorReading) -> Result<i64> {
    let db = get_database().ok_or_else(|| anyhow!("Database not initialized"))?;
    let conn = db.lock().map_err(|_| anyhow!("Failed to lock database"))?;
    
    conn.execute(
        "INSERT INTO sensor_readings (sensor_type, value, timestamp, is_alert) VALUES (?, ?, ?, ?)",
        params![
            reading.sensor_type,
            reading.value,
            reading.timestamp,
            reading.is_alert as i32
        ],
    )?;
    
    Ok(conn.last_insert_rowid())
}

pub fn insert_batch(readings: &[SensorReading]) -> Result<()> {
    let db = get_database().ok_or_else(|| anyhow!("Database not initialized"))?;
    let conn = db.lock().map_err(|_| anyhow!("Failed to lock database"))?;
    
    let tx = conn.transaction()?;
    
    for reading in readings {
        tx.execute(
            "INSERT INTO sensor_readings (sensor_type, value, timestamp, is_alert) VALUES (?, ?, ?, ?)",
            params![
                reading.sensor_type,
                reading.value,
                reading.timestamp,
                reading.is_alert as i32
            ],
        )?;
    }
    
    tx.commit()?;
    Ok(())
}

pub fn get_latest_by_type(sensor_type: &str) -> Result<Option<SensorReading>> {
    let db = get_database().ok_or_else(|| anyhow!("Database not initialized"))?;
    let conn = db.lock().map_err(|_| anyhow!("Failed to lock database"))?;
    
    let mut stmt = conn.prepare(
        "SELECT id, sensor_type, value, timestamp, is_alert 
         FROM sensor_readings 
         WHERE sensor_type = ? 
         ORDER BY timestamp DESC 
         LIMIT 1"
    )?;
    
    let mut rows = stmt.query(params![sensor_type])?;
    
    if let Some(row) = rows.next()? {
        Ok(Some(SensorReading {
            id: Some(row.get(0)?),
            sensor_type: row.get(1)?,
            value: row.get(2)?,
            timestamp: row.get(3)?,
            is_alert: row.get::<_, i32>(4)? != 0,
        }))
    } else {
        Ok(None)
    }
}

pub fn get_latest_readings() -> Result<Vec<SensorReading>> {
    let db = get_database().ok_or_else(|| anyhow!("Database not initialized"))?;
    let conn = db.lock().map_err(|_| anyhow!("Failed to lock database"))?;
    
    let mut stmt = conn.prepare(
        "SELECT sr.id, sr.sensor_type, sr.value, sr.timestamp, sr.is_alert
         FROM sensor_readings sr
         INNER JOIN (
            SELECT sensor_type, MAX(timestamp) as max_timestamp
            FROM sensor_readings
            GROUP BY sensor_type
         ) latest ON sr.sensor_type = latest.sensor_type AND sr.timestamp = latest.max_timestamp"
    )?;
    
    let rows = stmt.query_map([], |row| {
        Ok(SensorReading {
            id: Some(row.get(0)?),
            sensor_type: row.get(1)?,
            value: row.get(2)?,
            timestamp: row.get(3)?,
            is_alert: row.get::<_, i32>(4)? != 0,
        })
    })?;
    
    let mut readings = Vec::new();
    for row in rows {
        readings.push(row?);
    }
    
    Ok(readings)
}

pub fn get_history_by_type(sensor_type: &str, limit: i64) -> Result<Vec<SensorReading>> {
    let db = get_database().ok_or_else(|| anyhow!("Database not initialized"))?;
    let conn = db.lock().map_err(|_| anyhow!("Failed to lock database"))?;
    
    let mut stmt = conn.prepare(
        "SELECT id, sensor_type, value, timestamp, is_alert 
         FROM sensor_readings 
         WHERE sensor_type = ? 
         ORDER BY timestamp DESC 
         LIMIT ?"
    )?;
    
    let rows = stmt.query_map(params![sensor_type, limit], |row| {
        Ok(SensorReading {
            id: Some(row.get(0)?),
            sensor_type: row.get(1)?,
            value: row.get(2)?,
            timestamp: row.get(3)?,
            is_alert: row.get::<_, i32>(4)? != 0,
        })
    })?;
    
    let mut readings = Vec::new();
    for row in rows {
        readings.push(row?);
    }
    
    Ok(readings)
} 
pub mod dao;
pub mod entity;
pub mod database;

use anyhow::Result;
use once_cell::sync::OnceCell;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

static DATABASE: OnceCell<Arc<Mutex<Connection>>> = OnceCell::new();

/// Initialize the database connection
pub fn initialize_database() -> Result<()> {
    let db_path = get_database_path();
    
    // Create parent directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    
    // Open or create database
    let conn = Connection::open(&db_path)?;
    
    // Create tables
    create_tables(&conn)?;
    
    // Store connection in global static
    DATABASE.get_or_init(|| Arc::new(Mutex::new(conn)));
    
    Ok(())
}

/// Get the database path
fn get_database_path() -> PathBuf {
    match std::env::var("ANDROID_DATA") {
        Ok(data_dir) => PathBuf::from(format!("{}/data/com.example.sensormonitor/databases/sensor_monitor.db", data_dir)),
        Err(_) => {
            if let Some(dir) = dirs_next::data_local_dir() {
                dir.join("sensor_monitor").join("sensor_monitor.db")
            } else {
                PathBuf::from("sensor_monitor.db")
            }
        }
    }
}

/// Get a reference to the database connection
pub fn get_database() -> Option<Arc<Mutex<Connection>>> {
    DATABASE.get().cloned()
}

/// Create database tables
fn create_tables(conn: &Connection) -> Result<()> {
    // Create sensor readings table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sensor_readings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sensor_type TEXT NOT NULL,
            value REAL NOT NULL,
            timestamp INTEGER NOT NULL,
            is_alert INTEGER NOT NULL
        )",
        [],
    )?;
    
    // Create sensor thresholds table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sensor_thresholds (
            sensor_type TEXT PRIMARY KEY,
            min_value REAL NOT NULL,
            max_value REAL NOT NULL
        )",
        [],
    )?;
    
    Ok(())
} 
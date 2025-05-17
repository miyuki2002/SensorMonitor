use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESP32SensorData {
    pub temperature: f32,
    pub humidity: f32,
    pub water_level: f32,
    pub ph: f32,
    pub salinity: f32,
    pub rain: bool,
    pub soil_moisture: f32,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub id: Option<i64>,
    pub sensor_type: String,
    pub value: f32,
    pub timestamp: i64,
    pub is_alert: bool,
}

impl SensorReading {
    pub fn new(sensor_type: &str, value: f32, timestamp: i64, is_alert: bool) -> Self {
        Self {
            id: None,
            sensor_type: sensor_type.to_string(),
            value,
            timestamp,
            is_alert,
        }
    }
    
    pub fn from_esp32_data(data: &ESP32SensorData) -> Vec<Self> {
        let timestamp = data.timestamp;
        
        vec![
            Self::new("temperature", data.temperature, timestamp, false),
            Self::new("humidity", data.humidity, timestamp, false),
            Self::new("water_level", data.water_level, timestamp, false),
            Self::new("ph", data.ph, timestamp, false),
            Self::new("salinity", data.salinity, timestamp, false),
            Self::new("rain", if data.rain { 1.0 } else { 0.0 }, timestamp, false),
            Self::new("soil_moisture", data.soil_moisture, timestamp, false),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorThreshold {
    pub sensor_type: String,
    pub min_value: f32,
    pub max_value: f32,
} 
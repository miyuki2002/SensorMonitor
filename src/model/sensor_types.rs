pub const TEMPERATURE: &str = "temperature";
pub const HUMIDITY: &str = "humidity";
pub const WATER_LEVEL: &str = "water_level";
pub const PH: &str = "ph";
pub const SALINITY: &str = "salinity";
pub const RAIN: &str = "rain";
pub const SOIL_MOISTURE: &str = "soil_moisture";

pub fn get_display_name(sensor_type: &str) -> &'static str {
    match sensor_type {
        TEMPERATURE => "Temperature",
        HUMIDITY => "Humidity",
        WATER_LEVEL => "Water Level",
        PH => "pH",
        SALINITY => "Salinity",
        RAIN => "Rain",
        SOIL_MOISTURE => "Soil Moisture",
        _ => "Unknown Sensor",
    }
}

pub fn get_unit(sensor_type: &str) -> &'static str {
    match sensor_type {
        TEMPERATURE => "°C",
        HUMIDITY => "%",
        WATER_LEVEL => "cm",
        PH => "pH",
        SALINITY => "ppt",
        RAIN => "",
        SOIL_MOISTURE => "%",
        _ => "",
    }
}

pub fn get_default_threshold(sensor_type: &str) -> (f32, f32) {
    match sensor_type {
        TEMPERATURE => (10.0, 40.0),
        HUMIDITY => (20.0, 80.0),
        WATER_LEVEL => (5.0, 90.0),
        PH => (5.0, 9.0),
        SALINITY => (0.0, 30.0),
        RAIN => (0.0, 1.0),
        SOIL_MOISTURE => (20.0, 80.0),
        _ => (0.0, 100.0),
    }
} 
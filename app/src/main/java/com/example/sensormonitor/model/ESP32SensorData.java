package com.example.sensormonitor.model;

import com.google.gson.annotations.SerializedName;

public class ESP32SensorData {

    @SerializedName("temperature")
    private float temperature;
    
    @SerializedName("humidity")
    private float humidity;
    
    @SerializedName("water_level")
    private float waterLevel;
    
    @SerializedName("ph")
    private float ph;
    
    @SerializedName("salinity")
    private float salinity;
    
    @SerializedName("rain")
    private boolean rain;
    
    @SerializedName("soil_moisture")
    private float soilMoisture;
    
    @SerializedName("timestamp")
    private long timestamp;

    public float getTemperature() {
        return temperature;
    }

    public float getHumidity() {
        return humidity;
    }

    public float getWaterLevel() {
        return waterLevel;
    }

    public float getPh() {
        return ph;
    }

    public float getSalinity() {
        return salinity;
    }

    public boolean isRain() {
        return rain;
    }

    public float getSoilMoisture() {
        return soilMoisture;
    }

    public long getTimestamp() {
        return timestamp;
    }
}

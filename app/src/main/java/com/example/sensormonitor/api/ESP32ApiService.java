package com.example.sensormonitor.api;

import com.example.sensormonitor.model.ESP32SensorData;

import retrofit2.Call;
import retrofit2.http.GET;

public interface ESP32ApiService {
    @GET("/api/sensors")
    Call<ESP32SensorData> getSensorData();
}

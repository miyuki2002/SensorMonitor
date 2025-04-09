package com.example.sensormonitor.util;

import android.content.Context;
import android.content.SharedPreferences;

import com.example.sensormonitor.SensorMonitorApp;

public class SharedPreferencesManager {
    
    private static final String PREF_NAME = "sensor_monitor_preferences";
    private static final String KEY_ESP32_IP_ADDRESS = "esp32_ip_address";
    private static final String DEFAULT_ESP32_IP_ADDRESS = "192.168.1.100";
    private static final String KEY_UPDATE_INTERVAL = "update_interval";
    private static final int DEFAULT_UPDATE_INTERVAL = 15; // phút
    
    private static SharedPreferencesManager instance;
    private final SharedPreferences sharedPreferences;
    
    private SharedPreferencesManager() {
        sharedPreferences = SensorMonitorApp.getAppContext()
                .getSharedPreferences(PREF_NAME, Context.MODE_PRIVATE);
    }
    
    public static synchronized SharedPreferencesManager getInstance() {
        if (instance == null) {
            instance = new SharedPreferencesManager();
        }
        return instance;
    }
    
    public String getESP32IpAddress() {
        return sharedPreferences.getString(KEY_ESP32_IP_ADDRESS, DEFAULT_ESP32_IP_ADDRESS);
    }
    
    public void setESP32IpAddress(String ipAddress) {
        sharedPreferences.edit().putString(KEY_ESP32_IP_ADDRESS, ipAddress).apply();
    }
    
    public int getUpdateInterval() {
        return sharedPreferences.getInt(KEY_UPDATE_INTERVAL, DEFAULT_UPDATE_INTERVAL);
    }
    
    public void setUpdateInterval(int minutes) {
        sharedPreferences.edit().putInt(KEY_UPDATE_INTERVAL, minutes).apply();
    }
}

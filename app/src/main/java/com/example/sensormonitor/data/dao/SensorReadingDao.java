package com.example.sensormonitor.data.dao;

import androidx.lifecycle.LiveData;
import androidx.room.Dao;
import androidx.room.Insert;
import androidx.room.Query;

import com.example.sensormonitor.data.entity.SensorReading;

import java.util.Date;
import java.util.List;

@Dao
public interface SensorReadingDao {
    @Insert
    void insert(SensorReading reading);
    
    @Insert
    void insertAll(List<SensorReading> readings);
    
    @Query("SELECT * FROM sensor_readings WHERE sensorType = :sensorType ORDER BY timestamp DESC LIMIT 1")
    LiveData<SensorReading> getLatestReading(String sensorType);
    
    @Query("SELECT * FROM sensor_readings WHERE sensorType = :sensorType AND timestamp BETWEEN :startDate AND :endDate ORDER BY timestamp ASC")
    LiveData<List<SensorReading>> getReadingsByDateRange(String sensorType, Date startDate, Date endDate);
    
    @Query("SELECT DISTINCT sensorType FROM sensor_readings")
    LiveData<List<String>> getAllSensorTypes();
    
    @Query("DELETE FROM sensor_readings WHERE timestamp < :olderThan")
    void deleteOldData(Date olderThan);
}

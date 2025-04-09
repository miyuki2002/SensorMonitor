package com.example.sensormonitor.data;

import androidx.room.Database;
import androidx.room.RoomDatabase;
import androidx.room.TypeConverters;

import com.example.sensormonitor.data.dao.SensorReadingDao;
import com.example.sensormonitor.data.entity.SensorReading;
import com.example.sensormonitor.util.DateConverter;

@Database(entities = {SensorReading.class}, version = 1, exportSchema = false)
@TypeConverters({DateConverter.class})
public abstract class AppDatabase extends RoomDatabase {
    public abstract SensorReadingDao sensorReadingDao();
}

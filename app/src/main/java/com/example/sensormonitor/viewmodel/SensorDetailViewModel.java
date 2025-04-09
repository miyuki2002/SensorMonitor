package com.example.sensormonitor.viewmodel;

import androidx.lifecycle.LiveData;
import androidx.lifecycle.ViewModel;

import com.example.sensormonitor.data.entity.SensorReading;
import com.example.sensormonitor.repository.SensorRepository;

import java.util.Calendar;
import java.util.Date;
import java.util.List;

public class SensorDetailViewModel extends ViewModel {
    private final SensorRepository repository;
    
    public SensorDetailViewModel() {
        repository = SensorRepository.getInstance();
    }
    
    public LiveData<SensorReading> getLatestReading(String sensorType) {
        return repository.getLatestReading(sensorType);
    }
    
    public LiveData<List<SensorReading>> getHistoricalData(String sensorType, int days) {
        Calendar calendar = Calendar.getInstance();
        Date endDate = calendar.getTime();
        calendar.add(Calendar.DAY_OF_YEAR, -days);
        Date startDate = calendar.getTime();
        
        return repository.getReadingsByDateRange(sensorType, startDate, endDate);
    }
}

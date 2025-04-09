package com.example.sensormonitor.viewmodel;

import androidx.lifecycle.LiveData;
import androidx.lifecycle.MediatorLiveData;
import androidx.lifecycle.MutableLiveData;
import androidx.lifecycle.ViewModel;

import com.example.sensormonitor.data.entity.SensorReading;
import com.example.sensormonitor.repository.SensorRepository;

import java.util.Calendar;
import java.util.Date;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class MainViewModel extends ViewModel {
    
    private final SensorRepository repository;
    private final MediatorLiveData<Map<String, SensorReading>> latestReadings = new MediatorLiveData<>();
    private final Map<String, SensorReading> readingsMap = new HashMap<>();
    
    private final String[] sensorTypes = new String[]{
            "temperature", "humidity", "water_level", "ph", "salinity", "rain", "soil_moisture"
    };
    
    public MainViewModel() {
        repository = SensorRepository.getInstance();
        
        // Khởi tạo bản đồ dữ liệu mới nhất
        for (String sensorType : sensorTypes) {
            // Thêm nguồn vào mediator
            LiveData<SensorReading> readingLiveData = repository.getLatestReading(sensorType);
            latestReadings.addSource(readingLiveData, reading -> {
                if (reading != null) {
                    readingsMap.put(sensorType, reading);
                    latestReadings.setValue(readingsMap);
                }
            });
        }
    }
    
    public LiveData<Map<String, SensorReading>> getLatestReadings() {
        return latestReadings;
    }
    
    public LiveData<List<SensorReading>> getHistoricalData(String sensorType, int days) {
        Calendar calendar = Calendar.getInstance();
        Date endDate = calendar.getTime();
        calendar.add(Calendar.DAY_OF_YEAR, -days);
        Date startDate = calendar.getTime();
        
        return repository.getReadingsByDateRange(sensorType, startDate, endDate);
    }
    
    public void refreshData() {
        repository.fetchLatestSensorData();
    }
    
    public LiveData<Boolean> isLoading() {
        return repository.isLoading();
    }
    
    public LiveData<String> getErrorMessage() {
        return repository.getErrorMessage();
    }
}

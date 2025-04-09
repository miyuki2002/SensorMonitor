package com.example.sensormonitor.repository;

import android.util.Log;

import androidx.annotation.NonNull;
import androidx.lifecycle.LiveData;
import androidx.lifecycle.MutableLiveData;

import com.example.sensormonitor.SensorMonitorApp;
import com.example.sensormonitor.data.AppDatabase;
import com.example.sensormonitor.data.dao.SensorReadingDao;
import com.example.sensormonitor.data.entity.SensorReading;
import com.example.sensormonitor.model.ESP32SensorData;
import com.google.firebase.database.DataSnapshot;
import com.google.firebase.database.DatabaseError;
import com.google.firebase.database.DatabaseReference;
import com.google.firebase.database.FirebaseDatabase;
import com.google.firebase.database.ValueEventListener;

import java.util.ArrayList;
import java.util.Calendar;
import java.util.Date;
import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class SensorRepository {
    private static final String TAG = "SensorRepository";
    private static final String FIREBASE_SENSOR_PATH = "sensor_readings";
    
    private static SensorRepository instance;
    private final DatabaseReference firebaseDatabase;
    private final SensorReadingDao sensorReadingDao;
    private final ExecutorService executorService;
    private final MutableLiveData<Boolean> isLoading = new MutableLiveData<>(false);
    private final MutableLiveData<String> errorMessage = new MutableLiveData<>();
    
    private SensorRepository() {
        // Khởi tạo tham chiếu cơ sở dữ liệu Firebase
        firebaseDatabase = FirebaseDatabase.getInstance().getReference(FIREBASE_SENSOR_PATH);
        
        // Lấy DAO
        AppDatabase db = SensorMonitorApp.getDatabase();
        sensorReadingDao = db.sensorReadingDao();
        
        // Tạo dịch vụ thực thi cho các hoạt động nền
        executorService = Executors.newFixedThreadPool(4);
    }
    
    public static synchronized SensorRepository getInstance() {
        if (instance == null) {
            instance = new SensorRepository();
        }
        return instance;
    }
    
    public void fetchLatestSensorData() {
        isLoading.setValue(true);
        errorMessage.setValue(null);
        
        // Lấy dữ liệu mới nhất từ Firebase
        firebaseDatabase.orderByChild("timestamp").limitToLast(1)
                .addListenerForSingleValueEvent(new ValueEventListener() {
                    @Override
                    public void onDataChange(@NonNull DataSnapshot dataSnapshot) {
                        try {
                            if (dataSnapshot.exists()) {
                                for (DataSnapshot snapshot : dataSnapshot.getChildren()) {
                                    ESP32SensorData data = snapshot.getValue(ESP32SensorData.class);
                                    if (data != null) {
                                        saveSensorData(data);
                                    }
                                }
                            } else {
                                errorMessage.setValue("Không có dữ liệu cảm biến trong Firebase");
                            }
                        } catch (Exception e) {
                            Log.e(TAG, "Lỗi phân tích dữ liệu Firebase", e);
                            errorMessage.setValue("Lỗi đọc dữ liệu: " + e.getMessage());
                        } finally {
                            isLoading.setValue(false);
                        }
                    }

                    @Override
                    public void onCancelled(@NonNull DatabaseError databaseError) {
                        Log.e(TAG, "Lỗi cơ sở dữ liệu Firebase", databaseError.toException());
                        errorMessage.setValue("Lỗi cơ sở dữ liệu: " + databaseError.getMessage());
                        isLoading.setValue(false);
                    }
                });
    }
    
    // Thêm phương thức lắng nghe cập nhật theo thời gian thực
    public void setupRealtimeUpdates() {
        firebaseDatabase.orderByChild("timestamp").limitToLast(1)
                .addValueEventListener(new ValueEventListener() {
                    @Override
                    public void onDataChange(@NonNull DataSnapshot dataSnapshot) {
                        try {
                            for (DataSnapshot snapshot : dataSnapshot.getChildren()) {
                                ESP32SensorData data = snapshot.getValue(ESP32SensorData.class);
                                if (data != null) {
                                    saveSensorData(data);
                                }
                            }
                        } catch (Exception e) {
                            Log.e(TAG, "Lỗi phân tích dữ liệu theo thời gian thực từ Firebase", e);
                        }
                    }

                    @Override
                    public void onCancelled(@NonNull DatabaseError databaseError) {
                        Log.e(TAG, "Cập nhật theo thời gian thực từ Firebase bị hủy", databaseError.toException());
                    }
                });
    }
    
    private void saveSensorData(ESP32SensorData data) {
        executorService.execute(() -> {
            Date timestamp = new Date(data.getTimestamp());
            
            List<SensorReading> readings = new ArrayList<>();
            readings.add(new SensorReading("temperature", data.getTemperature(), "°C", timestamp));
            readings.add(new SensorReading("humidity", data.getHumidity(), "%", timestamp));
            readings.add(new SensorReading("water_level", data.getWaterLevel(), "cm", timestamp));
            readings.add(new SensorReading("ph", data.getPh(), "pH", timestamp));
            readings.add(new SensorReading("salinity", data.getSalinity(), "ppt", timestamp));
            readings.add(new SensorReading("rain", data.isRain() ? 1f : 0f, "", timestamp));
            readings.add(new SensorReading("soil_moisture", data.getSoilMoisture(), "%", timestamp));
            
            sensorReadingDao.insertAll(readings);
        });
    }
    
    public LiveData<SensorReading> getLatestReading(String sensorType) {
        return sensorReadingDao.getLatestReading(sensorType);
    }
    
    public LiveData<List<SensorReading>> getReadingsByDateRange(String sensorType, Date startDate, Date endDate) {
        return sensorReadingDao.getReadingsByDateRange(sensorType, startDate, endDate);
    }
    
    public LiveData<List<String>> getAllSensorTypes() {
        return sensorReadingDao.getAllSensorTypes();
    }
    
    public void cleanupOldData(int daysToKeep) {
        executorService.execute(() -> {
            Calendar calendar = Calendar.getInstance();
            calendar.add(Calendar.DAY_OF_YEAR, -daysToKeep);
            sensorReadingDao.deleteOldData(calendar.getTime());
        });
    }
    
    public LiveData<Boolean> isLoading() {
        return isLoading;
    }
    
    public LiveData<String> getErrorMessage() {
        return errorMessage;
    }
}

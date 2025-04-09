package com.example.sensormonitor;

import android.app.Application;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.content.Context;
import android.os.Build;

import androidx.room.Room;
import androidx.work.Constraints;
import androidx.work.ExistingPeriodicWorkPolicy;
import androidx.work.NetworkType;
import androidx.work.PeriodicWorkRequest;
import androidx.work.WorkManager;

import com.example.sensormonitor.data.AppDatabase;
import com.example.sensormonitor.worker.SensorDataWorker;
import com.google.firebase.FirebaseApp;

import java.util.concurrent.TimeUnit;

public class SensorMonitorApp extends Application {
    
    public static final String SENSOR_NOTIFICATION_CHANNEL_ID = "sensor_notification_channel";
    private static AppDatabase database;
    private static Context appContext;

    @Override
    public void onCreate() {
        super.onCreate();
        appContext = getApplicationContext();
        
        // Khởi tạo Firebase
        FirebaseApp.initializeApp(this);
        
        // Khởi tạo cơ sở dữ liệu
        database = Room.databaseBuilder(getApplicationContext(),
                AppDatabase.class, "sensor-monitor-db")
                .fallbackToDestructiveMigration()
                .build();
        
        // Tạo kênh thông báo
        createNotificationChannel();
        
        // Lên lịch cập nhật dữ liệu nền
        scheduleDataUpdates();
    }
    
    private void createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            NotificationChannel channel = new NotificationChannel(
                    SENSOR_NOTIFICATION_CHANNEL_ID,
                    "Sensor Monitoring",
                    NotificationManager.IMPORTANCE_DEFAULT
            );
            channel.setDescription("Notifications for sensor readings");
            
            NotificationManager notificationManager = getSystemService(NotificationManager.class);
            notificationManager.createNotificationChannel(channel);
        }
    }
    
    private void scheduleDataUpdates() {
        Constraints constraints = new Constraints.Builder()
                .setRequiredNetworkType(NetworkType.CONNECTED)
                .build();
                
        PeriodicWorkRequest sensorDataWorkRequest =
                new PeriodicWorkRequest.Builder(SensorDataWorker.class, 15, TimeUnit.MINUTES)
                        .setConstraints(constraints)
                        .build();
                        
        WorkManager.getInstance(this).enqueueUniquePeriodicWork(
                "sensorDataSync",
                ExistingPeriodicWorkPolicy.KEEP,
                sensorDataWorkRequest);
    }
    
    public static AppDatabase getDatabase() {
        return database;
    }

    // Thêm phương thức này để truy cập context từ mọi nơi
    public static Context getAppContext() {
        return appContext;
    }
}

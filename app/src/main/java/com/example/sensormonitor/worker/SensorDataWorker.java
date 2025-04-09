package com.example.sensormonitor.worker;

import android.content.Context;
import android.util.Log;

import androidx.annotation.NonNull;
import androidx.work.Worker;
import androidx.work.WorkerParameters;

import com.example.sensormonitor.repository.SensorRepository;

public class SensorDataWorker extends Worker {
    
    private static final String TAG = "SensorDataWorker";
    
    public SensorDataWorker(@NonNull Context context, @NonNull WorkerParameters workerParams) {
        super(context, workerParams);
    }

    @NonNull
    @Override
    public Result doWork() {
        Log.d(TAG, "Bắt đầu đồng bộ dữ liệu nền");
        try {
            SensorRepository repository = SensorRepository.getInstance();
            repository.fetchLatestSensorData();
            
            // Thiết lập cập nhật theo thời gian thực nếu cần (hữu ích cho việc khởi động lại dịch vụ)
            repository.setupRealtimeUpdates();
            
            // Dọn dẹp dữ liệu cũ - giữ lịch sử 30 ngày
            repository.cleanupOldData(30);
            
            Log.d(TAG, "Đồng bộ dữ liệu nền hoàn thành thành công");
            return Result.success();
        } catch (Exception e) {
            Log.e(TAG, "Đồng bộ dữ liệu nền thất bại", e);
            return Result.retry();
        }
    }
}

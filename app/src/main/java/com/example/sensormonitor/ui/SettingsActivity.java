package com.example.sensormonitor.ui;

import android.content.Context;
import android.content.Intent;
import android.os.Bundle;
import android.view.MenuItem;
import android.widget.Toast;

import androidx.annotation.NonNull;
import androidx.appcompat.app.AppCompatActivity;

import com.example.sensormonitor.databinding.ActivitySettingsBinding;
import com.example.sensormonitor.util.SharedPreferencesManager;

public class SettingsActivity extends AppCompatActivity {

    private ActivitySettingsBinding binding;
    private SharedPreferencesManager preferencesManager;

    public static Intent getIntent(Context context) {
        return new Intent(context, SettingsActivity.class);
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        binding = ActivitySettingsBinding.inflate(getLayoutInflater());
        setContentView(binding.getRoot());

        setSupportActionBar(binding.toolbar);
        if (getSupportActionBar() != null) {
            getSupportActionBar().setDisplayHomeAsUpEnabled(true);
            getSupportActionBar().setTitle("Cài đặt");
        }

        preferencesManager = SharedPreferencesManager.getInstance();
        
        // Khởi tạo giá trị hiện tại
        binding.editTextFirebaseUrl.setText(preferencesManager.getESP32IpAddress());
        binding.sliderUpdateInterval.setValue(preferencesManager.getUpdateInterval());
        binding.textViewIntervalValue.setText(String.valueOf(preferencesManager.getUpdateInterval()) + " phút");
        
        // Xử lý thay đổi slider
        binding.sliderUpdateInterval.addOnChangeListener((slider, value, fromUser) -> {
            int interval = (int) value;
            binding.textViewIntervalValue.setText(interval + " phút");
        });
        
        // Xử lý nút lưu
        binding.buttonSave.setOnClickListener(v -> saveSettings());
    }
    
    private void saveSettings() {
        String firebaseUrl = binding.editTextFirebaseUrl.getText().toString().trim();
        int updateInterval = (int) binding.sliderUpdateInterval.getValue();
        
        if (firebaseUrl.isEmpty()) {
            Toast.makeText(this, "Vui lòng nhập URL Firebase", Toast.LENGTH_SHORT).show();
            return;
        }
        
        // Lưu cài đặt
        preferencesManager.setESP32IpAddress(firebaseUrl);
        preferencesManager.setUpdateInterval(updateInterval);
        
        Toast.makeText(this, "Đã lưu cài đặt", Toast.LENGTH_SHORT).show();
        finish();
    }

    @Override
    public boolean onOptionsItemSelected(@NonNull MenuItem item) {
        if (item.getItemId() == android.R.id.home) {
            finish();
            return true;
        }
        return super.onOptionsItemSelected(item);
    }
}

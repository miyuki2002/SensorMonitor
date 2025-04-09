package com.example.sensormonitor;

import android.os.Bundle;
import android.view.Menu;
import android.view.MenuItem;
import android.view.View;
import android.widget.Toast;

import androidx.annotation.NonNull;
import androidx.appcompat.app.AppCompatActivity;
import androidx.lifecycle.ViewModelProvider;
import androidx.swiperefreshlayout.widget.SwipeRefreshLayout;

import com.example.sensormonitor.data.entity.SensorReading;
import com.example.sensormonitor.databinding.ActivityMainBinding;
import com.example.sensormonitor.repository.SensorRepository;
import com.example.sensormonitor.ui.SensorDetailActivity;
import com.example.sensormonitor.ui.SettingsActivity;
import com.example.sensormonitor.ui.adapter.SensorCardAdapter;
import com.example.sensormonitor.viewmodel.MainViewModel;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

public class MainActivity extends AppCompatActivity {
    
    private ActivityMainBinding binding;
    private MainViewModel viewModel;
    private SensorCardAdapter adapter;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        binding = ActivityMainBinding.inflate(getLayoutInflater());
        setContentView(binding.getRoot());
        
        setSupportActionBar(binding.toolbar);
        
        viewModel = new ViewModelProvider(this).get(MainViewModel.class);
        
        setupRecyclerView();
        setupSwipeRefresh();
        setupObservers();
        
        // Lấy dữ liệu ban đầu
        viewModel.refreshData();
        
        // Thiết lập cập nhật theo thời gian thực từ Firebase
        SensorRepository.getInstance().setupRealtimeUpdates();
    }

    private void setupRecyclerView() {
        adapter = new SensorCardAdapter(reading -> {
            // Xử lý khi nhấp - mở xem chi tiết
            startActivity(SensorDetailActivity.getIntent(this, reading.getSensorType()));
        });
        binding.recyclerViewSensors.setAdapter(adapter);
    }
    
    private void setupSwipeRefresh() {
        binding.swipeRefreshLayout.setOnRefreshListener(() -> {
            viewModel.refreshData();
        });
    }
    
    private void setupObservers() {
        viewModel.getLatestReadings().observe(this, this::updateSensorCards);
        
        viewModel.isLoading().observe(this, isLoading -> {
            binding.swipeRefreshLayout.setRefreshing(isLoading);
        });
        
        viewModel.getErrorMessage().observe(this, errorMsg -> {
            if (errorMsg != null && !errorMsg.isEmpty()) {
                Toast.makeText(this, errorMsg, Toast.LENGTH_LONG).show();
            }
        });
    }
    
    private void updateSensorCards(Map<String, SensorReading> readingsMap) {
        if (readingsMap == null || readingsMap.isEmpty()) {
            binding.textViewNoData.setVisibility(View.VISIBLE);
            return;
        }
        
        binding.textViewNoData.setVisibility(View.GONE);
        List<SensorReading> readings = new ArrayList<>(readingsMap.values());
        adapter.submitList(readings);
    }

    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        getMenuInflater().inflate(R.menu.menu_main, menu);
        return true;
    }

    @Override
    public boolean onOptionsItemSelected(@NonNull MenuItem item) {
        int id = item.getItemId();
        if (id == R.id.action_settings) {
            startActivity(SettingsActivity.getIntent(this));
            return true;
        } else if (id == R.id.action_refresh) {
            viewModel.refreshData();
            return true;
        }
        return super.onOptionsItemSelected(item);
    }
}

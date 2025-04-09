package com.example.sensormonitor.ui;

import android.content.Context;
import android.content.Intent;
import android.graphics.Color;
import android.os.Bundle;
import android.view.MenuItem;

import androidx.annotation.NonNull;
import androidx.appcompat.app.AppCompatActivity;
import androidx.lifecycle.ViewModelProvider;

import com.example.sensormonitor.R;
import com.example.sensormonitor.data.entity.SensorReading;
import com.example.sensormonitor.databinding.ActivitySensorDetailBinding;
import com.example.sensormonitor.viewmodel.SensorDetailViewModel;
import com.github.mikephil.charting.components.XAxis;
import com.github.mikephil.charting.components.YAxis;
import com.github.mikephil.charting.data.Entry;
import com.github.mikephil.charting.data.LineData;
import com.github.mikephil.charting.data.LineDataSet;
import com.github.mikephil.charting.formatter.ValueFormatter;

import java.text.SimpleDateFormat;
import java.util.ArrayList;
import java.util.Date;
import java.util.List;
import java.util.Locale;

public class SensorDetailActivity extends AppCompatActivity {

    private static final String EXTRA_SENSOR_TYPE = "extra_sensor_type";
    
    private ActivitySensorDetailBinding binding;
    private SensorDetailViewModel viewModel;
    private String sensorType;

    public static Intent getIntent(Context context, String sensorType) {
        Intent intent = new Intent(context, SensorDetailActivity.class);
        intent.putExtra(EXTRA_SENSOR_TYPE, sensorType);
        return intent;
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        binding = ActivitySensorDetailBinding.inflate(getLayoutInflater());
        setContentView(binding.getRoot());

        setSupportActionBar(binding.toolbar);
        if (getSupportActionBar() != null) {
            getSupportActionBar().setDisplayHomeAsUpEnabled(true);
        }

        sensorType = getIntent().getStringExtra(EXTRA_SENSOR_TYPE);
        if (sensorType == null) {
            finish();
            return;
        }

        // Đặt tiêu đề dựa trên loại cảm biến
        setTitle(getSensorDisplayName(sensorType));
        
        // Cấu hình biểu đồ
        setupChart();
        
        // Khởi tạo ViewModel
        viewModel = new ViewModelProvider(this).get(SensorDetailViewModel.class);
        
        // Cấu hình nút thời gian
        setupTimeButtons();
        
        // Quan sát dữ liệu
        viewModel.getLatestReading(sensorType).observe(this, this::updateCurrentValue);
        
        // Tải dữ liệu lịch sử ban đầu (mặc định 24 giờ)
        loadHistoricalData(1);
    }

    private void setupChart() {
        binding.lineChart.getDescription().setEnabled(false);
        binding.lineChart.setTouchEnabled(true);
        binding.lineChart.setDragEnabled(true);
        binding.lineChart.setScaleEnabled(true);
        binding.lineChart.setPinchZoom(true);
        binding.lineChart.setDrawGridBackground(false);
        binding.lineChart.getLegend().setEnabled(false);
        
        XAxis xAxis = binding.lineChart.getXAxis();
        xAxis.setPosition(XAxis.XAxisPosition.BOTTOM);
        xAxis.setDrawGridLines(false);
        xAxis.setGranularity(1f);
        xAxis.setValueFormatter(new ValueFormatter() {
            private final SimpleDateFormat mFormat = new SimpleDateFormat("HH:mm", Locale.getDefault());
            
            @Override
            public String getFormattedValue(float value) {
                return mFormat.format(new Date((long) value));
            }
        });
        
        YAxis leftAxis = binding.lineChart.getAxisLeft();
        leftAxis.setDrawGridLines(true);
        
        binding.lineChart.getAxisRight().setEnabled(false);
    }
    
    private void setupTimeButtons() {
        binding.buttonDay.setOnClickListener(v -> {
            loadHistoricalData(1);
            updateButtonSelection(binding.buttonDay);
        });
        
        binding.buttonWeek.setOnClickListener(v -> {
            loadHistoricalData(7);
            updateButtonSelection(binding.buttonWeek);
        });
        
        binding.buttonMonth.setOnClickListener(v -> {
            loadHistoricalData(30);
            updateButtonSelection(binding.buttonMonth);
        });
        
        // Mặc định chọn nút ngày
        updateButtonSelection(binding.buttonDay);
    }
    
    private void updateButtonSelection(androidx.appcompat.widget.AppCompatButton selectedButton) {
        binding.buttonDay.setSelected(selectedButton == binding.buttonDay);
        binding.buttonWeek.setSelected(selectedButton == binding.buttonWeek);
        binding.buttonMonth.setSelected(selectedButton == binding.buttonMonth);
    }
    
    private void loadHistoricalData(int days) {
        binding.progressBar.setVisibility(android.view.View.VISIBLE);
        
        viewModel.getHistoricalData(sensorType, days).observe(this, readings -> {
            updateChart(readings);
            binding.progressBar.setVisibility(android.view.View.GONE);
        });
    }
    
    private void updateCurrentValue(SensorReading reading) {
        if (reading != null) {
            String value = String.format(Locale.getDefault(), "%.1f%s", reading.getValue(), reading.getUnit());
            binding.textViewCurrentValue.setText(value);
            
            SimpleDateFormat dateFormat = new SimpleDateFormat("dd/MM/yyyy HH:mm:ss", Locale.getDefault());
            String dateTime = dateFormat.format(reading.getTimestamp());
            binding.textViewTimestamp.setText(dateTime);
        }
    }
    
    private void updateChart(List<SensorReading> readings) {
        if (readings == null || readings.isEmpty()) {
            binding.lineChart.clear();
            binding.lineChart.invalidate();
            return;
        }
        
        ArrayList<Entry> values = new ArrayList<>();
        for (SensorReading reading : readings) {
            values.add(new Entry(reading.getTimestamp().getTime(), reading.getValue()));
        }
        
        LineDataSet dataSet;
        if (binding.lineChart.getData() != null && 
            binding.lineChart.getData().getDataSetCount() > 0) {
            dataSet = (LineDataSet) binding.lineChart.getData().getDataSetByIndex(0);
            dataSet.setValues(values);
            binding.lineChart.getData().notifyDataChanged();
            binding.lineChart.notifyDataSetChanged();
        } else {
            dataSet = new LineDataSet(values, "Dữ liệu cảm biến");
            dataSet.setDrawIcons(false);
            dataSet.setColor(getChartColor());
            dataSet.setCircleColor(getChartColor());
            dataSet.setLineWidth(2f);
            dataSet.setCircleRadius(3f);
            dataSet.setDrawCircleHole(false);
            dataSet.setValueTextSize(9f);
            dataSet.setDrawValues(false);
            dataSet.setDrawFilled(true);
            dataSet.setFillColor(getChartColor());
            dataSet.setFillAlpha(50);
            dataSet.setMode(LineDataSet.Mode.CUBIC_BEZIER);
            
            LineData lineData = new LineData(dataSet);
            binding.lineChart.setData(lineData);
        }
        
        binding.lineChart.invalidate();
    }
    
    private String getSensorDisplayName(String sensorType) {
        switch (sensorType) {
            case "temperature":
                return "Nhiệt độ";
            case "humidity":
                return "Độ ẩm";
            case "water_level":
                return "Mực nước";
            case "ph":
                return "Độ pH";
            case "salinity":
                return "Độ mặn";
            case "rain":
                return "Mưa";
            case "soil_moisture":
                return "Độ ẩm đất";
            default:
                return sensorType;
        }
    }
    
    private int getChartColor() {
        switch (sensorType) {
            case "temperature":
                return Color.RED;
            case "humidity":
                return Color.BLUE;
            case "water_level":
                return Color.CYAN;
            case "ph":
                return Color.MAGENTA;
            case "salinity":
                return Color.GREEN;
            case "rain":
                return Color.DKGRAY;
            case "soil_moisture":
                return Color.rgb(139, 69, 19); // Nâu
            default:
                return Color.BLACK;
        }
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

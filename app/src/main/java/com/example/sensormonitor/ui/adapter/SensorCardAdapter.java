package com.example.sensormonitor.ui.adapter;

import android.text.format.DateUtils;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;

import androidx.annotation.NonNull;
import androidx.recyclerview.widget.DiffUtil;
import androidx.recyclerview.widget.ListAdapter;
import androidx.recyclerview.widget.RecyclerView;

import com.example.sensormonitor.R;
import com.example.sensormonitor.data.entity.SensorReading;
import com.example.sensormonitor.databinding.ItemSensorCardBinding;

import java.util.Date;

public class SensorCardAdapter extends ListAdapter<SensorReading, SensorCardAdapter.SensorViewHolder> {

    private final OnSensorClickListener clickListener;

    public interface OnSensorClickListener {
        void onSensorClick(SensorReading reading);
    }

    public SensorCardAdapter(OnSensorClickListener listener) {
        super(new SensorDiffCallback());
        this.clickListener = listener;
    }

    @NonNull
    @Override
    public SensorViewHolder onCreateViewHolder(@NonNull ViewGroup parent, int viewType) {
        ItemSensorCardBinding binding = ItemSensorCardBinding.inflate(
                LayoutInflater.from(parent.getContext()), parent, false);
        return new SensorViewHolder(binding);
    }

    @Override
    public void onBindViewHolder(@NonNull SensorViewHolder holder, int position) {
        holder.bind(getItem(position), clickListener);
    }

    static class SensorViewHolder extends RecyclerView.ViewHolder {
        private final ItemSensorCardBinding binding;

        public SensorViewHolder(@NonNull ItemSensorCardBinding binding) {
            super(binding.getRoot());
            this.binding = binding;
        }

        public void bind(SensorReading reading, OnSensorClickListener listener) {
            binding.getRoot().setOnClickListener(v -> listener.onSensorClick(reading));
            
            // Đặt tên cảm biến dựa trên loại
            String sensorName = getSensorDisplayName(reading.getSensorType());
            binding.textViewSensorName.setText(sensorName);
            
            // Đặt giá trị cảm biến với đơn vị
            String value = String.format("%.1f%s", reading.getValue(), reading.getUnit());
            binding.textViewSensorValue.setText(value);
            
            // Đặt thời gian cập nhật
            String timeAgo = getTimeAgoString(reading.getTimestamp());
            binding.textViewLastUpdate.setText("Cập nhật: " + timeAgo);
            
            // Đặt biểu tượng cảm biến
            binding.imageViewSensor.setImageResource(getSensorIcon(reading.getSensorType()));
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
        
        private int getSensorIcon(String sensorType) {
            switch (sensorType) {
                case "temperature":
                    return R.drawable.ic_temperature;
                case "humidity":
                    return R.drawable.ic_humidity;
                case "water_level":
                    return R.drawable.ic_water_level;
                case "ph":
                    return R.drawable.ic_ph;
                case "salinity":
                    return R.drawable.ic_salinity;
                case "rain":
                    return R.drawable.ic_rain;
                case "soil_moisture":
                    return R.drawable.ic_soil;
                default:
                    return R.drawable.ic_sensor;
            }
        }
        
        private String getTimeAgoString(Date timestamp) {
            return DateUtils.getRelativeTimeSpanString(
                    timestamp.getTime(),
                    System.currentTimeMillis(),
                    DateUtils.MINUTE_IN_MILLIS
            ).toString();
        }
    }

    static class SensorDiffCallback extends DiffUtil.ItemCallback<SensorReading> {
        @Override
        public boolean areItemsTheSame(@NonNull SensorReading oldItem, @NonNull SensorReading newItem) {
            return oldItem.getSensorType().equals(newItem.getSensorType());
        }

        @Override
        public boolean areContentsTheSame(@NonNull SensorReading oldItem, @NonNull SensorReading newItem) {
            return oldItem.getValue() == newItem.getValue() &&
                   oldItem.getTimestamp().equals(newItem.getTimestamp());
        }
    }
}

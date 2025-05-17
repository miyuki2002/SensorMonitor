use eframe::{egui, NativeOptions};
use egui_plot::{Line, Plot, PlotPoints};
use std::time::Instant;

mod model;
mod data;
mod api;
mod repository;
mod util;
mod worker;

#[cfg(target_os = "android")]
use ndk_glue::{android_main, native_activity::NativeActivity};

#[cfg(target_os = "android")]
#[no_mangle]
#[android_main]
fn android_main(app: NativeActivity) {
    android_run(app);
}

#[cfg(target_os = "android")]
fn android_run(app: NativeActivity) {
    // Khởi tạo logging
    env_logger::init();
    log::info!("Starting Sensor Monitor app");

    // Khởi tạo cơ sở dữ liệu
    if let Err(e) = data::initialize_database() {
        log::error!("Failed to initialize database: {}", e);
    }

    // Chạy ứng dụng
    let options = NativeOptions::default();
    eframe::run_native(
        "Sensor Monitor",
        options,
        Box::new(|cc| Box::new(SensorMonitorApp::new(cc))),
    )
    .expect("Failed to start app");
}

#[cfg(not(target_os = "android"))]
fn main() {
    // Khởi tạo logging
    env_logger::init();
    log::info!("Starting Sensor Monitor app");

    // Khởi tạo cơ sở dữ liệu
    if let Err(e) = data::initialize_database() {
        log::error!("Failed to initialize database: {}", e);
    }

    // Chạy ứng dụng
    let options = NativeOptions::default();
    eframe::run_native(
        "Sensor Monitor",
        options,
        Box::new(|cc| Box::new(SensorMonitorApp::new(cc))),
    )
    .expect("Failed to start app");
}

struct SensorMonitorApp {
    selected_tab: Tab,
    esp32_url: String,
    last_update: Instant,
    sensor_data: std::collections::HashMap<String, model::sensor_data::SensorReading>,
    sensor_history: Vec<model::sensor_data::SensorReading>,
    selected_sensor: String,
    error_message: Option<String>,
    is_loading: bool,
}

enum Tab {
    Dashboard,
    History,
    Settings,
}

impl Default for SensorMonitorApp {
    fn default() -> Self {
        Self {
            selected_tab: Tab::Dashboard,
            esp32_url: String::from("http://192.168.1.100"),
            last_update: Instant::now(),
            sensor_data: std::collections::HashMap::new(),
            sensor_history: Vec::new(),
            selected_sensor: String::from(model::sensor_types::TEMPERATURE),
            error_message: None,
            is_loading: false,
        }
    }
}

impl SensorMonitorApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Tùy chỉnh font và style
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(24.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(18.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(16.0, egui::FontFamily::Monospace)),
            (egui::TextStyle::Button, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Small, egui::FontId::new(12.0, egui::FontFamily::Proportional)),
        ].into();
        cc.egui_ctx.set_style(style);
        
        // Khởi tạo trạng thái mặc định
        let mut app = Self::default();
        
        // Tải URL ESP32 từ cài đặt
        if let Ok(url) = util::preferences::load_esp32_url() {
            app.esp32_url = url;
        }
        
        // Kích hoạt cập nhật dữ liệu ban đầu
        app.refresh_data();
        
        app
    }
    
    fn refresh_data(&mut self) {
        self.is_loading = true;
        self.error_message = None;
        
        // Sử dụng reqwest blocking để thực hiện yêu cầu HTTP
        let esp32_url = self.esp32_url.clone();
        
        // Tải dữ liệu sử dụng repository
        match repository::sensor_repository::fetch_latest_readings() {
            Ok(json) => {
                match serde_json::from_str(&json) {
                    Ok(data) => {
                        self.sensor_data = data;
                        self.last_update = Instant::now();
                    },
                    Err(e) => {
                        self.error_message = Some(format!("Failed to parse data: {}", e));
                    }
                }
            },
            Err(e) => {
                self.error_message = Some(format!("Failed to fetch data: {}", e));
            }
        }
        
        self.is_loading = false;
    }
    
    fn load_history(&mut self) {
        self.is_loading = true;
        self.error_message = None;
        
        // Tải lịch sử cho cảm biến đã chọn
        match repository::sensor_repository::fetch_reading_history(&self.selected_sensor, 100) {
            Ok(json) => {
                match serde_json::from_str(&json) {
                    Ok(data) => {
                        self.sensor_history = data;
                    },
                    Err(e) => {
                        self.error_message = Some(format!("Failed to parse history data: {}", e));
                    }
                }
            },
            Err(e) => {
                self.error_message = Some(format!("Failed to fetch history: {}", e));
            }
        }
        
        self.is_loading = false;
    }
}

impl eframe::App for SensorMonitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Tự động làm mới dữ liệu mỗi 60 giây
        if self.last_update.elapsed().as_secs() > 60 && !self.is_loading {
            self.refresh_data();
        }
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Sensor Monitor");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⟳").clicked() {
                        self.refresh_data();
                    }
                });
            });
            
            ui.horizontal(|ui| {
                if ui.selectable_label(matches!(self.selected_tab, Tab::Dashboard), "Dashboard").clicked() {
                    self.selected_tab = Tab::Dashboard;
                }
                if ui.selectable_label(matches!(self.selected_tab, Tab::History), "History").clicked() {
                    self.selected_tab = Tab::History;
                    self.load_history();
                }
                if ui.selectable_label(matches!(self.selected_tab, Tab::Settings), "Settings").clicked() {
                    self.selected_tab = Tab::Settings;
                }
            });
        });
        
        if let Some(error) = &self.error_message {
            egui::Window::new("Error")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(error);
                    if ui.button("Close").clicked() {
                        self.error_message = None;
                    }
                });
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.selected_tab {
                Tab::Dashboard => self.render_dashboard(ui),
                Tab::History => self.render_history(ui),
                Tab::Settings => self.render_settings(ui),
            }
        });
    }
}

impl SensorMonitorApp {
    fn render_dashboard(&mut self, ui: &mut egui::Ui) {
        ui.heading("Sensor Readings");
        
        if self.is_loading {
            ui.spinner();
            ui.label("Loading data...");
            return;
        }
        
        if self.sensor_data.is_empty() {
            ui.label("No sensor data available. Click refresh to load data.");
            return;
        }
        
        egui::Grid::new("sensors_grid")
            .striped(true)
            .spacing([40.0, 20.0])
            .show(ui, |ui| {
                ui.label("Sensor");
                ui.label("Value");
                ui.label("Status");
                ui.end_row();
                
                for sensor_type in [
                    model::sensor_types::TEMPERATURE,
                    model::sensor_types::HUMIDITY,
                    model::sensor_types::WATER_LEVEL,
                    model::sensor_types::PH,
                    model::sensor_types::SALINITY,
                    model::sensor_types::RAIN,
                    model::sensor_types::SOIL_MOISTURE,
                ] {
                    if let Some(reading) = self.sensor_data.get(sensor_type) {
                        let display_name = model::sensor_types::get_display_name(sensor_type);
                        let unit = model::sensor_types::get_unit(sensor_type);
                        
                        ui.label(display_name);
                        
                        let value_text = if sensor_type == model::sensor_types::RAIN {
                            if reading.value > 0.5 { "Yes" } else { "No" }.to_string()
                        } else {
                            format!("{:.1} {}", reading.value, unit)
                        };
                        
                        ui.label(value_text);
                        
                        let status_text = if reading.is_alert {
                            "⚠ Alert"
                        } else {
                            "✓ Normal"
                        };
                        
                        let status_color = if reading.is_alert {
                            egui::Color32::from_rgb(255, 100, 100)
                        } else {
                            egui::Color32::from_rgb(100, 255, 100)
                        };
                        
                        ui.colored_label(status_color, status_text);
                        
                        ui.end_row();
                    }
                }
            });
        
        ui.add_space(20.0);
        ui.label(format!("Last updated: {}", 
            util::date_converter::format_timestamp(
                self.sensor_data.values().next().map_or(0, |r| r.timestamp)
            )
        ));
    }
    
    fn render_history(&mut self, ui: &mut egui::Ui) {
        ui.heading("Sensor History");
        
        ui.horizontal(|ui| {
            ui.label("Select sensor:");
            egui::ComboBox::from_id_source("sensor_selector")
                .selected_text(model::sensor_types::get_display_name(&self.selected_sensor))
                .show_ui(ui, |ui| {
                    for sensor_type in [
                        model::sensor_types::TEMPERATURE,
                        model::sensor_types::HUMIDITY,
                        model::sensor_types::WATER_LEVEL,
                        model::sensor_types::PH,
                        model::sensor_types::SALINITY,
                        model::sensor_types::RAIN,
                        model::sensor_types::SOIL_MOISTURE,
                    ] {
                        let display_name = model::sensor_types::get_display_name(sensor_type);
                        if ui.selectable_label(self.selected_sensor == sensor_type, display_name).clicked() {
                            self.selected_sensor = sensor_type.to_string();
                            self.load_history();
                        }
                    }
                });
        });
        
        if self.is_loading {
            ui.spinner();
            ui.label("Loading data...");
            return;
        }
        
        if self.sensor_history.is_empty() {
            ui.label("No history data available for the selected sensor.");
            return;
        }
        
        let unit = model::sensor_types::get_unit(&self.selected_sensor);
        let display_name = model::sensor_types::get_display_name(&self.selected_sensor);
        
        // Chuẩn bị dữ liệu cho biểu đồ
        let points: PlotPoints = self.sensor_history.iter()
            .map(|reading| [reading.timestamp as f64, reading.value as f64])
            .collect();
        
        let line = Line::new(points)
            .name(display_name)
            .width(2.0);
        
        Plot::new("history_plot")
            .height(300.0)
            .show_x_axis(true)
            .show_y_axis(true)
            .allow_zoom(true)
            .allow_drag(true)
            .y_axis_label(format!("{} ({})", display_name, unit))
            .x_axis_label("Time")
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
        
        ui.add_space(20.0);
        
        // Hiển thị dữ liệu trong bảng
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("history_grid")
                .striped(true)
                .spacing([40.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Time");
                    ui.label("Value");
                    ui.label("Status");
                    ui.end_row();
                    
                    for reading in &self.sensor_history {
                        let time_str = util::date_converter::format_timestamp(reading.timestamp);
                        
                        ui.label(time_str);
                        
                        let value_text = if self.selected_sensor == model::sensor_types::RAIN {
                            if reading.value > 0.5 { "Yes" } else { "No" }.to_string()
                        } else {
                            format!("{:.1} {}", reading.value, unit)
                        };
                        
                        ui.label(value_text);
                        
                        let status_text = if reading.is_alert {
                            "⚠ Alert"
                        } else {
                            "✓ Normal"
                        };
                        
                        let status_color = if reading.is_alert {
                            egui::Color32::from_rgb(255, 100, 100)
                        } else {
                            egui::Color32::from_rgb(100, 255, 100)
                        };
                        
                        ui.colored_label(status_color, status_text);
                        
                        ui.end_row();
                    }
                });
        });
    }
    
    fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        
        ui.add_space(20.0);
        ui.label("ESP32 Connection Settings");
        
        ui.horizontal(|ui| {
            ui.label("ESP32 URL:");
            let response = ui.text_edit_singleline(&mut self.esp32_url);
            
            if response.changed() {
                // Lưu URL khi thay đổi
                if let Err(e) = util::preferences::save_esp32_url(&self.esp32_url) {
                    self.error_message = Some(format!("Failed to save settings: {}", e));
                }
            }
        });
        
        ui.add_space(10.0);
        if ui.button("Test Connection").clicked() {
            // Kiểm tra kết nối đến ESP32
            match api::esp32_api::fetch_data_from_esp32(&self.esp32_url) {
                Ok(_) => {
                    ui.label("Connection successful");
                },
                Err(e) => {
                    self.error_message = Some(format!("Connection failed: {}", e));
                }
            }
        }
        
        ui.add_space(20.0);
        ui.label("Sensor Thresholds");
        ui.add_space(10.0);
        
        // Hiển thị cài đặt ngưỡng cho mỗi loại cảm biến
        for sensor_type in [
            model::sensor_types::TEMPERATURE,
            model::sensor_types::HUMIDITY,
            model::sensor_types::WATER_LEVEL,
            model::sensor_types::PH,
            model::sensor_types::SALINITY,
            model::sensor_types::SOIL_MOISTURE,
        ] {
            self.render_threshold_settings(ui, sensor_type);
        }
        
        ui.add_space(20.0);
        if ui.button("Delete All Data").clicked() {
            // Hiển thị hộp thoại xác nhận
            self.error_message = Some("Are you sure you want to delete all data? This action cannot be undone.".to_string());
        }
    }
    
    fn render_threshold_settings(&mut self, ui: &mut egui::Ui, sensor_type: &str) {
        let display_name = model::sensor_types::get_display_name(sensor_type);
        let unit = model::sensor_types::get_unit(sensor_type);
        
        // Lấy ngưỡng hiện tại
        let threshold_result = || -> anyhow::Result<model::sensor_data::SensorThreshold> {
            let result = repository::sensor_repository::get_threshold(sensor_type)?;
            let (min_value, max_value) = model::sensor_types::get_default_threshold(sensor_type);
            
            Ok(model::sensor_data::SensorThreshold {
                sensor_type: sensor_type.to_string(),
                min_value,
                max_value: result,
            })
        };
        
        match threshold_result() {
            Ok(mut threshold) => {
                ui.collapsing(format!("{} Threshold", display_name), |ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("Max value ({}): ", unit));
                        let mut value = threshold.max_value;
                        if ui.add(egui::DragValue::new(&mut value).speed(0.1)).changed() {
                            threshold.max_value = value;
                            // Lưu ngưỡng
                            if let Err(e) = repository::sensor_repository::set_threshold(sensor_type, value) {
                                self.error_message = Some(format!("Failed to save threshold: {}", e));
                            }
                        }
                    });
                });
            },
            Err(e) => {
                ui.label(format!("Failed to load threshold for {}: {}", display_name, e));
            }
        }
    }
} 
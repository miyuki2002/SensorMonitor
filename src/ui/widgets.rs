use eframe::egui;
use egui::Color32;
use crate::model::sensor_data::SensorReading;
use crate::model::sensor_types;
use crate::util::date_converter;

/// Sensor card widget for the dashboard
pub fn sensor_card(ui: &mut egui::Ui, reading: &SensorReading) -> egui::Response {
    let sensor_type = &reading.sensor_type;
    let display_name = sensor_types::get_display_name(sensor_type);
    let unit = sensor_types::get_unit(sensor_type);
    
    // Create frame for the card
    let frame = egui::Frame::group(ui.style())
        .fill(ui.style().visuals.window_fill)
        .inner_margin(10.0)
        .outer_margin(5.0)
        .stroke(egui::Stroke::new(1.0, ui.style().visuals.widgets.noninteractive.fg_stroke.color));
    
    // Card content
    frame.show(ui, |ui| {
        ui.vertical(|ui| {
            // Display sensor name
            ui.heading(display_name);
            
            ui.add_space(5.0);
            
            // Display sensor value with appropriate formatting
            let value_text = if sensor_type == sensor_types::RAIN {
                if reading.value > 0.5 { "Yes" } else { "No" }.to_string()
            } else {
                format!("{:.1} {}", reading.value, unit)
            };
            
            ui.label(egui::RichText::new(value_text).size(24.0));
            
            ui.add_space(5.0);
            
            // Display status indicator
            let (status_text, status_color) = if reading.is_alert {
                ("⚠ Alert", Color32::from_rgb(255, 100, 100))
            } else {
                ("✓ Normal", Color32::from_rgb(100, 220, 100))
            };
            
            ui.label(egui::RichText::new(status_text).color(status_color));
            
            ui.add_space(5.0);
            
            // Display timestamp
            ui.label(format!("Updated: {}", date_converter::format_time(reading.timestamp)));
        });
    }).response
}

/// Alert badge for sensor readings that exceed thresholds
pub fn alert_badge(ui: &mut egui::Ui, count: usize) -> egui::Response {
    if count == 0 {
        return ui.label("");
    }
    
    let frame = egui::Frame::none()
        .fill(Color32::from_rgb(220, 50, 50))
        .rounding(egui::Rounding::same(10.0))
        .inner_margin(egui::vec2(8.0, 4.0));
    
    frame.show(ui, |ui| {
        ui.label(egui::RichText::new(format!("{}", count)).color(Color32::WHITE));
    }).response
}

/// Value indicator with color coding based on threshold
pub fn value_indicator(ui: &mut egui::Ui, value: f32, min: f32, max: f32) -> egui::Response {
    let normalized = if max > min {
        (value - min) / (max - min)
    } else {
        0.5
    };
    
    let clamped = normalized.clamp(0.0, 1.0);
    
    let color = if value < min || value > max {
        // Red for out of bounds
        Color32::from_rgb(220, 50, 50)
    } else {
        // Gradient from green to yellow to red
        let r = if clamped < 0.5 { (clamped * 2.0 * 255.0) as u8 } else { 255 };
        let g = if clamped > 0.5 { ((1.0 - clamped) * 2.0 * 255.0) as u8 } else { 255 };
        Color32::from_rgb(r, g, 50)
    };
    
    let mut response = ui.allocate_response(egui::vec2(100.0, 20.0), egui::Sense::hover());
    
    if ui.is_rect_visible(response.rect) {
        let painter = ui.painter();
        
        // Background
        painter.rect_filled(
            response.rect,
            egui::Rounding::same(4.0),
            ui.style().visuals.widgets.noninteractive.bg_fill,
        );
        
        // Value indicator
        let indicator_width = response.rect.width() * clamped;
        let indicator_rect = egui::Rect::from_min_size(
            response.rect.min,
            egui::vec2(indicator_width, response.rect.height()),
        );
        
        painter.rect_filled(
            indicator_rect,
            egui::Rounding::same(4.0),
            color,
        );
        
        // Border
        painter.rect_stroke(
            response.rect,
            egui::Rounding::same(4.0),
            egui::Stroke::new(1.0, ui.style().visuals.widgets.noninteractive.bg_stroke.color),
        );
    }
    
    response
} 
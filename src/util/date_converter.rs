use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};

/// Convert timestamp to local date time string
pub fn format_timestamp(timestamp_ms: i64) -> String {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp_ms / 1000, 0).unwrap_or_default();
    let datetime: DateTime<Utc> = Utc.from_utc_datetime(&naive);
    let local_time = datetime.with_timezone(&Local);
    local_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Convert timestamp to date-only string
pub fn format_date(timestamp_ms: i64) -> String {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp_ms / 1000, 0).unwrap_or_default();
    let datetime: DateTime<Utc> = Utc.from_utc_datetime(&naive);
    let local_time = datetime.with_timezone(&Local);
    local_time.format("%Y-%m-%d").to_string()
}

/// Convert timestamp to time-only string
pub fn format_time(timestamp_ms: i64) -> String {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp_ms / 1000, 0).unwrap_or_default();
    let datetime: DateTime<Utc> = Utc.from_utc_datetime(&naive);
    let local_time = datetime.with_timezone(&Local);
    local_time.format("%H:%M:%S").to_string()
}

/// Get current timestamp in milliseconds
pub fn current_timestamp() -> i64 {
    Utc::now().timestamp_millis()
} 
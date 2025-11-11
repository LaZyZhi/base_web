use core::fmt;

use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, Timelike, Utc};
use tracing_subscriber::fmt::time::FormatTime;

// 自定义时间格式化器，用于tracing日志
#[derive(Debug, Clone, Copy, Default)]
pub struct LocalTimeWithMillis;

impl FormatTime for LocalTimeWithMillis {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> fmt::Result {
        // 获取本地时间
        let now = Local::now();
        // 格式化为 yyyy-mm-dd hh:mm:ss.sss
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

/// 获取本地当前时间 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn now_local_timer() -> String {
    return Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
}

/// 获取UTC当前时间 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn now_utc_timer() -> String {
    return Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
}


/// 获取本地当前时间 yyyy-mm-dd hh:mm:ss.sss
#[allow(dead_code)]
pub async fn now_local_timer_with_millis() -> String {
    return Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
}

/// 获取UTC当前时间 yyyy-mm-dd hh:mm:ss.sss
#[allow(dead_code)]
pub async fn now_utc_timer_with_millis() -> String {
    return Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
}

/// 获取本地当前日期 yyyy-mm-dd
#[allow(dead_code)]
pub async fn now_local_date() -> String {
    return Local::now().format("%Y-%m-%d").to_string();
}

/// 获取UTC当前日期 yyyy-mm-dd
#[allow(dead_code)]
pub async fn now_utc_date() -> String {
    return Utc::now().format("%Y-%m-%d").to_string();
}

/// 获取本地当前时间戳（秒）
#[allow(dead_code)]
pub async fn now_local_timestamp() -> i64 {
    return Local::now().timestamp();
}

/// 获取UTC当前时间戳（秒）
#[allow(dead_code)]
pub async fn now_utc_timestamp() -> i64 {
    return Utc::now().timestamp();
}

/// 获取本地当前时间戳（毫秒）
#[allow(dead_code)]
pub async fn now_local_timestamp_millis() -> i64 {
    return Local::now().timestamp_millis();
}

/// 获取UTC当前时间戳（毫秒）
#[allow(dead_code)]
pub async fn now_utc_timestamp_millis() -> i64 {
    return Utc::now().timestamp_millis();
}

/// 获取本地当前年份
#[allow(dead_code)]
pub async fn now_local_year() -> i32 {
    return Local::now().year();
}

/// 获取UTC当前年份
#[allow(dead_code)]
pub async fn now_utc_year() -> i32 {
    return Utc::now().year();
}

/// 获取本地当前月份 (1-12)
#[allow(dead_code)]
pub async fn now_local_month() -> u32 {
    return Local::now().month();
}

/// 获取UTC当前月份 (1-12)
#[allow(dead_code)]
pub async fn now_utc_month() -> u32 {
    return Utc::now().month();
}

/// 获取本地当前日 (1-31)
#[allow(dead_code)]
pub async fn now_local_day() -> u32 {
    return Local::now().day();
}

/// 获取UTC当前日 (1-31)
#[allow(dead_code)]
pub async fn now_utc_day() -> u32 {
    return Utc::now().day();
}

/// 获取本地当前星期几 (1=周一, 7=周日)
#[allow(dead_code)]
pub async fn now_local_weekday() -> u32 {
    return Local::now().weekday().number_from_monday();
}

/// 获取UTC当前星期几 (1=周一, 7=周日)
#[allow(dead_code)]
pub async fn now_utc_weekday() -> u32 {
    return Utc::now().weekday().number_from_monday();
}

/// 获取本地当前小时 (0-23)
#[allow(dead_code)]
pub async fn now_local_hour() -> u32 {
    return Local::now().hour();
}

/// 获取UTC当前小时 (0-23)
#[allow(dead_code)]
pub async fn now_utc_hour() -> u32 {
    return Utc::now().hour();
}

/// 获取本地当前分钟 (0-59)
#[allow(dead_code)]
pub async fn now_local_minute() -> u32 {
    return Local::now().minute();
}

/// 获取UTC当前分钟 (0-59)
#[allow(dead_code)]
pub async fn now_utc_minute() -> u32 {
    return Utc::now().minute();
}

/// 获取本地当前秒 (0-59)
#[allow(dead_code)]
pub async fn now_local_second() -> u32 {
    return Local::now().second();
}

/// 获取UTC当前秒 (0-59)
#[allow(dead_code)]
pub async fn now_utc_second() -> u32 {
    return Utc::now().second();
}

/// 获取本地N天前的时间 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn local_days_ago(days: i64) -> String {
    let now = Local::now();
    let past = now - Duration::days(days);
    return past.format("%Y-%m-%d %H:%M:%S").to_string();
}

/// 获取UTC N天前的时间 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn utc_days_ago(days: i64) -> String {
    let now = Utc::now();
    let past = now - Duration::days(days);
    return past.format("%Y-%m-%d %H:%M:%S").to_string();
}

/// 获取本地N天后的时间 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn local_days_after(days: i64) -> String {
    let now = Local::now();
    let future = now + Duration::days(days);
    return future.format("%Y-%m-%d %H:%M:%S").to_string();
}

/// 获取UTC N天后的时间 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn utc_days_after(days: i64) -> String {
    let now = Utc::now();
    let future = now + Duration::days(days);
    return future.format("%Y-%m-%d %H:%M:%S").to_string();
}

/// 获取本地N小时前的时间 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn local_hours_ago(hours: i64) -> String {
    let now = Local::now();
    let past = now - Duration::hours(hours);
    return past.format("%Y-%m-%d %H:%M:%S").to_string();
}

/// 获取UTC N小时前的时间 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn utc_hours_ago(hours: i64) -> String {
    let now = Utc::now();
    let past = now - Duration::hours(hours);
    return past.format("%Y-%m-%d %H:%M:%S").to_string();
}

/// 获取本地N小时后的时间 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn local_hours_after(hours: i64) -> String {
    let now = Local::now();
    let future = now + Duration::hours(hours);
    return future.format("%Y-%m-%d %H:%M:%S").to_string();
}

/// 获取UTC N小时后的时间 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn utc_hours_after(hours: i64) -> String {
    let now = Utc::now();
    let future = now + Duration::hours(hours);
    return future.format("%Y-%m-%d %H:%M:%S").to_string();
}

/// 获取本地N分钟前的时间 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn local_minutes_ago(minutes: i64) -> String {
    let now = Local::now();
    let past = now - Duration::minutes(minutes);
    return past.format("%Y-%m-%d %H:%M:%S").to_string();
}

/// 获取UTC N分钟前的时间 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn utc_minutes_ago(minutes: i64) -> String {
    let now = Utc::now();
    let past = now - Duration::minutes(minutes);
    return past.format("%Y-%m-%d %H:%M:%S").to_string();
}

/// 获取本地N分钟后的时间 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn local_minutes_after(minutes: i64) -> String {
    let now = Local::now();
    let future = now + Duration::minutes(minutes);
    return future.format("%Y-%m-%d %H:%M:%S").to_string();
}

/// 获取UTC N分钟后的时间 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn utc_minutes_after(minutes: i64) -> String {
    let now = Utc::now();
    let future = now + Duration::minutes(minutes);
    return future.format("%Y-%m-%d %H:%M:%S").to_string();
}

/// 格式化本地时间戳为字符串 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn format_local_timestamp(timestamp: i64) -> String {
    let dt = DateTime::from_timestamp(timestamp, 0).unwrap();
    let local_dt = dt.with_timezone(&Local);
    return local_dt.format("%Y-%m-%d %H:%M:%S").to_string();
}

/// 格式化UTC时间戳为字符串 yyyy-mm-dd hh:mm:ss
#[allow(dead_code)]
pub async fn format_utc_timestamp(timestamp: i64) -> String {
    let dt = DateTime::from_timestamp(timestamp, 0).unwrap();
    return dt.format("%Y-%m-%d %H:%M:%S").to_string();
}

/// 解析时间字符串为本地时间戳（支持 yyyy-mm-dd hh:mm:ss）
#[allow(dead_code)]
pub async fn parse_local_timestamp(time_str: &str) -> Option<i64> {
    let format = "%Y-%m-%d %H:%M:%S";
    if let Ok(dt) = DateTime::parse_from_str(time_str, format) {
        return Some(dt.timestamp());
    }
    None
}

/// 解析时间字符串为UTC时间戳（支持 yyyy-mm-dd hh:mm:ss）
#[allow(dead_code)]
pub async fn parse_utc_timestamp(time_str: &str) -> Option<i64> {
    let format = "%Y-%m-%d %H:%M:%S";
    if let Ok(dt) = NaiveDate::parse_from_str(time_str, format) {
        if let Some(dt) = dt.and_hms_opt(0, 0, 0) {
            return Some(dt.and_utc().timestamp());
        }
    }
    None
}

/// 获取本周一的本地日期 yyyy-mm-dd
#[allow(dead_code)]
pub async fn local_monday_date() -> String {
    let now = Local::now();
    let days_since_monday = now.weekday().num_days_from_monday();
    let monday = now - Duration::days(days_since_monday as i64);
    return monday.format("%Y-%m-%d").to_string();
}

/// 获取本周一的UTC日期 yyyy-mm-dd
#[allow(dead_code)]
pub async fn utc_monday_date() -> String {
    let now = Utc::now();
    let days_since_monday = now.weekday().num_days_from_monday();
    let monday = now - Duration::days(days_since_monday as i64);
    return monday.format("%Y-%m-%d").to_string();
}

/// 获取本月第一天的本地日期 yyyy-mm-dd
#[allow(dead_code)]
pub async fn local_first_day_of_month() -> String {
    let now = Local::now();
    let first_day = now.with_day(1).unwrap();
    return first_day.format("%Y-%m-%d").to_string();
}

/// 获取本月第一天的UTC日期 yyyy-mm-dd
#[allow(dead_code)]
pub async fn utc_first_day_of_month() -> String {
    let now = Utc::now();
    let first_day = now.with_day(1).unwrap();
    return first_day.format("%Y-%m-%d").to_string();
}

/// 获取本月最后一天的本地日期 yyyy-mm-dd
#[allow(dead_code)]
pub async fn local_last_day_of_month() -> String {
    let now = Local::now();
    let next_month = now.with_month(now.month() + 1).unwrap_or(now.with_month(1).unwrap().with_year(now.year() + 1).unwrap());
    let last_day = next_month.with_day(1).unwrap() - Duration::days(1);
    return last_day.format("%Y-%m-%d").to_string();
}

/// 获取本月最后一天的UTC日期 yyyy-mm-dd
#[allow(dead_code)]
pub async fn utc_last_day_of_month() -> String {
    let now = Utc::now();
    let next_month = now.with_month(now.month() + 1).unwrap_or(now.with_month(1).unwrap().with_year(now.year() + 1).unwrap());
    let last_day = next_month.with_day(1).unwrap() - Duration::days(1);
    return last_day.format("%Y-%m-%d").to_string();
}

/// 计算两个时间戳之间的天数差（本地时间）
#[allow(dead_code)]
pub async fn days_between_local(start_timestamp: i64, end_timestamp: i64) -> i64 {
    let start_dt = DateTime::from_timestamp(start_timestamp, 0).unwrap();
    let end_dt = DateTime::from_timestamp(end_timestamp, 0).unwrap();
    let duration = end_dt - start_dt;
    return duration.num_days();
}

/// 计算两个时间戳之间的天数差（UTC时间）
#[allow(dead_code)]
pub async fn days_between_utc(start_timestamp: i64, end_timestamp: i64) -> i64 {
    let start_dt = DateTime::from_timestamp(start_timestamp, 0).unwrap();
    let end_dt = DateTime::from_timestamp(end_timestamp, 0).unwrap();
    let duration = end_dt - start_dt;
    return duration.num_days();
}

/// 检查时间字符串是否是有效的 yyyy-mm-dd hh:mm:ss 格式
#[allow(dead_code)]
pub async fn is_valid_time_format(time_str: &str) -> bool {
    let format = "%Y-%m-%d %H:%M:%S";
    DateTime::parse_from_str(time_str, format).is_ok()
}

/// 获取当前季度 (1-4)
#[allow(dead_code)]
pub async fn now_quarter() -> u32 {
    let month = Utc::now().month();
    return (month - 1) / 3 + 1;
}

/// 获取当前是否在营业时间（9:00-18:00）
#[allow(dead_code)]
pub async fn is_business_hours() -> bool {
    let now = Local::now();
    let hour = now.hour();
    return hour >= 9 && hour < 18;
}

/// 获取当前是否工作日（周一至周五）
#[allow(dead_code)]
pub async fn is_weekday() -> bool {
    let weekday = Local::now().weekday();
    return weekday != chrono::Weekday::Sat && weekday != chrono::Weekday::Sun;
}
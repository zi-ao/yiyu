use std::env;
use time::{format_description, UtcOffset};
use tracing_appender::{
    non_blocking,
    non_blocking::{NonBlocking, WorkerGuard},
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::fmt::{
    format::{Format, Json, JsonFields, Pretty},
    layer as fmt_layer,
    time::OffsetTime,
    Layer,
};

/// 创建自定义的时间格式器
///
/// timer("[year]-[month]-[day] [hour]:[minute]:[second]");
pub fn timer(s: &str) -> OffsetTime<Vec<format_description::BorrowedFormatItem>> {
    // 定义自定义时间格式
    let custom_format = format_description::parse(s).expect("Invalid time format");
    // 获取当前时区偏移量
    let time_offset = UtcOffset::current_local_offset().unwrap_or_else(|_| UtcOffset::UTC);

    OffsetTime::new(time_offset, custom_format)
}

pub fn text_layer<S>() -> Layer<S, Pretty, Format<Pretty>> {
    fmt_layer().pretty()
}

/// 拥有 JSON 格式器的 Layer
///
/// json_layer();
pub fn json_layer<S>() -> Layer<S, JsonFields, Format<Json>> {
    fmt_layer()
        .json()
        .with_ansi(false)
        .with_span_list(true)
        .with_file(true)
        .with_line_number(true)
}

/// 创建日志文件附加器
///
/// file_appender(Rotation::NEVER, "log", "app", "log")
pub fn file_appender(
    rotation: Rotation,
    log_dir: &str,
    file_prefix: &str,
    file_suffix: &str,
) -> (NonBlocking, WorkerGuard) {
    let file_appender = RollingFileAppender::builder()
        .rotation(rotation)
        .filename_prefix(file_prefix)
        .filename_suffix(file_suffix)
        .build(log_dir)
        .expect("Initializing rolling file appender failed");

    non_blocking(file_appender)
}

/// 创建默认日志文件附加器
///
/// default_file_appender()
pub fn default_file_appender() -> (NonBlocking, WorkerGuard) {
    let log_dir = env::var("LOG_DIR").unwrap_or_default();
    let log_rotation = env::var("LOG_ROTATION").unwrap();
    let file_prefix = env::var("LOG_FILE_PREFIX").unwrap();
    let file_suffix = env::var("LOG_FILE_SUFFIX").unwrap();

    file_appender(
        match log_rotation.as_str() {
            "minutely" => Rotation::MINUTELY,
            "hourly" => Rotation::HOURLY,
            "daily" => Rotation::DAILY,
            _ => Rotation::NEVER,
        },
        if log_dir != "" { &log_dir } else { "logs" },
        if file_prefix != "" {
            &file_prefix
        } else {
            "app"
        },
        if file_suffix != "" {
            &file_suffix
        } else {
            "log"
        },
    )
}

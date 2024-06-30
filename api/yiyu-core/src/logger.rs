use time::{format_description, UtcOffset};
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::{
    format::{Format, Json, JsonFields},
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

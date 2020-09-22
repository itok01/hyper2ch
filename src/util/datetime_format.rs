use chrono::prelude::*;

/// Format a weekday to Japanese
fn format_japanese_weekday(weekday: &Weekday) -> &str {
    match weekday {
        Weekday::Mon => "月",
        Weekday::Tue => "火",
        Weekday::Wed => "水",
        Weekday::Thu => "木",
        Weekday::Fri => "金",
        Weekday::Sat => "土",
        Weekday::Sun => "日",
    }
}

/// Gererate a datetime in Japanese format
pub fn format_japan_datetime(datetime: &DateTime<Local>) -> String {
    let datetime = datetime.with_timezone(&Local);

    format!(
        "{}/{:02}/{:02}({}) {:02}:{:02}:{:02}.{}",
        datetime.year(),
        datetime.month(),
        datetime.day(),
        format_japanese_weekday(&datetime.weekday()),
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
        &datetime.nanosecond().to_string()[0..2]
    )
}

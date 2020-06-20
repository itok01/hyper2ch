use chrono::prelude::*;

/// 曜日の列挙型を日本語に変換
fn format_japan_weekday(weekday: &Weekday) -> &str {
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

/// 日本語の日付の文字列を生成
pub fn format_japan_datetime(datetime: &DateTime<Utc>) -> String {
    let datetime = datetime.with_timezone(&Local);

    format!(
        "{}/{:02}/{:02}({}) {:02}:{:02}:{:02}.{}",
        datetime.year(),
        datetime.month(),
        datetime.day(),
        format_japan_weekday(&datetime.weekday()),
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
        &datetime.nanosecond().to_string()[0..2]
    )
}

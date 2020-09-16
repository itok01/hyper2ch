use bytes::Bytes;
use percent_encoding::percent_decode_str;
use std::collections::HashMap;

use crate::util::shift_jis_bytes_to_string;

/// Shift_JISのフォームデータを読み込む
pub fn parse_shift_jis_formdata<S: Into<String>>(raw_data: S) -> HashMap<String, String> {
    let data_string: String = raw_data.into();

    let splited_data = data_string.split("&");

    let mut data_map: HashMap<String, String> = HashMap::new();

    for pair in splited_data {
        let p: Vec<&str> = pair.split("=").collect();

        let key: Bytes = percent_decode_str(p[0]).collect();
        let key = shift_jis_bytes_to_string(key);

        let value: Bytes = percent_decode_str(p[1]).collect();
        let value = shift_jis_bytes_to_string(value);

        data_map.entry(key).or_insert(value);
    }

    data_map
}

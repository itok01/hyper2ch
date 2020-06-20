use bytes::Bytes;
use encoding_rs::SHIFT_JIS;

/// 文字列をShift_JISのバイト列に変換
pub fn convert_to_shift_jis<S: Into<String>>(s: S) -> Bytes {
    let s = s.into();
    let e = SHIFT_JIS.encode(s.as_str());
    Bytes::from(e.0.to_vec())
}

/// Shift_JISのバイト列をStringに変換
pub fn shift_jis_bytes_to_string(b: Bytes) -> String {
    let decoded_data = SHIFT_JIS.decode(&b);
    String::from(decoded_data.0)
}

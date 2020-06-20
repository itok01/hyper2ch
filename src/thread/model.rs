use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/// レスのデータ構造
#[derive(Deserialize, Serialize)]
pub struct MessageData {
    pub name: String,
    pub mail: String,
    pub date: DateTime<Utc>,
    pub uid: String,
    pub message: String,
    pub subject: String,
}

/// スレッドのデータ構造
#[derive(Deserialize, Serialize)]
pub struct Thread {
    pub subject: String,
    pub key: i64,
    pub message: Box<Vec<MessageData>>,
    pub hidden: bool,
}

/// スレッドの情報
#[derive(Deserialize, Serialize)]
pub struct ThreadInfo {
    pub subject: String,
    pub key: i64,
    pub message_count: i64,
    pub hidden: bool,
}

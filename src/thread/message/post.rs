use actix_web::{Error, HttpResponse};
use bytes::Bytes;
use chrono::prelude::*;
use mongodb::bson;
use serde::Deserialize;
use std::collections::HashMap;

use crate::database::connect_database;
use crate::encoding::shift_jis_bytes_to_string;
use crate::thread::{MessageData, Thread};
use crate::util::parse_shift_jis_formdata;

/// post_message_handlerが受け取るフォームデータ
#[derive(Deserialize)]
pub struct PostMessageData {
    subject: Option<String>,
    #[serde(rename = "FROM")]
    from: String,
    mail: String,
    #[serde(rename = "MESSAGE")]
    message: String,
    bbs: String,
    key: Option<i64>,
}

impl PostMessageData {
    pub fn from_hashmap(
        data: HashMap<String, String>,
    ) -> Result<PostMessageData, Box<dyn std::error::Error>> {
        Ok(PostMessageData {
            subject: match data.get("subject") {
                Some(s) => Option::from(s.to_owned()),
                None => None,
            },
            from: data.get("FROM").unwrap().to_owned(),
            mail: data.get("mail").unwrap().to_owned(),
            message: data.get("MESSAGE").unwrap().to_owned(),
            bbs: data.get("bbs").unwrap().to_owned(),
            key: match data.get("key") {
                Some(s) => Option::from(s.parse::<i64>().unwrap()),
                None => None,
            },
        })
    }
}

/// レスの書き込み
pub async fn post_message_handler(b: Bytes) -> Result<HttpResponse, Error> {
    let data = shift_jis_bytes_to_string(b);
    let data = parse_shift_jis_formdata(data);
    let message_data = PostMessageData::from_hashmap(data)
        .unwrap_or_else(|_| panic!("データの形式が間違えています。"));

    match post_message(message_data).await {
        Ok(_) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=Shift_JIS")
            .finish()),
        Err(_) => Ok(HttpResponse::BadRequest()
            .content_type("text/html; charset=Shift_JIS")
            .finish()),
    }
}

/// データベースにレスを書き込む
pub async fn post_message(message: PostMessageData) -> Result<(), Box<dyn std::error::Error>> {
    let db = connect_database().await?;

    let collection = db.collection(&format!("bbs.{}", message.bbs));

    match message.key {
        Some(key) => {
            append_message(collection, key, message).await?;
        }
        None => create_thread(collection, message).await?,
    }

    Ok(())
}

/// スレッドにレスを挿入
pub async fn append_message(
    collection: mongodb::Collection,
    key: i64,
    message: PostMessageData,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("append_thread: {:?}", message.from);
    println!("append_thread: {:?}", message.mail);
    println!("append_thread: {:?}", message.message);

    let m = MessageData {
        name: message.from,
        mail: message.mail,
        date: Utc::now(),
        uid: String::from("HELLO"),
        message: message.message,
        subject: String::new(),
    };

    let m_bson = bson::to_bson(&m)?;

    let update = bson::doc! {
        "$push": {
            "message": m_bson
        }
    };

    let filter = bson::doc! { "key": key };
    collection.update_one(filter, update, None).await?;

    Ok(())
}

/// スレッドの作成
pub async fn create_thread(
    collection: mongodb::Collection,
    message: PostMessageData,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("create_thread: {:?}", message.subject);
    println!("create_thread: {:?}", message.from);
    println!("create_thread: {:?}", message.mail);
    println!("create_thread: {:?}", message.message);
    match message.subject {
        Some(subject) => {
            let m = vec![MessageData {
                name: message.from,
                mail: message.mail,
                date: Utc::now(),
                uid: String::from("HELLO"),
                message: message.message,
                subject: subject.clone(),
            }];

            let key = collection.count_documents(None, None).await?;

            let t = Thread {
                key,
                subject: subject,
                message: Box::from(m),
                hidden: false,
            };

            let t_bson = bson::to_bson(&t)?;
            let t_doc = t_bson.as_document().unwrap().to_owned();

            collection.insert_one(t_doc, None).await?;
        }
        None => panic!("スレタイがありません。"),
    }

    Ok(())
}

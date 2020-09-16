use crate::models::{get_id_by_bbs_path_name, Message, Thread};
use crate::util::{parse_shift_jis_formdata, shift_jis_bytes_to_string};
use actix_web::{http::header, post, HttpRequest, HttpResponse, Responder};
use bytes::Bytes;
use chrono::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

/// Post message to thread
#[post("/test/bbs.cgi")]
pub async fn post_message_handler(b: Bytes, req: HttpRequest) -> impl Responder {
    let data = shift_jis_bytes_to_string(b);
    println!("{:?}", data);
    let data = parse_shift_jis_formdata(data);
    let message_data = PostMessageData::from_hashmap(data).unwrap();
    let connection_info = req.connection_info();
    let headers = req.headers();

    match message_data.thread_id {
        Some(thread_id) => {
            Message::create(
                thread_id,
                &message_data.user_name,
                &message_data.user_email,
                "NULL",
                connection_info.realip_remote_addr().unwrap(),
                connection_info.host(),
                headers.get(header::USER_AGENT).unwrap().to_str().unwrap(),
                Utc::now().with_timezone(&Utc::now().timezone().fix()),
                &message_data.text,
            )
            .await
            .unwrap();
        }
        None => {
            let thread_title = message_data.thread_title.unwrap();
            let bbs_id = get_id_by_bbs_path_name(&message_data.bbs_path_name)
                .await
                .unwrap();

            let thread = Thread::create(bbs_id, &thread_title).await.unwrap();
            Message::create(
                thread.id,
                &message_data.user_name,
                &message_data.user_email,
                "NULL",
                connection_info.realip_remote_addr().unwrap(),
                connection_info.host(),
                headers.get(header::USER_AGENT).unwrap().to_str().unwrap(),
                Utc::now().with_timezone(&Utc::now().timezone().fix()),
                &message_data.text,
            )
            .await
            .unwrap();
        }
    }

    HttpResponse::Ok().finish()
}

/// Form data model that post_message_handler receive
#[derive(Deserialize, Debug)]
pub struct PostMessageData {
    #[serde(rename = "subject")]
    thread_title: Option<String>,
    #[serde(rename = "FROM")]
    user_name: String,
    #[serde(rename = "mail")]
    user_email: String,
    #[serde(rename = "MESSAGE")]
    text: String,
    #[serde(rename = "bbs")]
    bbs_path_name: String,
    #[serde(rename = "key")]
    thread_id: Option<i64>,
}

impl PostMessageData {
    pub fn from_hashmap(
        data: HashMap<String, String>,
    ) -> Result<PostMessageData, Box<dyn std::error::Error>> {
        Ok(PostMessageData {
            thread_title: match data.get("subject") {
                Some(s) => Option::from(s.to_owned()),
                None => None,
            },
            user_name: data.get("FROM").unwrap().to_owned(),
            user_email: data.get("mail").unwrap().to_owned(),
            text: data.get("MESSAGE").unwrap().to_owned(),
            bbs_path_name: data.get("bbs").unwrap().to_owned(),
            thread_id: match data.get("key") {
                Some(s) => Option::from(s.parse::<i64>().unwrap()),
                None => None,
            },
        })
    }
}

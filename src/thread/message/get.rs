use actix_web::{web, Error, HttpResponse};
use htmlescape::encode_minimal;
use mongodb::bson;
use serde::Deserialize;

use crate::database::connect_database;
use crate::encoding;
use crate::thread::Thread;
use crate::util::format_japan_datetime;

/// get_dat_handlerが受け取るパラメータ
#[derive(Deserialize)]
pub struct GetMessagesParams {
    bbs: String,
    key: i64,
}

/// datの取得
pub async fn get_dat_handler(params: web::Path<GetMessagesParams>) -> Result<HttpResponse, Error> {
    match get_thread(&params.bbs, params.key).await {
        Some(thread) => {
            let dat = generate_dat(thread);
            let encoded_dat = encoding::convert_to_shift_jis(dat);

            Ok(HttpResponse::Ok()
                .content_type("text/plain; charset=Shift_JIS")
                .body(encoded_dat))
        }
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

/// datをVec<Message>から生成
fn generate_dat(mut thread: Thread) -> String {
    let mut dat = String::new();

    thread.message.sort_by_key(|m| m.date);

    for (i, m) in thread.message.iter().enumerate() {
        let date = format_japan_datetime(&m.date);
        match i {
            0 => {
                let subject = encode_minimal(&m.subject);
                let subject = subject.replace("\n", "");

                dat.push_str(
                    format!(
                        "{}<>{}<>{} ID:{}<> {} <>{}\n",
                        m.name, m.mail, date, m.uid, m.message, subject
                    )
                    .as_str(),
                )
            }
            _ => dat.push_str(
                format!(
                    "{}<>{}<>{} ID:{}<> {} <>\n",
                    m.name, m.mail, date, m.uid, m.message
                )
                .as_str(),
            ),
        }
    }

    dat
}

/// スレッドのデータをデータベースから読み込む
pub async fn get_thread<S: Into<String>>(bbs: S, key: i64) -> Option<Thread> {
    let db = connect_database()
        .await
        .unwrap_or_else(|_| panic!("データベースへの接続に失敗しました。"));
    let collection = db.collection(format!("bbs.{}", bbs.into()).as_str());

    let filter: bson::Document = bson::doc! {"key": key};
    if let Some(thread_doc) = collection
        .find_one(filter, None)
        .await
        .unwrap_or_else(|_| panic!("メッセージの読み込みに失敗しました。"))
    {
        let thread: Thread = bson::from_bson(bson::Bson::Document(thread_doc))
            .unwrap_or_else(|_| panic!("メッセージの読み込みに失敗しました。"));

        return Some(thread);
    }

    None
}

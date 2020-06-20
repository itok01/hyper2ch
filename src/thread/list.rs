use actix_web::{web, Error, HttpResponse};
use htmlescape::encode_minimal;
use mongodb::bson;
use serde::Deserialize;
use tokio::stream::StreamExt;

use crate::database::connect_database;
use crate::encoding;
use crate::thread::{Thread, ThreadInfo};

/// get_thread_list_handlerが受け取るパラメータ
#[derive(Deserialize)]
pub struct GetThreadListParams {
    bbs: String,
}

/// スレッド一覧(subject.txt)をVec<Thread>から生成
fn generate_thread_list(thread: Vec<ThreadInfo>) -> String {
    let mut thread_list = String::new();

    for mut t in thread {
        t.subject = encode_minimal(&t.subject);
        t.subject = t.subject.replace("\n", "");

        thread_list
            .push_str(format!("{}.dat<>{} ({})\n", t.key, t.subject, t.message_count).as_str());
    }

    thread_list
}

/// スレッド一覧の取得
pub async fn get_thread_list_handler(
    params: web::Path<GetThreadListParams>,
) -> Result<HttpResponse, Error> {
    let thread_info = get_thread_info_list(&params.bbs).await;

    let thread_list = generate_thread_list(thread_info);

    let encoded_thread_list = encoding::convert_to_shift_jis(thread_list);

    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=Shift_JIS")
        .body(encoded_thread_list))
}

/// スレッド情報をデータベースから読み込む
pub async fn get_thread_info_list<S: Into<String>>(bbs: S) -> Vec<ThreadInfo> {
    let mut thread_info: Vec<ThreadInfo> = Vec::new();

    let db = connect_database()
        .await
        .unwrap_or_else(|_| panic!("データベースへの接続に失敗しました。"));
    let collection = db.collection(format!("bbs.{}", bbs.into()).as_str());

    let mut cursor = collection
        .find(None, None)
        .await
        .unwrap_or_else(|_| panic!("スレッドの読み込みに失敗しました。"));

    while let Some(thread_doc) = cursor.next().await {
        let thread_doc =
            thread_doc.unwrap_or_else(|_| panic!("スレッドの読み込みに失敗しました。"));

        let thread: Thread = bson::from_bson(bson::Bson::Document(thread_doc))
            .unwrap_or_else(|_| panic!("メッセージの読み込みに失敗しました。"));

        thread_info.push(ThreadInfo {
            subject: thread.subject,
            key: thread.key,
            message_count: thread.message.len() as i64,
            hidden: thread.hidden,
        });
    }

    thread_info
}

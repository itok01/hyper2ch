use crate::models::{
    get_id_by_bbs_path_name, get_message_count, get_thread_title, Message, Thread,
};
use crate::util::convert_to_shift_jis;
use crate::util::format_japan_datetime;
use actix_web::{get, web, HttpResponse, Responder};
use bytes::Bytes;
use htmlescape::encode_minimal;

#[get("/{bbs_path_name}/dat/{thread_id}.dat")]
pub async fn get_thread_dat_handler(
    web::Path((_, thread_id)): web::Path<(String, i64)>,
) -> impl Responder {
    println!("{}", thread_id);
    let messages = Message::find_messages_by_thread_id(thread_id)
        .await
        .unwrap();
    let title = get_thread_title(thread_id).await.unwrap();

    HttpResponse::Ok()
        .content_type("text/plain; charset=Shift_JIS")
        .body(generate_dat(&title, messages))
}

/// Generale thread dat for view messages
fn generate_dat(title: &str, messages: Vec<Message>) -> Bytes {
    let mut thread_dat = String::new();

    for (i, message) in messages.iter().enumerate() {
        let user_name = encode_minimal(&message.user_name);
        let user_email = encode_minimal(&message.user_email);
        let user_uid = encode_minimal(&message.user_uid);
        let text = encode_minimal(&message.text);
        let timestamp = format_japan_datetime(&message.timestamp);
        match i {
            0 => {
                let title = encode_minimal(title).replace("\n", "");

                thread_dat.push_str(
                    format!(
                        "{}<>{}<>{} ID:{}<> {} <>{}\n",
                        user_name, user_email, timestamp, user_uid, text, title
                    )
                    .as_str(),
                )
            }
            _ => thread_dat.push_str(
                format!(
                    "{}<>{}<>{} ID:{}<> {} <>\n",
                    user_name, user_email, timestamp, user_uid, text
                )
                .as_str(),
            ),
        }
    }

    convert_to_shift_jis(thread_dat)
}

#[get("/{bbs_path_name}/subject.txt")]
pub async fn get_subject_txt_handler(
    web::Path(bbs_path_name): web::Path<String>,
) -> impl Responder {
    let bbs_id = get_id_by_bbs_path_name(&bbs_path_name).await.unwrap();
    let threads = Thread::find_available_in_bbs(bbs_id).await.unwrap();

    match generate_subject_txt(threads).await {
        Some(subject_txt) => HttpResponse::Ok()
            .content_type("text/plain; charset=Shift_JIS")
            .body(subject_txt),
        None => HttpResponse::Ok()
            .content_type("text/plain; charset=Shift_JIS")
            .body("No thread"),
    }
}

/// Generale subject.txt for listing threads
async fn generate_subject_txt(threads: Vec<Thread>) -> Option<Bytes> {
    if threads.len() == 0 {
        return None;
    }

    let mut subject_txt = String::new();
    for thread in threads {
        let title = encode_minimal(&thread.title).replace("\n", "");
        subject_txt.push_str(
            format!(
                "{}.dat<>{} ({})\n",
                thread.id,
                title,
                get_message_count(thread.id).await.unwrap_or(0)
            )
            .as_str(),
        );
    }
    let subject_txt_sjis = convert_to_shift_jis(subject_txt);
    Option::from(subject_txt_sjis)
}

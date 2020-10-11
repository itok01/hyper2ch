use crate::models::Message;
use actix_web::{get, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Deserialize)]
pub struct GetMessagesRequest {
    thread: i64,
}

#[derive(Serialize)]
pub struct GetMessagesResponse {
    ok: bool,
    messages: Vec<MessageData>,
}

#[derive(Serialize)]
struct MessageData {
    id: i64,
    user_name: String,
    user_email: String,
    user_uid: String,
    timestamp: DateTime<Local>,
    text: String,
}

#[get("/api/messages")]
pub async fn get_messages_handler(request: web::Query<GetMessagesRequest>) -> impl Responder {
    let messages = Message::find_messages_by_thread_id(request.thread)
        .await
        .unwrap();

    HttpResponse::Ok().json({
        GetMessagesResponse {
            ok: true,
            messages: transform_messages(messages).await,
        }
    })
}

/// Transform threads for response
async fn transform_messages(messages: Vec<Message>) -> Vec<MessageData> {
    let mut transformed_threads: Vec<MessageData> = Vec::new();

    for message in messages {
        transformed_threads.push(MessageData {
            id: message.id,
            user_name: message.user_name,
            user_email: message.user_email,
            user_uid: message.user_uid,
            timestamp: message.timestamp,
            text: message.text,
        })
    }

    transformed_threads
}

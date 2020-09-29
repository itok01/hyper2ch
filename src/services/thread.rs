use crate::models::{get_id_by_bbs_path_name, get_message_count, Thread};
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Deserialize)]
pub struct GetThreadsRequest {
    bbs: String,
}

#[derive(Serialize)]
pub struct GetThreadsResponse {
    ok: bool,
    threads: Vec<ThreadData>,
}

#[derive(Serialize)]
struct ThreadData {
    id: i64,
    title: String,
    message_count: i64,
}

#[get("/api/threads")]
pub async fn get_threads_handler(request: web::Query<GetThreadsRequest>) -> impl Responder {
    let bbs_id = get_id_by_bbs_path_name(&request.bbs).await.unwrap();
    let threads = Thread::find_available_in_bbs(bbs_id).await.unwrap();

    HttpResponse::Ok().json({
        GetThreadsResponse {
            ok: true,
            threads: transform_threads(threads).await,
        }
    })
}

/// Transform threads for response
async fn transform_threads(threads: Vec<Thread>) -> Vec<ThreadData> {
    let mut transformed_threads: Vec<ThreadData> = Vec::new();

    for thread in threads {
        transformed_threads.push(ThreadData {
            id: thread.id,
            title: thread.title,
            message_count: get_message_count(thread.id).await.unwrap(),
        })
    }

    transformed_threads
}

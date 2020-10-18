use crate::models::Bbs;
use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;
use std::vec::Vec;

#[derive(Serialize)]
pub struct GetBbsListResponse {
    ok: bool,
    bbs_list: Vec<BbsData>,
}

#[derive(Serialize)]
struct BbsData {
    id: i64,
    name: String,
    path_name: String,
    description: String,
    category: String,
}

#[get("/api/bbs/list")]
pub async fn get_bbs_list_handler() -> impl Responder {
    let bbses = Bbs::find_shown().await.unwrap();

    HttpResponse::Ok().json({
        GetBbsListResponse {
            ok: true,
            bbs_list: transform_bbs_list(bbses),
        }
    })
}

/// Transform bbs list for response
fn transform_bbs_list(bbses: Vec<Bbs>) -> Vec<BbsData> {
    let mut transformed_threads: Vec<BbsData> = Vec::new();

    for bbs in bbses {
        transformed_threads.push(BbsData {
            id: bbs.id,
            name: bbs.name,
            path_name: bbs.path_name,
            description: bbs.description,
            category: bbs.category,
        })
    }

    transformed_threads
}

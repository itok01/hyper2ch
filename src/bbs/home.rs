use actix_web::{error, web, Error, HttpResponse};
use serde::Deserialize;

use crate::config::load_config;

/// get_bbs_handlerが受け取るパラメータ
#[derive(Deserialize)]
pub struct GetBbsParams {
    bbs: String,
}

/// 板のホーム
pub async fn get_bbs_handler(
    params: web::Path<GetBbsParams>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let my_config = load_config();

    let mut ctx = tera::Context::new();
    ctx.insert("title", &params.bbs);
    ctx.insert("site_name", &my_config.site_name);

    let rendered_html = tmpl
        .render("bbs.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered_html))
}

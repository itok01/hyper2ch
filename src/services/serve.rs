use super::legacy;
use crate::util::get_env;
use actix_web::{middleware, App, HttpServer};

/// Run HTTP server
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        // Logger
        std::env::set_var("RUST_LOG", "actix_web=info");
        env_logger::init();

        App::new()
            .wrap(middleware::Logger::default())
            .service(legacy::get_bbs_handler)
            .service(legacy::get_bbs_menu_handler)
            .service(legacy::get_subject_txt_handler)
            .service(legacy::get_thread_dat_handler)
            .service(legacy::post_message_handler)
    })
    .bind(format!("0.0.0.0:{}", get_env("BACKEND_PORT")))?
    .run()
    .await
}

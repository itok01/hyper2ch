use super::*;
use crate::util::get_env;
use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};

/// Run HTTP server
pub async fn run() -> std::io::Result<()> {
    // Logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_origin(&get_env("FRONTEND_ADDRESS"))
                    .finish(),
            )
            .service(legacy::get_bbs_handler)
            .service(legacy::get_bbs_menu_handler)
            .service(legacy::get_subject_txt_handler)
            .service(legacy::get_thread_dat_handler)
            .service(legacy::post_message_handler)
            .service(get_threads_handler)
            .service(get_messages_handler)
    })
    .bind(format!("0.0.0.0:{}", get_env("BACKEND_PORT")))?
    .run()
    .await
}

use actix_web::{middleware, web, App, HttpServer};
use tera::Tera;

mod bbs;
mod config;
mod database;
mod encoding;
mod thread;
mod util;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //データベースへの接続
    let db = database::connect_database()
        .await
        .unwrap_or_else(|_| panic!("データベースへの接続に失敗しま_した。"));

    // データベースの初期化
    database::database_init(db)
        .await
        .unwrap_or_else(|_| panic!("データベースの初期化に失敗しました。"));

    // 設定(config.json)の読み込み
    let my_config = config::load_config();

    // サーバー
    HttpServer::new(|| {
        // ロガーの準備
        std::env::set_var("RUST_LOG", "actix_web=info");
        env_logger::init();

        // HTMLテンプレートの準備
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        // ルーティング等
        App::new()
            // HTMLテンプレートの準備
            .data(tera)
            // パスのスラッシュをいい感じにする
            .wrap(middleware::NormalizePath)
            // ロガーの準備
            .wrap(middleware::Logger::default())
            // 板のホーム
            .service(web::resource("/{bbs}/").route(web::get().to(bbs::get_bbs_handler)))
            // スレ一覧
            .service(
                web::resource("/{bbs}/subject.txt")
                    .route(web::get().to(thread::get_thread_list_handler)),
            )
            // スレのdat
            .service(
                web::resource("/{bbs}/dat/{key}.dat").route(web::get().to(thread::get_dat_handler)),
            )
            // 書き込むためのエンドポイント
            .route(
                "/test/bbs.cgi",
                web::post().to(thread::post_message_handler),
            )
    })
    .bind(my_config.listen_host)?
    .run()
    .await
}

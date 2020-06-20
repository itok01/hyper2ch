use crate::config::load_config;

/// データベースに接続
pub async fn connect_database() -> Result<mongodb::Database, mongodb::error::Error> {
    // 設定を読み込む
    let config = load_config();

    // データベースサーバーに接続
    let database_uri = format!(
        "mongodb://{}:{}@{}",
        config.database_username, config.database_password, config.database_host,
    );
    let client = mongodb::Client::with_uri_str(database_uri.as_str()).await?;

    // データベースに接続
    let database = client.database(config.database_name.as_str());
    Ok(database)
}

/// データベースの初期化
pub async fn database_init(database: mongodb::Database) -> Result<(), mongodb::error::Error> {
    // 必要なコレクション
    let necessary_collection_names: Vec<&str> = vec!["config"];

    // 存在するコレクション
    let collection_names = database.list_collection_names(None).await?;

    // delete_meコレクションがあれば消す
    if collection_names.contains(&"delete_me".to_string()) {
        match database.collection("delete_me").drop(None).await {
            Ok(_) => println!("\"delete_me\"コレクションを削除しました。"),
            Err(e) => println!("{}", e),
        }
    }

    // 足りないコレクションを作成
    for necessary_collection_name in necessary_collection_names {
        if !collection_names.contains(&necessary_collection_name.to_string()) {
            match database
                .create_collection(
                    necessary_collection_name,
                    mongodb::options::CreateCollectionOptions::default(),
                )
                .await
            {
                Ok(_) => println!(
                    "\"{}\"コレクションを作成しました。",
                    necessary_collection_name
                ),
                Err(e) => println!("{}", e),
            }
        }
    }

    Ok(())
}

use super::client::{connect_with_url, get_db_name, get_db_url, get_db_user, get_postgres_url};
use crate::models::sql::*;
use tokio_postgres::Error;

/// Create a database
pub async fn create() -> Result<(), Error> {
    let (client, connection) = connect_with_url(&get_postgres_url()).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let db_name = get_db_name();

    println!("Creating database {}...", db_name);

    client.batch_execute(&format!(
        "
        CREATE DATABASE {} WITH OWNER {} TEMPLATE template0 ENCODING 'UTF8' LC_COLLATE 'C' LC_CTYPE 'C';
        ",
        db_name, get_db_user()
    )).await?;

    Ok(())
}

/// Initialize a database
pub async fn init() -> Result<(), Error> {
    let (client, connection) = connect_with_url(&get_db_url()).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let db_name = get_db_name();

    println!("Initializing database {}...", db_name);

    client
        .batch_execute(&format!(
            "{}\n{}\n{}",
            CREATE_BBS_TABLE, CREATE_MESSAGE_TABLE, CREATE_THREAD_TABLE
        ))
        .await?;

    Ok(())
}

/// Drop a database
pub async fn drop() -> Result<(), Error> {
    let (client, connection) = connect_with_url(&get_postgres_url()).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let db_name = get_db_name();

    println!("Dropping database {}...", db_name);

    client
        .batch_execute(&format!(
            "
        DROP DATABASE {};
        ",
            db_name
        ))
        .await?;

    Ok(())
}

use crate::cli::is_using_test_db;
use dotenv_codegen::dotenv;
use tokio_postgres::{tls::NoTlsStream, Client, Connection, Error, NoTls, Socket};

/// Connect to a database
pub async fn connect() -> Result<(Client, Connection<Socket, NoTlsStream>), Error> {
    #[cfg(test)]
    return tokio_postgres::connect(
        &format!(
            "postgresql://{}:{}@{}/{}",
            dotenv!("TEST_DATABASE_USER"),
            dotenv!("TEST_DATABASE_PASSWORD"),
            dotenv!("TEST_DATABASE_HOST"),
            dotenv!("TEST_DATABASE_NAME")
        ),
        NoTls,
    )
    .await;
    #[cfg(not(test))]
    return tokio_postgres::connect(&get_db_url(), NoTls).await;
}

/// Connect to a specified database
pub async fn connect_with_url(
    url: &str,
) -> Result<(Client, Connection<Socket, NoTlsStream>), Error> {
    tokio_postgres::connect(url, NoTls).await
}

/// Get appropriate database URL
pub fn get_db_url() -> String {
    if is_using_test_db() {
        format!(
            "postgresql://{}:{}@{}/{}",
            dotenv!("TEST_DATABASE_USER"),
            dotenv!("TEST_DATABASE_PASSWORD"),
            dotenv!("TEST_DATABASE_HOST"),
            dotenv!("TEST_DATABASE_NAME")
        )
    } else {
        format!(
            "postgresql://{}:{}@{}/{}",
            dotenv!("DATABASE_USER"),
            dotenv!("DATABASE_PASSWORD"),
            dotenv!("DATABASE_HOST"),
            dotenv!("DATABASE_NAME")
        )
    }
}

/// Get appropriate postgres url
pub fn get_postgres_url() -> String {
    if is_using_test_db() {
        format!(
            "postgresql://{}:{}@{}/postgres",
            dotenv!("TEST_DATABASE_USER"),
            dotenv!("TEST_DATABASE_PASSWORD"),
            dotenv!("TEST_DATABASE_HOST")
        )
    } else {
        format!(
            "postgresql://{}:{}@{}/postgres",
            dotenv!("DATABASE_USER"),
            dotenv!("DATABASE_PASSWORD"),
            dotenv!("DATABASE_HOST"),
        )
    }
}

/// Get appropriate database name
pub fn get_db_name() -> String {
    if is_using_test_db() {
        dotenv!("TEST_DATABASE_NAME").to_string()
    } else {
        dotenv!("DATABASE_NAME").to_string()
    }
}

/// Get appropriate database user
pub fn get_db_user() -> String {
    if is_using_test_db() {
        dotenv!("TEST_DATABASE_USER").to_string()
    } else {
        dotenv!("DATABASE_USER").to_string()
    }
}

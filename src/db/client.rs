use crate::cli::is_using_test_db;
use crate::util::get_env;
use tokio_postgres::{tls::NoTlsStream, Client, Connection, Error, NoTls, Socket};

/// Connect to a database
pub async fn connect() -> Result<(Client, Connection<Socket, NoTlsStream>), Error> {
    #[cfg(test)]
    return tokio_postgres::connect(
        &format!(
            "postgresql://{}:{}@{}/{}",
            get_env("TEST_DATABASE_USER"),
            get_env("TEST_DATABASE_PASSWORD"),
            get_env("TEST_DATABASE_HOST"),
            get_env("TEST_DATABASE_NAME")
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
            get_env("TEST_DATABASE_USER"),
            get_env("TEST_DATABASE_PASSWORD"),
            get_env("TEST_DATABASE_HOST"),
            get_env("TEST_DATABASE_NAME")
        )
    } else {
        format!(
            "postgresql://{}:{}@{}/{}",
            get_env("DATABASE_USER"),
            get_env("DATABASE_PASSWORD"),
            get_env("DATABASE_HOST"),
            get_env("DATABASE_NAME")
        )
    }
}

/// Get appropriate postgres url
pub fn get_postgres_url() -> String {
    if is_using_test_db() {
        format!(
            "postgresql://{}:{}@{}/postgres",
            get_env("TEST_DATABASE_USER"),
            get_env("TEST_DATABASE_PASSWORD"),
            get_env("TEST_DATABASE_HOST")
        )
    } else {
        format!(
            "postgresql://{}:{}@{}/postgres",
            get_env("DATABASE_USER"),
            get_env("DATABASE_PASSWORD"),
            get_env("DATABASE_HOST"),
        )
    }
}

/// Get appropriate database name
pub fn get_db_name() -> String {
    if is_using_test_db() {
        get_env("TEST_DATABASE_NAME")
    } else {
        get_env("DATABASE_NAME")
    }
}

/// Get appropriate database user
pub fn get_db_user() -> String {
    if is_using_test_db() {
        get_env("TEST_DATABASE_USER")
    } else {
        get_env("DATABASE_USER")
    }
}

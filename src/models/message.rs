use crate::db;
use chrono::prelude::*;
use tokio_postgres::Error;

/// Message model
pub struct Message {
    pub id: i64,
    pub thread_id: i64,
    pub user_name: String,
    pub user_email: String,
    pub user_uid: String,
    pub user_ip: String,
    pub user_hostname: String,
    pub user_agent: String,
    pub timestamp: DateTime<FixedOffset>,
    pub text: String,
    pub hidden: bool,
}

/// SQL for creating message table
pub const CREATE_MESSAGE_TABLE: &str = "
CREATE TABLE message (
    id              BIGSERIAL PRIMARY KEY,
    thread_id       BIGSERIAL NOT NULL,
    user_name       TEXT NOT NULL,
    user_email      TEXT NOT NULL,
    user_uid        TEXT NOT NULL,
    user_ip         TEXT NOT NULL,
    user_hostname   TEXT NOT NULL,
    user_agent      TEXT NOT NULL,
    timestamp       TIMESTAMP WITH TIME ZONE NOT NULL,
    text            TEXT NOT NULL,
    hidden          BOOLEAN NOT NULL DEFAULT false
);
";

impl Message {
    /// Create a message in a database
    pub async fn create(
        thread_id: i64,
        user_name: &str,
        user_email: &str,
        user_uid: &str,
        user_ip: &str,
        user_hostname: &str,
        user_agent: &str,
        timestamp: DateTime<FixedOffset>,
        text: &str,
    ) -> Result<Message, Error> {
        let (client, connection) = db::connect().await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let statement = client
            .prepare(
                "
            INSERT INTO message (
                thread_id,
                user_name,
                user_email,
                user_uid,
                user_ip,
                user_hostname,
                user_agent,
                text,
                timestamp
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id, hidden
            ",
            )
            .await?;

        let row = client
            .query_one(
                &statement,
                &[
                    &thread_id,
                    &user_name,
                    &user_email,
                    &user_uid,
                    &user_ip,
                    &user_hostname,
                    &user_agent,
                    &text,
                    &timestamp,
                ],
            )
            .await?;
        let id: i64 = row.get(0);
        let hidden: bool = row.get(1);

        Ok(Message {
            id,
            thread_id,
            user_name: user_name.to_string(),
            user_email: user_email.to_string(),
            user_uid: user_uid.to_string(),
            user_ip: user_ip.to_string(),
            user_hostname: user_hostname.to_string(),
            user_agent: user_agent.to_string(),
            timestamp,
            text: text.to_string(),
            hidden,
        })
    }

    /// Find a message in a database
    pub async fn find(id: i64) -> Result<Message, Error> {
        let (client, connection) = db::connect().await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let row = client
            .query_one("SELECT * FROM message WHERE id = ($1)", &[&id])
            .await?;
        let id: i64 = row.get(0);
        let thread_id: i64 = row.get(1);
        let user_name: String = row.get(2);
        let user_email: String = row.get(3);
        let user_uid: String = row.get(4);
        let user_ip: String = row.get(5);
        let user_hostname: String = row.get(6);
        let user_agent: String = row.get(7);
        let timestamp: DateTime<FixedOffset> = row.get(8);
        let text: String = row.get(9);
        let hidden: bool = row.get(10);

        Ok(Message {
            id,
            thread_id,
            user_name,
            user_email,
            user_uid,
            user_ip,
            user_hostname,
            user_agent,
            timestamp,
            text,
            hidden,
        })
    }

    pub async fn find_messages_by_thread_id(thread_id: i64) -> Result<Vec<Message>, Error> {
        let (client, connection) = db::connect().await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let rows = client
            .query(
                "SELECT * FROM message WHERE thread_id = ($1)",
                &[&thread_id],
            )
            .await?;

        Ok(rows
            .iter()
            .map(|row| {
                let id: i64 = row.get(0);
                let thread_id: i64 = row.get(1);
                let user_name: String = row.get(2);
                let user_email: String = row.get(3);
                let user_uid: String = row.get(4);
                let user_ip: String = row.get(5);
                let user_hostname: String = row.get(6);
                let user_agent: String = row.get(7);
                let timestamp: DateTime<FixedOffset> = row.get(8);
                let text: String = row.get(9);
                let hidden: bool = row.get(10);

                Message {
                    id,
                    thread_id,
                    user_name,
                    user_email,
                    user_uid,
                    user_ip,
                    user_hostname,
                    user_agent,
                    timestamp,
                    text,
                    hidden,
                }
            })
            .collect())
    }

    /// Delete a message in a database
    pub async fn delete(id: i64) -> Result<(), Error> {
        let (client, connection) = db::connect().await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        client
            .execute("DELETE FROM message WHERE id = ($1)", &[&id])
            .await?;

        Ok(())
    }
}

pub async fn get_message_count(thread_id: i64) -> Result<i64, Error> {
    let (client, connection) = db::connect().await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let row = client
        .query_one(
            "SELECT COUNT(id) FROM message WHERE thread_id = ($1)",
            &[&thread_id],
        )
        .await?;

    let message_count: i64 = row.get(0);

    Ok(message_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_and_find_and_delete() {
        let timestamp = FixedOffset::east(9 * 3600)
            .ymd(2014, 11, 28)
            .and_hms_nano(21, 45, 59, 324310000);
        let message = Message::create(
        1,
        "名無し",
        "sage",
        "ABCDEFGHI",
        "127.0.0.1",
        "example.com",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.102 Safari/537.36",
        timestamp,
        "こんにちは",
    ).await
    .unwrap();

        assert_eq!(1, message.thread_id);
        assert_eq!("名無し", message.user_name);
        assert_eq!("sage", message.user_email);
        assert_eq!("ABCDEFGHI", message.user_uid);
        assert_eq!("127.0.0.1", message.user_ip);
        assert_eq!("example.com", message.user_hostname);
        assert_eq!("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.102 Safari/537.36",message.user_agent);
        assert_eq!(timestamp, message.timestamp);
        assert_eq!("こんにちは", message.text);
        assert_eq!(false, message.hidden);

        let finded_message = Message::find(message.id).await.unwrap();

        assert_eq!(message.id, finded_message.id);
        assert_eq!(message.thread_id, finded_message.thread_id);
        assert_eq!(message.user_name, finded_message.user_name);
        assert_eq!(message.user_email, finded_message.user_email);
        assert_eq!(message.user_uid, finded_message.user_uid);
        assert_eq!(message.user_ip, finded_message.user_ip);
        assert_eq!(message.user_hostname, finded_message.user_hostname);
        assert_eq!(message.user_agent, finded_message.user_agent);
        assert_eq!(message.timestamp, finded_message.timestamp);
        assert_eq!(message.text, finded_message.text);
        assert_eq!(message.hidden, finded_message.hidden);

        Message::delete(message.id).await.unwrap();
    }
}

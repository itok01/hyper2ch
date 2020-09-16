use crate::db;
use tokio_postgres::Error;

/// Thread model
pub struct Thread {
    pub id: i64,
    pub bbs_id: i64,
    pub title: String,
    pub archived: bool,
    pub hidden: bool,
}

/// SQL for creating thread table
pub const CREATE_THREAD_TABLE: &str = "
CREATE TABLE thread (
    id          BIGSERIAL PRIMARY KEY,
    bbs_id      BIGSERIAL NOT NULL,
    title       TEXT NOT NULL,
    archived    BOOLEAN NOT NULL DEFAULT false,
    hidden      BOOLEAN NOT NULL DEFAULT false
);
";

impl Thread {
    /// Create a thread in a database
    pub async fn create(bbs_id: i64, title: &str) -> Result<Thread, Error> {
        let (client, connection) = db::connect().await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let statement = client
            .prepare(
                "INSERT INTO thread (bbs_id, title) VALUES ($1, $2) RETURNING id, archived, hidden",
            )
            .await?;

        let row = client.query_one(&statement, &[&bbs_id, &title]).await?;
        let id: i64 = row.get(0);
        let archived: bool = row.get(1);
        let hidden: bool = row.get(2);

        Ok(Thread {
            id,
            bbs_id,
            title: title.to_string(),
            archived,
            hidden,
        })
    }

    /// Find a thread in a database
    pub async fn find(id: i64) -> Result<Thread, Error> {
        let (client, connection) = db::connect().await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let row = client
            .query_one("SELECT * FROM thread WHERE id = ($1)", &[&id])
            .await?;
        let id: i64 = row.get(0);
        let bbs_id: i64 = row.get(1);
        let title: String = row.get(2);
        let archived: bool = row.get(3);
        let hidden: bool = row.get(4);

        Ok(Thread {
            id,
            bbs_id,
            title,
            archived,
            hidden,
        })
    }

    /// Delete a thread in a database
    pub async fn delete(id: i64) -> Result<(), Error> {
        let (client, connection) = db::connect().await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        client
            .execute("DELETE FROM thread WHERE id = ($1)", &[&id])
            .await?;

        Ok(())
    }

    /// Find a thread in a available database
    pub async fn find_available_in_bbs(bbs_id: i64) -> Result<Vec<Thread>, Error> {
        let (client, connection) = db::connect().await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let rows = client
            .query(
                "SELECT * FROM thread WHERE bbs_id = ($1) AND archived = false AND hidden = false",
                &[&bbs_id],
            )
            .await?;

        Ok(rows
            .iter()
            .map(|row| {
                let id: i64 = row.get(0);
                let bbs_id: i64 = row.get(1);
                let title: String = row.get(2);
                let archived: bool = row.get(3);
                let hidden: bool = row.get(4);

                Thread {
                    id,
                    bbs_id,
                    title,
                    archived,
                    hidden,
                }
            })
            .collect())
    }
}

/// Get a thread title in a database
pub async fn get_thread_title(id: i64) -> Result<String, Error> {
    let (client, connection) = db::connect().await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let row = client
        .query_one("SELECT title FROM thread WHERE id = ($1)", &[&id])
        .await?;
    let title: String = row.get(0);

    Ok(title)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_and_find_and_delete() {
        let thread = Thread::create(1, "title").await.unwrap();

        assert_eq!(1, thread.bbs_id);
        assert_eq!("title", thread.title);
        assert_eq!(false, thread.archived);
        assert_eq!(false, thread.hidden);

        let finded_thread = Thread::find(thread.id).await.unwrap();

        assert_eq!(thread.id, finded_thread.id);
        assert_eq!(thread.bbs_id, finded_thread.bbs_id);
        assert_eq!(thread.title, finded_thread.title);
        assert_eq!(thread.archived, finded_thread.archived);
        assert_eq!(thread.hidden, finded_thread.hidden);

        Thread::delete(thread.id).await.unwrap();
    }
}

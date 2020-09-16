use crate::db;
use tokio_postgres::Error;

/// BBS model
pub struct Bbs {
    pub id: i64,
    pub name: String,
    pub path_name: String,
    pub description: String,
    pub category: String,
    pub hidden: bool,
}

/// SQL for creating bbs table
pub const CREATE_BBS_TABLE: &str = "
CREATE TABLE bbs (
    id          BIGSERIAL PRIMARY KEY,
    name        TEXT NOT NULL,
    path_name   TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    category    TEXT NOT NULL,
    hidden      BOOLEAN NOT NULL DEFAULT false
);
";

impl Bbs {
    /// Create a bbs in a database
    pub async fn create(
        name: &str,
        path_name: &str,
        description: &str,
        category: &str,
    ) -> Result<Bbs, Error> {
        let (client, connection) = db::connect().await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let statement = client
            .prepare(
                "
            INSERT INTO bbs (
                name,
                path_name,
                description,
                category
            ) VALUES ($1, $2, $3, $4) RETURNING id, hidden
            ",
            )
            .await?;

        let row = client
            .query_one(&statement, &[&name, &path_name, &description, &category])
            .await?;
        let id: i64 = row.get(0);
        let hidden: bool = row.get(1);

        Ok(Bbs {
            id,
            name: name.to_string(),
            path_name: path_name.to_string(),
            description: description.to_string(),
            category: category.to_string(),
            hidden,
        })
    }

    /// Find a bbs in a database
    pub async fn find(id: i64) -> Result<Bbs, Error> {
        let (client, connection) = db::connect().await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let row = client
            .query_one("SELECT * FROM bbs WHERE id = ($1)", &[&id])
            .await?;
        let id: i64 = row.get(0);
        let name: String = row.get(1);
        let path_name: String = row.get(2);
        let description: String = row.get(3);
        let category: String = row.get(4);
        let hidden: bool = row.get(5);

        Ok(Bbs {
            id,
            name,
            path_name,
            description,
            category,
            hidden,
        })
    }

    /// Find a bbs in a database by path_name
    pub async fn find_by_path_name(path_name: &str) -> Result<Bbs, Error> {
        let (client, connection) = db::connect().await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let row = client
            .query_one("SELECT * FROM bbs WHERE path_name = ($1)", &[&path_name])
            .await?;
        let id: i64 = row.get(0);
        let name: String = row.get(1);
        let path_name: String = row.get(2);
        let description: String = row.get(3);
        let category: String = row.get(4);
        let hidden: bool = row.get(5);

        Ok(Bbs {
            id,
            name,
            path_name,
            description,
            category,
            hidden,
        })
    }

    /// Delete a bbs in a database
    pub async fn delete(id: i64) -> Result<(), Error> {
        let (client, connection) = db::connect().await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        client
            .execute("DELETE FROM bbs WHERE id = ($1)", &[&id])
            .await?;

        Ok(())
    }

    /// Find a bbs in a available database
    pub async fn find_shown() -> Result<Vec<Bbs>, Error> {
        let (client, connection) = db::connect().await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let rows = client
            .query(
                "SELECT * FROM bbs WHERE hidden = false ORDER BY category",
                &[],
            )
            .await?;

        Ok(rows
            .iter()
            .map(|row| {
                let id: i64 = row.get(0);
                let name: String = row.get(1);
                let path_name: String = row.get(2);
                let description: String = row.get(3);
                let category: String = row.get(4);
                let hidden: bool = row.get(5);

                Bbs {
                    id,
                    name,
                    path_name,
                    description,
                    category,
                    hidden,
                }
            })
            .collect())
    }
}

pub async fn get_id_by_bbs_path_name(bbs_path_name: &str) -> Result<i64, Error> {
    let (client, connection) = db::connect().await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let row = client
        .query_one(
            "SELECT id FROM bbs WHERE path_name = ($1)",
            &[&bbs_path_name],
        )
        .await?;

    let id: i64 = row.get(0);

    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_and_find_and_delete() {
        let bbs = Bbs::create("プログラミング", "programming", "description", "Tech")
            .await
            .unwrap();

        assert_eq!("プログラミング", bbs.name);
        assert_eq!("programming", bbs.path_name);
        assert_eq!("description", bbs.description);
        assert_eq!("Tech", bbs.category);
        assert_eq!(false, bbs.hidden);

        let finded_bbs = Bbs::find(bbs.id).await.unwrap();

        assert_eq!(bbs.id, finded_bbs.id);
        assert_eq!(bbs.name, finded_bbs.name);
        assert_eq!(bbs.path_name, finded_bbs.path_name);
        assert_eq!(bbs.description, finded_bbs.description);
        assert_eq!(bbs.category, finded_bbs.category);
        assert_eq!(bbs.hidden, finded_bbs.hidden);

        Bbs::delete(bbs.id).await.unwrap();
    }
}

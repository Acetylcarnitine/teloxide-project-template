use std::ops::Deref;
use std::sync::Arc;
use sqlx::{Error, Pool, Postgres, Row};
use crate::database::traits::reader::DefaultReader;

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub deposit: i32,
    pub rating: i32
}

impl User {
    pub async fn save(
        tele_id: i64,
        pool: Arc<Pool<Postgres>>
    ) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO users (tele_id) VALUES ($1) RETURNING tele_id"
        )
            .bind(tele_id)
            .execute(pool.deref())
            .await?;

        Ok(())
    }

    pub async fn add_to_balance(
        tele_id: i64,
        value: i64,
        pool: Arc<Pool<Postgres>>
    ) -> Result<(), Error> {
        if value < 0 {
            panic!("Cannot add number that less than 0")
        }

        sqlx::query(
            "UPDATE users SET deposit = deposit + $1 WHERE tele_id = $2"
        )
            .bind(value)
            .bind(tele_id)
            .execute(pool.deref())
            .await?;

        Ok(())
    }
}

impl DefaultReader for User {
    async fn read_one(pool: Arc<Pool<Postgres>>, id: i64) -> Result<Self, Error>
        where Self: Sized
    {
        let row = sqlx::query("SELECT * FROM users WHERE tele_id = $1")
            .bind(id)
            .fetch_one(pool.deref())
            .await?;

        Ok(Self {
            id: row.get(0),
            deposit: row.get(1),
            rating: row.get(2)
        })
    }

    async fn read_all(pool: Arc<Pool<Postgres>>) -> Result<Vec<Self>, Error>
        where Self: Sized
    {
        let rows = sqlx::query("SELECT * FROM users")
            .fetch_all(pool.deref())
            .await?;

        let mut users: Vec<Self> = Vec::new();
        let _ = rows.iter().map(|row| {
            users.push(User {
                id: row.get(0),
                deposit: row.get(1),
                rating: row.get(2)
            });
        });

        Ok(users)
    }
}
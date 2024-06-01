use std::sync::Arc;
use sqlx::{Pool, Postgres};

pub trait DefaultReader {
    async fn read_one(pool: Arc<Pool<Postgres>>, id: i64) -> Result<Self, sqlx::Error> where Self: Sized;
    async fn read_all(pool: Arc<Pool<Postgres>>) -> Result<Vec<Self>, sqlx::Error> where Self: Sized;
}
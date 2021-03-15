use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::Row;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .min_connections(10)
        .connect_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(1800))
        .idle_timeout(Duration::from_secs(600))
        .connect("postgres://postgres:postgres@localhost/learn_sqlx")
        .await?;

    let stream = sqlx::query("select id,name from to_do limit 10")
        .map(|row: PgRow| User {
            id: row.try_get("id").unwrap_or(0),
            name: row.try_get("name").unwrap_or("".into())
        })
        .fetch(&pool);


    Ok(())
}

#[derive(Debug, Default)]
struct User {
    id: i32,
    name: String,
}
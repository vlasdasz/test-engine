#![allow(unused_imports)]
#![allow(dead_code)]

use std::time::Duration;

use anyhow::Result;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions, query_as};
use tain::Postgres;
use tokio::time::sleep;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations/"); // defaults to "./migrations"

#[derive(Debug)]
struct User {
    pub id:       i32,
    pub username: String,
    pub email:    String,
    pub age:      i16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let pg = Postgres::default().data("./tmp/pg").start_container();

    let pg_port = pg.get_host_port_ipv4(5432);

    dbg!(&pg_port);

    let pool = PgPoolOptions::new()
        .connect(&format!(
            "postgres://postgres:postgres@localhost:{pg_port}/postgres"
        ))
        .await?;

    MIGRATOR.run(&pool).await?;

    let row: (i64,) = sqlx::query_as("SELECT $1").bind(150_i64).fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    let user = User {
        id:       0,
        username: "prostaf".to_string(),
        email:    "prostaf@gmail.com".to_string(),
        age:      10,
    };

    // let user: User = query_as!(
    //     User,
    //     r#"
    //     INSERT INTO users (username, email, age)
    //     VALUES ($1, $2, $3)
    //     RETURNING id, username, email, age
    //     "#,
    //     user.username,
    //     user.email,
    //     user.age
    // )
    // .fetch_one(&pool)
    // .await?;

    dbg!(&user);

    sleep(Duration::from_secs(1000000)).await;

    dbg!("a");

    Ok(())
}

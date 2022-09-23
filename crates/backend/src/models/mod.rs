use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{env, process::exit};

mod positions;
mod scheduled_position;
mod team;
mod team_member;
mod types;
mod user;

pub fn _default_false() -> bool {
    false
}

pub fn _default_true() -> bool {
    false
}

pub async fn db() -> Result<Pool<Postgres>> {
    let db_url = match env::var("DATABASE_URL") {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Failed to get DATABASE_URL variable. {}", e);
            exit(1);
        }
    };

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let migrator = sqlx::migrate!();
    migrator.run(&pool).await?;

    Ok(pool)
}

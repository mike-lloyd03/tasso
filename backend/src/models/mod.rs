use anyhow::Result;
use async_trait::async_trait;
use sqlx::{
    postgres::{PgPoolOptions, PgQueryResult, PgRow},
    PgPool, Pool, Postgres, QueryBuilder,
};
use std::{env, fmt::Display, process::exit};

mod team;
mod team_member;
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

#[derive(Debug, Clone)]
pub enum DataType {
    Str(String),
    OptStr(Option<String>),
    Int(i64),
    Bool(bool),
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Str(v) => write!(f, "'{}'", v),
            DataType::OptStr(v) => match v {
                Some(t) => write!(f, "'{}'", t),
                None => write!(f, "null"),
            },
            DataType::Int(v) => write!(f, "'{}'", v),
            DataType::Bool(v) => write!(f, "'{}'", v),
        }
    }
}

#[async_trait]
pub trait Resource: Sized + for<'r> sqlx::FromRow<'r, PgRow> + Unpin + Send {
    fn table_name() -> &'static str;

    fn fields(&self) -> Vec<(&'static str, DataType)>;

    fn primary_key() -> &'static str {
        "id"
    }

    fn primary_key_value(&self) -> DataType;

    async fn create(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let mut query: QueryBuilder<Postgres> = QueryBuilder::new("INSERT INTO ");
        query.push(Self::table_name());

        let mut columns = query.separated(", ");
        columns.push_unseparated(" (");
        for f in self.fields().iter() {
            columns.push(f.0);
        }
        columns.push_unseparated(") ");

        query.push("VALUES (");
        let mut values = query.separated(", ");
        for f in self.fields().iter() {
            values.push(f.1.to_string());
        }
        values.push_unseparated(") ");

        query.build().execute(pool).await
    }

    /// Get an item by it's primary key
    async fn get(pool: &PgPool, identifier: DataType) -> Result<Self, sqlx::Error> {
        let mut query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM ");
        query
            .push(Self::table_name())
            .push(" WHERE ")
            .push(Self::primary_key())
            .push(" = ")
            .push(identifier.clone());

        query.build_query_as().fetch_one(pool).await
    }

    async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let mut query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM ");
        query
            .push(Self::table_name())
            .push(" ORDER BY ")
            .push(Self::primary_key());

        query.build_query_as().fetch_all(pool).await
    }

    async fn update(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let mut query: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE ");

        query.push(Self::table_name()).push(" SET ");
        let mut columns = query.separated(", ");
        for f in self.fields().iter() {
            let v = format!("{} = {}", f.0, f.1);
            columns.push(v);
        }

        query.push("WHERE id = ").push(self.primary_key_value());

        query.build().execute(pool).await
    }

    async fn delete(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query(&format!(
            "DELETE FROM {} WHERE id = {}",
            Self::table_name(),
            self.primary_key_value()
        ))
        .execute(pool)
        .await
    }
}

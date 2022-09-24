use async_trait::async_trait;
use sqlx::{
    postgres::{PgQueryResult, PgRow},
    PgPool,
};

pub use resource_derive::Resource;

#[async_trait]
pub trait Resource: Sized + for<'r> sqlx::FromRow<'r, PgRow> + Unpin + Send {
    async fn create(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error>;

    async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error>;

    async fn update(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error>;

    async fn delete(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error>;
}

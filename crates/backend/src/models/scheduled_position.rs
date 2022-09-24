use resource::Resource;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow, Serialize, Deserialize, Resource)]
pub struct ScheduledPosition {
    #[primary_key]
    id: i64,
    position_id: i64,
    user_id: i64,
}

#[cfg(test)]
mod scheduled_position_tests {
    use crate::models::scheduled_position::ScheduledPosition;
    use anyhow::Result;
    use resource::Resource;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_get_all_scheduled_positions(pool: PgPool) -> Result<()> {
        let res = ScheduledPosition::get_all(&pool).await?;
        assert_eq!(0, res.len());

        Ok(())
    }

    #[sqlx::test(fixtures("users", "teams", "positions", "scheduled_positions"))]
    async fn test_get_scheduled_positions(pool: PgPool) -> Result<()> {
        let sp = ScheduledPosition::get(&pool, 1).await?;
        assert_eq!(sp.position_id, 1);

        Ok(())
    }

    #[sqlx::test(fixtures("users", "teams", "positions", "scheduled_positions"))]
    async fn test_delete_scheduled_positions(pool: PgPool) -> Result<()> {
        let sp = ScheduledPosition::get(&pool, 1).await?;
        let res = sp.delete(&pool).await?;
        assert_eq!(1, res.rows_affected());

        let sp_deleted = ScheduledPosition::get(&pool, 1).await;
        assert!(sp_deleted.is_err());

        Ok(())
    }

    #[sqlx::test(fixtures("users", "teams", "positions"))]
    async fn test_create_scheduled_positions(pool: PgPool) -> Result<()> {
        let sp = ScheduledPosition {
            id: 0,
            position_id: 1,
            user_id: 1,
        };
        let res = sp.create(&pool).await?;
        assert_eq!(1, res.rows_affected());

        Ok(())
    }

    #[sqlx::test(fixtures("users", "teams", "positions", "scheduled_positions"))]
    async fn test_update_scheduled_positions(pool: PgPool) -> Result<()> {
        let mut sp = ScheduledPosition::get(&pool, 1).await?;
        sp.user_id = 2;
        sp.update(&pool).await?;

        let sp_updated = ScheduledPosition::get(&pool, 1).await?;
        assert_eq!(2, sp_updated.user_id);

        Ok(())
    }
}

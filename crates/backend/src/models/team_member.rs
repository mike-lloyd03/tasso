use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::resource::Resource;

#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow, Serialize, Deserialize, Resource)]
pub struct TeamMember {
    #[primary_key]
    id: i64,
    team_id: i64,
    user_id: i64,
    manager: bool,
}

#[cfg(test)]
mod team_member_tests {
    use crate::models::resource::Resource;
    use crate::models::team_member::TeamMember;
    use anyhow::Result;
    use sqlx::PgPool;

    #[sqlx::test(fixtures("team_members"))]
    async fn test_create_team_member(pool: PgPool) -> Result<()> {
        let tm1 = TeamMember {
            id: 0,
            team_id: 1,
            user_id: 1,
            manager: true,
        };
        let res = tm1.create(&pool).await?;
        assert_eq!(1, res.rows_affected());

        Ok(())
    }

    #[sqlx::test(fixtures("team_members"))]
    async fn test_create_team_member_error(pool: PgPool) -> Result<()> {
        let tm1 = TeamMember {
            id: 0,
            team_id: 3, // Team 3 doesn't exist. Foreign key error.
            user_id: 1,
            manager: true,
        };
        let res = tm1.create(&pool).await;
        assert!(res.is_err());

        let tm2 = TeamMember {
            id: 0,
            team_id: 1,
            user_id: 100, // User 100 doesn't exist. Foreign key error.
            manager: false,
        };
        let res = tm2.create(&pool).await;
        assert!(res.is_err());

        Ok(())
    }
}

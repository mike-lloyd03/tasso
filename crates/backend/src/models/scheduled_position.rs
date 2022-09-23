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
    use resource::Resource;
    //     use crate::models::types::Resource;
    use anyhow::Result;
    use sqlx::PgPool;
    //     use sqlx::PgPool;
    #[test]
    fn test_resource() -> Result<()> {
        // let sp = ScheduledPosition {
        //     id: 0,
        //     position_id: 1,
        //     user_id: 2,
        // };

        assert_eq!("id", ScheduledPosition::primary_key());
        assert_ne!("ScheduledPosition", ScheduledPosition::table_name());

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_all_scheduled_positions(pool: PgPool) -> Result<()> {
        let res = ScheduledPosition::get_all(&pool).await?;
        assert_eq!(0, res.len());

        Ok(())
    }

    //     #[sqlx::test(fixtures("team_members"))]
    //     async fn test_create_team_member_error(pool: PgPool) -> Result<()> {
    //         let tm1 = TeamMember {
    //             id: 0,
    //             team_id: 3, // Team 3 doesn't exist. Foreign key error.
    //             user_id: 1,
    //             manager: true,
    //         };
    //         let res = tm1.create(&pool).await;
    //         assert!(res.is_err());

    //         let tm2 = TeamMember {
    //             id: 0,
    //             team_id: 1,
    //             user_id: 100, // User 100 doesn't exist. Foreign key error.
    //             manager: false,
    //         };
    //         let res = tm2.create(&pool).await;
    //         assert!(res.is_err());

    //         Ok(())
    //     }
}

use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::resource::Resource;

#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow, Serialize, Deserialize, Resource)]
pub struct Position {
    #[primary_key]
    id: i64,
    team_id: i64,
    name: String,
    date: NaiveDate,
    start_time: NaiveTime,
    end_time: NaiveTime,
}

#[cfg(test)]
mod position_tests {
    use crate::models::positions::Position;
    use crate::models::resource::Resource;
    use anyhow::Result;
    use chrono::{NaiveDate, NaiveTime};
    use sqlx::PgPool;

    #[sqlx::test(fixtures("teams"))]
    async fn test_create_position(pool: PgPool) -> Result<()> {
        let pos1 = Position {
            id: 0,
            team_id: 1,
            name: "Position1".into(),
            date: NaiveDate::from_ymd(2022, 11, 4),
            start_time: NaiveTime::from_hms(9, 30, 0),
            end_time: NaiveTime::from_hms(10, 45, 0),
        };
        let res = pos1.create(&pool).await?;
        assert_eq!(1, res.rows_affected());

        Ok(())
    }
}

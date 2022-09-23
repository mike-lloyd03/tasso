use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::types::{DataType, Resource};

#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow, Serialize, Deserialize)]
pub struct Position {
    id: i64,
    team_id: i64,
    name: String,
    date: NaiveDate,
    start_time: NaiveTime,
    end_time: NaiveTime,
}

impl Resource for Position {
    fn table_name() -> &'static str {
        "positions"
    }

    fn fields(&self) -> Vec<(&'static str, DataType)> {
        vec![
            ("team_id", DataType::Int64(self.team_id)),
            ("name", DataType::String(self.name.clone())),
            ("date", DataType::Date(self.date)),
            ("start_time", DataType::Time(self.start_time)),
            ("end_time", DataType::Time(self.end_time)),
        ]
    }

    fn primary_key_value(&self) -> DataType {
        DataType::Int64(self.id)
    }
}

#[cfg(test)]
mod position_tests {
    use crate::models::positions::Position;
    use crate::models::types::Resource;
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
        assert_eq!(2, res.rows_affected());

        Ok(())
    }
}

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::resource::Resource;

#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow, Serialize, Deserialize, Resource)]
pub struct Team {
    #[primary_key]
    id: i64,
    name: String,
    description: Option<String>,
}

#[cfg(test)]
mod team_tests {
    use crate::models::resource::Resource;
    use crate::models::team::Team;
    use anyhow::Result;
    use sqlx::{query, PgPool};

    #[sqlx::test()]
    async fn test_create_team(pool: PgPool) -> Result<()> {
        let name = "team1".into();
        let description = Some("This is a team".into());
        let team = Team {
            name,
            description,
            ..Default::default()
        };
        team.create(&pool).await?;
        let got_team = Team::get(&pool, 1).await?;

        assert_eq!(team.name, got_team.name);
        assert_eq!(team.description, got_team.description);

        Ok(())
    }

    #[sqlx::test(fixtures("teams"))]
    async fn test_get_team(pool: PgPool) -> Result<()> {
        let team = Team::get(&pool, 1).await?;

        assert_eq!("team1", team.name);
        assert_eq!(Some("this is a good team".into()), team.description);

        Ok(())
    }

    #[sqlx::test(fixtures("teams"))]
    async fn test_get_all_teams(pool: PgPool) -> Result<()> {
        let teams = Team::get_all(&pool).await?;

        assert_eq!(3, teams.len());

        Ok(())
    }

    #[sqlx::test(fixtures("teams"))]
    async fn test_update_team(pool: PgPool) -> Result<()> {
        let new_name = "teamTwo";
        let mut team = Team::get(&pool, 1).await?;
        team.name = new_name.into();
        team.update(&pool).await?;

        let updated_team = Team::get(&pool, 1).await?;

        assert_eq!(new_name, updated_team.name);

        Ok(())
    }

    #[sqlx::test(fixtures("teams"))]
    async fn test_delete_team(pool: PgPool) -> Result<()> {
        let team = Team::get(&pool, 2).await?;
        team.delete(&pool).await?;

        let res = query("SELECT * FROM users WHERE id = $1")
            .bind(2)
            .execute(&pool)
            .await?;

        assert_eq!(res.rows_affected(), 0);

        Ok(())
    }
}

use std::env;

use anyhow::Result;
use chrono::NaiveDate;
use lazy_static::lazy_static;
use orion::pwhash::{self, hash_password_verify, Password, PasswordHash};
use pwgen;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, query, query_as, FromRow, PgPool};
use validator::Validate;

use super::{DataType, Resource, _default_false, _default_true};

lazy_static! {
    static ref USERNAME: Regex = Regex::new(r#"[\w\d]{3,}"#).expect("failed creating regex");
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow, Serialize, Deserialize, Validate)]
pub struct User {
    pub id: i64,
    #[validate(regex = "USERNAME")]
    pub username: String,
    pub lastname: Option<String>,
    pub firstname: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[serde(skip)]
    password_hash: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    #[serde(default = "_default_false")]
    pub admin: bool,
    #[serde(default = "_default_true")]
    pub active: bool,
}

impl Resource for User {
    fn table_name() -> &'static str {
        "users"
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn fields(&self) -> Vec<(&'static str, super::DataType)> {
        vec![
            ("username", DataType::Str(self.username.clone())),
            ("lastname", DataType::OptStr(self.lastname.clone())),
            ("firstname", DataType::OptStr(self.firstname.clone())),
            ("email", DataType::OptStr(self.email.clone())),
            ("admin", DataType::Bool(self.admin)),
            ("active", DataType::Bool(self.active)),
        ]
    }
}

impl User {
    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Self, sqlx::Error> {
        query_as!(
            Self,
            r#"
                SELECT
                    id,
                    username,
                    lastname,
                    firstname,
                    email,
                    password_hash,
                    date_of_birth,
                    admin,
                    active
                FROM users
                WHERE username = $1
            "#,
            username
        )
        .fetch_one(pool)
        .await
    }

    pub async fn authenticate(pool: &PgPool, creds: Credentials) -> Result<Self, actix_web::Error> {
        let user = match Self::get_by_username(pool, &creds.username).await {
            Ok(u) => u,
            Err(_) => {
                // Attempt to validate the password on a fake account to prevent a timing attack
                fake_validate();
                return Err(actix_web::error::ErrorUnauthorized("Authentication failed"));
            }
        };
        if user.password_hash.is_some() && user.validate_password(&creds.password) {
            Ok(user)
        } else {
            fake_validate();
            Err(actix_web::error::ErrorUnauthorized("Authentication failed"))
        }
    }

    pub async fn set_password(
        &mut self,
        pool: &PgPool,
        password: &str,
    ) -> Result<PgQueryResult, sqlx::Error> {
        let pw = Password::from_slice(password.as_bytes())
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let hash = pwhash::hash_password(&pw, 3, 1 << 16).unwrap();
        self.password_hash = Some(hash.unprotected_as_encoded().to_string());

        query!(
            "Update users SET password_hash = $1 WHERE username = $2",
            self.password_hash,
            self.username
        )
        .execute(pool)
        .await
    }

    pub fn validate_password(&self, input_password: &str) -> bool {
        match &self.password_hash {
            Some(h) => {
                let hash = match PasswordHash::from_encoded(h) {
                    Err(_) => return false,
                    Ok(p) => p,
                };
                let input_password =
                    Password::from_slice(input_password.as_bytes()).unwrap_or_default();
                hash_password_verify(&hash, &input_password).is_ok()
            }
            None => false,
        }
    }

    pub async fn count_admins(pool: &PgPool) -> Result<i64, sqlx::Error> {
        let count =
            sqlx::query_scalar!(r#"SELECT count(*) as "count!" FROM users WHERE admin = 't'"#)
                .fetch_one(pool)
                .await?;
        Ok(count)
    }
}

pub async fn initialize_admin(pool: &PgPool) -> Result<(), sqlx::Error> {
    if User::count_admins(pool).await? >= 1 {
        println!("Admin user exists.");
        return Ok(());
    }

    log::info!("Creating admin user");
    let mut pw_from_env = false;
    let admin_username = env::var("TASSO_ADMIN_USER").unwrap_or_else(|_| "admin".into());
    let admin_pass = match env::var("TASSO_ADMIN_PASS") {
        Ok(v) => {
            pw_from_env = true;
            v
        }
        Err(_) => pwgen::generate("full", 14),
    };

    let mut admin = User {
        username: admin_username.clone(),
        admin: true,
        ..Default::default()
    };
    admin.create(pool).await?;
    admin.set_password(pool, &admin_pass).await?;
    log::info!(
        "Admin user created. username: '{}', password: '{}'",
        &admin_username,
        if pw_from_env {
            "<FROM ENVIRONMENT>"
        } else {
            &admin_pass
        }
    );

    Ok(())
}

fn fake_validate() {
    User {
        username: "_".into(),
        password_hash: Some("$argon2i$v=19$m=65536,t=3,p=1$4MHN0rGSFfQxAfCHfD1Ncg$+psDULFfyWAaQ6H/tI/KH5LMcfZBjlpxOyFXJIa4ezM".into()),
        ..Default::default()
    }.validate_password("hunter2");
}

#[cfg(test)]
mod user_tests {
    use crate::models::user::{Credentials, User};
    use crate::models::Resource;
    use anyhow::Result;
    use sqlx::{query, PgPool};

    #[sqlx::test()]
    async fn test_create_user(pool: PgPool) -> Result<()> {
        let username = "user1";
        let email = "user1@email.com";
        let user = User {
            username: username.to_string(),
            email: Some(email.to_string()),
            admin: true,
            ..Default::default()
        };
        user.create(&pool).await?;
        let got_user = User::get_by_username(&pool, &user.username).await?;

        assert_eq!(user.username, got_user.username);
        assert_eq!(user.email, got_user.email);
        assert_eq!(user.admin, got_user.admin);
        assert_eq!(user.password_hash, got_user.password_hash);

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn test_get_user(pool: PgPool) -> Result<()> {
        let user = User::get_by_username(&pool, "user1").await?;

        assert_eq!("user1", user.username);
        assert_eq!("User", user.lastname.unwrap());
        assert_eq!("Juan", user.firstname.unwrap());
        assert_eq!("user@email.com", user.email.unwrap());
        assert_eq!(
            "46a9d5bde718bf366178313019f04a753bad00685d38e3ec81c8628f35dfcb1b",
            user.password_hash.unwrap()
        );
        assert!(!user.admin);

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn test_update_user(pool: PgPool) -> Result<()> {
        let new_firstname = "John";
        let mut user = User::get_by_username(&pool, "user1").await?;
        user.firstname = Some(new_firstname.to_string());
        user.update(&pool).await?;

        let updated_user = User::get_by_username(&pool, "user1").await?;

        assert_eq!(new_firstname, updated_user.firstname.unwrap());

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn test_delete_user(pool: PgPool) -> Result<()> {
        let user = User::get_by_username(&pool, "user1").await?;
        user.delete(&pool).await?;

        let res = query("SELECT * FROM users WHERE username = $1")
            .bind(user.username)
            .execute(&pool)
            .await?;

        assert_eq!(res.rows_affected(), 0);

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn test_set_password(pool: PgPool) -> Result<()> {
        let mut user = User::get_by_username(&pool, "userNoPass").await?;
        let password = "itsagoodpass2";
        user.set_password(&pool, password).await?;

        user = User::get_by_username(&pool, "userNoPass").await?;

        assert!(user.password_hash.is_some());

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn test_validate_password(pool: PgPool) -> Result<()> {
        let mut user = User::get_by_username(&pool, "userNoPass").await?;
        let password = "itsagoodpass2";
        user.set_password(&pool, password).await?;

        user = User::get_by_username(&pool, "userNoPass").await?;

        assert!(user.validate_password(password));
        assert!(!user.validate_password("itsabadpass3"));

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn test_authenticate(pool: PgPool) -> Result<()> {
        let username = "userCanLogin".to_string();
        let password = "abc123".to_string();
        let creds = Credentials {
            username: username.clone(),
            password,
        };
        let auth_user = User::authenticate(&pool, creds).await.unwrap();

        assert_eq!(User::get_by_username(&pool, &username).await?, auth_user);

        Ok(())
    }
}

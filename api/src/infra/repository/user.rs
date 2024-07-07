use anyhow::Result;
use sqlx::{PgPool, Postgres};

use crate::domain::{user::User, vo::providers::AuthProvider};

#[trait_variant::make]
pub trait UserRepository {
    async fn find_by_provider_user_id(
        &self,
        provider: &AuthProvider,
        user_id: &str,
    ) -> Result<Option<User>>;
    async fn find_by_user_id(&self, user_id: &str) -> Result<User>;
    // async fn create_or_update(
    //     &self,
    //     provider: &AuthProvider,
    //     user_id: &str,
    //     user_name: &str,
    // ) -> Result<()>;
}

impl UserRepository for PgPool {
    async fn find_by_provider_user_id(
        &self,
        provider: &AuthProvider,
        user_id: &str,
    ) -> Result<Option<User>> {
        match provider {
            AuthProvider::Google => {
                let user = sqlx::query_as::<Postgres, User>(
                    r"SELECT * FROM internal_users WHERE google_id = $1",
                )
                .bind(user_id)
                .fetch_one(self)
                .await?;

                return Ok(Some(user));

                // if user.github_id.is_none() {
                //     sqlx::query(
                //         r"INSERT INTO internal_users (google_id, google_email) VALUES ($1, $2)",
                //     )
                //     .bind(user_id)
                //     .bind(user_name)
                //     .execute(&mut *transaction)
                //     .await?;
                // }
            }
            AuthProvider::Github => {
                let user = sqlx::query_as::<Postgres, User>(
                    r"SELECT * FROM internal_users WHERE github_id = $1",
                )
                .bind(user_id)
                .fetch_one(self)
                .await?;

                return Ok(Some(user));

                // if user.github_id.is_none() {
                //     sqlx::query(
                //         r"INSERT INTO internal_users (github_id, github_username) VALUES ($1, $2)",
                //     )
                //     .bind(user_id)
                //     .bind(user_name)
                //     .execute(&mut *transaction)
                //     .await?;
                // }
            }
        }
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<User> {
        let user = sqlx::query_as::<Postgres, User>(
            r"
            SELECT * FROM internal_users
            WHERE id = $1
            ",
        )
        .bind(user_id)
        .fetch_one(self)
        .await?;

        Ok(user)
    }

    // async fn create_or_update(
    //     &self,
    //     provider: &AuthProvider,
    //     user_id: &str,
    //     user_name: &str,
    // ) -> Result<()> {
    // }
}

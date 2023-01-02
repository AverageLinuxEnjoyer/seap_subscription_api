use anyhow::Result;
use sqlx::{query_as, PgPool};

use crate::utils::Email;

use super::User;

impl User {
    pub async fn update(pool: &PgPool, email: Email) -> Result<User> {
        let email: String = email.try_into()?;

        Ok(query_as!(
            User,
            r#"
            SELECT
                id as "id!",
                email as "email!",
                created_at as "created_at!"
            FROM create_user($1)"#,
            email
        )
        .fetch_one(pool)
        .await?)
    }
}

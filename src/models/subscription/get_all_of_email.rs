use crate::utils::{Email, Pagination};

use super::Subscription;
use anyhow::Result;
use sqlx::{query_as, PgPool};

impl Subscription {
    pub async fn get_all_of_email(pool: &PgPool, email: Email) -> Result<Vec<Subscription>> {
        let email: String = email.try_into()?;

        Ok(query_as!(
            Subscription,
            r#"SELECT
                id as "id!", 
                id_user as "id_user!",
                min_price, 
                max_price, 
                title_keywords,
                desc_keywords,
                additional_info_keywords
            FROM get_subscriptions()
            WHERE id_user IN 
                (SELECT id FROM users WHERE email = $1)"#,
            email
        )
        .fetch_all(pool)
        .await?)
    }
}

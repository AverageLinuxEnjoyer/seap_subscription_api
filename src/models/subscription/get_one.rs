use super::Subscription;
use anyhow::Result;
use sqlx::{query_as, PgPool};

impl Subscription {
    pub async fn get_one(pool: &PgPool, id: u32) -> Result<Option<Subscription>> {
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
            FROM get_subscriptions() WHERE id=$1"#,
            id as i64
        )
        .fetch_optional(pool)
        .await?)
    }
}

use super::Subscription;
use anyhow::Result;
use sqlx::{query_as, PgPool};

impl Subscription {
    pub async fn update(pool: &PgPool, sub: Subscription) -> Result<Subscription> {
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
            FROM update_subscription($1,$2,$3,$4,$5,$6,$7)
            "#,
            sub.id,
            sub.id_user,
            sub.min_price,
            sub.max_price,
            sub.title_keywords.as_deref(),
            sub.desc_keywords.as_deref(),
            sub.additional_info_keywords.as_deref()
        )
        .fetch_one(pool)
        .await?)
    }
}

use super::Subscription;
use anyhow::Result;
use sqlx::{query, query_as, PgPool};

impl Subscription {
    pub async fn delete(pool: &PgPool, id: u32) -> Result<Option<Subscription>> {
        let sub = Subscription::get_one(pool, id).await?;

        query!("DELETE FROM subscriptions WHERE id = $1", id as i32)
            .execute(pool)
            .await?;

        Ok(sub)
    }
}

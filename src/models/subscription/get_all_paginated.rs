use crate::utils::Pagination;

use super::Subscription;
use anyhow::Result;
use sqlx::{query_as, PgPool};

impl Subscription {
    pub async fn get_all_paginated(
        pool: &PgPool,
        pagination: &Pagination,
    ) -> Result<Vec<Subscription>> {
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
            LIMIT $1
            OFFSET $2"#,
            pagination.count as i64,
            pagination.start_index as i64
        )
        .fetch_all(pool)
        .await?)
    }
}

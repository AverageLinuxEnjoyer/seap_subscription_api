use crate::utils::{Email, Pagination};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NoID;

pub type ID = i32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription<IdType> {
    // #[serde(default)]
    pub id: IdType,
    pub id_user: i32,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
    pub title_keywords: Option<Vec<String>>,
    pub desc_keywords: Option<Vec<String>>,
    pub additional_info_keywords: Option<Vec<String>>,
}

impl Subscription<NoID> {
    pub fn with_id(self, id: ID) -> Subscription<ID> {
        Subscription {
            id,
            id_user: self.id_user,
            min_price: self.min_price,
            max_price: self.max_price,
            title_keywords: self.title_keywords,
            desc_keywords: self.desc_keywords,
            additional_info_keywords: self.additional_info_keywords,
        }
    }
}

impl Subscription<ID> {
    pub fn without_id(self) -> Subscription<NoID> {
        Subscription {
            id: NoID,
            id_user: self.id_user,
            min_price: self.min_price,
            max_price: self.max_price,
            title_keywords: self.title_keywords,
            desc_keywords: self.desc_keywords,
            additional_info_keywords: self.additional_info_keywords,
        }
    }
}

pub async fn create(pool: &PgPool, sub: Subscription<NoID>) -> Result<Subscription<ID>> {
    Ok(query_as!(
        Subscription,
        r#"
            SELECT
                id as "id!", 
                id_user as "id_user!",
                min_price, 
                max_price, 
                title_keywords,
                desc_keywords,
                additional_info_keywords
            FROM create_subscription($1,$2,$3,$4,$5,$6)"#,
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

pub async fn get_one(pool: &PgPool, id: ID) -> Result<Subscription<ID>> {
    Ok(query_as!(
        Subscription::<ID>,
        r#"SELECT
                id as "id!", 
                id_user as "id_user!",
                min_price, 
                max_price, 
                title_keywords,
                desc_keywords,
                additional_info_keywords
            FROM get_subscriptions() WHERE id=$1"#,
        id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_paginated(
    pool: &PgPool,
    pagination: &Pagination,
) -> Result<Vec<Subscription<ID>>> {
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

pub async fn get_all_of_email(pool: &PgPool, email: Email) -> Result<Vec<Subscription<ID>>> {
    let email: String = email.try_into()?;

    Ok(query_as!(
        Subscription::<ID>,
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

pub async fn update(pool: &PgPool, sub: Subscription<ID>) -> Result<Subscription<ID>> {
    Ok(query_as!(
        Subscription::<ID>,
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

pub async fn delete(pool: &PgPool, id: ID) -> Result<Subscription<ID>> {
    Ok(query_as!(
        Subscription::<ID>,
        r#"SELECT 
                id as "id!", 
                id_user as "id_user!",
                min_price, 
                max_price, 
                title_keywords,
                desc_keywords,
                additional_info_keywords 
            FROM delete_subscription($1)"#,
        id
    )
    .fetch_one(pool)
    .await?)
}

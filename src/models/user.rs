use crate::utils::{Email, Pagination};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};
use time::{OffsetDateTime, PrimitiveDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub created_at: OffsetDateTime,
}

pub async fn create(pool: &PgPool, email: Email) -> Result<User> {
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

pub async fn get_one(pool: &PgPool, id: i32) -> Result<User> {
    Ok(query_as!(
        User,
        r#"
            SELECT * FROM users WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_paginated(pool: &PgPool, pagination: Pagination) -> Result<Vec<User>> {
    Ok(query_as!(
        User,
        "SELECT * FROM users LIMIT $1 OFFSET $2",
        pagination.count as i64,
        pagination.start_index as i64
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_by_email(pool: &PgPool, email: Email) -> Result<User> {
    let email: String = email.try_into()?;

    Ok(
        query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn update(pool: &PgPool, user: User) -> Result<User> {
    Ok(query_as!(
        User,
        r#"
            SELECT
                id as "id!",
                email as "email!",
                created_at as "created_at!"
            FROM update_user($1, $2, $3) 
        "#,
        user.id,
        user.email,
        Option::<OffsetDateTime>::None
    )
    .fetch_one(pool)
    .await?)
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<User> {
    Ok(query_as!(
        User,
        r#"
            SELECT
                id as "id!",
                email as "email!",
                created_at as "created_at!"
            FROM delete_user($1) 
        "#,
        id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn delete_by_email(pool: &PgPool, email: Email) -> Result<User> {
    let email: String = email.try_into()?;

    Ok(query_as!(
        User,
        r#"
            SELECT
                id as "id!",
                email as "email!",
                created_at as "created_at!"
            FROM delete_user_by_email($1) 
        "#,
        email
    )
    .fetch_one(pool)
    .await?)
}

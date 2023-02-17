use crate::utils::{Email, Pagination};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};
use time::{OffsetDateTime, PrimitiveDateTime};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

pub async fn create_or_return(pool: &PgPool, email: Email) -> Result<User> {
    let email: String = email.try_into()?;

    Ok(query_as!(
        User,
        r#"
            SELECT
                id as "id!",
                email as "email!",
                created_at as "created_at!"
            FROM create_or_return_user($1)"#,
        email
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_one(pool: &PgPool, id: usize) -> Result<User> {
    let id: i32 = id.try_into()?;
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
        i64::from(pagination.count),
        i64::from(pagination.start_index)
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

pub async fn delete(pool: &PgPool, id: usize) -> Result<User> {
    let id: i32 = id.try_into()?;

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

#[cfg(test)]
mod test {
    use anyhow::Result;
    use sqlx::PgPool;
    use time::OffsetDateTime;

    use crate::models::user::{
        create, delete, delete_by_email, get_by_email, get_one, get_paginated, update,
    };
    use crate::models::User;
    use crate::utils::{Email, Pagination};

    #[sqlx::test(fixtures("users"))]
    async fn test_create(pool: PgPool) -> Result<()> {
        {
            let res = create(
                &pool,
                Email {
                    email: "test@email.com".to_string(),
                },
            )
            .await?;

            assert_eq!(res.email, "test@email.com");
        }

        {
            let res = create(
                &pool,
                Email {
                    email: "definitely not valid".to_string(),
                },
            )
            .await;

            assert!(res.is_err());
        }

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn test_get_one(pool: PgPool) -> Result<()> {
        {
            let res = get_one(&pool, 1).await?;

            assert_eq!(res.id, 1);
        }

        {
            let res = get_one(&pool, 100).await;

            assert!(res.is_err());
        }

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn test_get_paginated(pool: PgPool) -> Result<()> {
        {
            let res = get_paginated(
                &pool,
                Pagination {
                    count: 1,
                    start_index: 0,
                },
            )
            .await?;

            assert_eq!(res.len(), 1);
            assert_eq!(res[0].id, 1);
        }

        {
            let res = get_paginated(
                &pool,
                Pagination {
                    count: 5,
                    start_index: 0,
                },
            )
            .await?;

            assert_eq!(res.len(), 2);
            assert_eq!(res[0].id, 1);
            assert_eq!(res[1].id, 2);
        }

        {
            let res = get_paginated(
                &pool,
                Pagination {
                    count: 0,
                    start_index: 0,
                },
            )
            .await?;

            assert_eq!(res.len(), 0);
        }

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn test_get_by_email(pool: PgPool) -> Result<()> {
        {
            let res = get_by_email(
                &pool,
                Email {
                    email: "test@test.test".to_string(),
                },
            )
            .await?;

            assert_eq!(res.id, 1);
        }

        {
            let res = get_by_email(
                &pool,
                Email {
                    email: "thing@test.test".to_string(),
                },
            )
            .await;

            assert!(res.is_err());
        }

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn test_update(pool: PgPool) -> Result<()> {
        {
            let time = OffsetDateTime::now_utc();

            let res = update(
                &pool,
                User {
                    id: 1,
                    email: "test2@test2.test2".to_string(),
                    created_at: time,
                },
            )
            .await?;

            assert_eq!(res.id, 1);
            assert_eq!(res.email, "test2@test2.test2");
            assert_eq!(res.created_at.to_hms(), time.to_hms());
        }

        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn test_delete(pool: PgPool) -> Result<()> {
        {
            let res = delete(&pool, 1).await?;

            assert_eq!(res.id, 1);
            assert_eq!(res.email, "test@test.test");
        }

        {
            let res = delete(&pool, 100).await;

            assert!(res.is_err());
        }

        Ok(())
    }
}

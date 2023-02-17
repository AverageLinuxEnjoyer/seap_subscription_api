use crate::utils::{Email, Pagination};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subscription {
    // #[serde(default)]
    pub id: i32,
    pub id_user: i32,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
    pub title_keywords: Option<Vec<String>>,
    pub desc_keywords: Option<Vec<String>>,
    pub additional_info_keywords: Option<Vec<String>>,
}

pub async fn create(pool: &PgPool, sub: &Subscription) -> Result<Subscription> {
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

pub async fn get_one(pool: &PgPool, id: usize) -> Result<Subscription> {
    let id: i32 = id.try_into()?;

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
        id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_paginated(pool: &PgPool, pagination: &Pagination) -> Result<Vec<Subscription>> {
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
        i64::from(pagination.count),
        i64::from(pagination.start_index)
    )
    .fetch_all(pool)
    .await?)
}

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

pub async fn delete(pool: &PgPool, id: usize) -> Result<Subscription> {
    let id: i32 = id.try_into()?;

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
            FROM delete_subscription($1)"#,
        id
    )
    .fetch_one(pool)
    .await?)
}

#[cfg(test)]
mod test {
    use crate::{
        models::{
            subscription::{create, delete, get_all_of_email, get_one, get_paginated, update},
            Subscription,
        },
        utils::{Email, Pagination},
    };
    use anyhow::Result;
    use sqlx::PgPool;

    #[ignore]
    #[sqlx::test(fixtures("users"))]
    async fn test_create(pool: PgPool) -> Result<()> {
        {
            let s = Subscription {
                id: 1,
                id_user: 1,
                min_price: Some(100),
                max_price: None,
                title_keywords: Some(vec!["kw1".into(), "kw2".into(), "kw3".into()]),
                desc_keywords: None,
                additional_info_keywords: None,
            };

            let res = create(&pool, &s).await?;

            assert_eq!(res, s);
        }

        {
            let s = Subscription {
                id: 2,
                id_user: 1,
                min_price: None,
                max_price: None,
                title_keywords: None,
                desc_keywords: Some(vec![]),
                additional_info_keywords: None,
            };

            let res = create(&pool, &s).await?;

            assert_eq!(res.desc_keywords, None);
        }

        Ok(())
    }

    #[ignore]
    #[sqlx::test(fixtures("users", "subscriptions"))]
    async fn test_get_one(pool: PgPool) -> Result<()> {
        {
            let s = Subscription {
                id: 1,
                id_user: 1,
                min_price: None,
                max_price: None,
                title_keywords: None,
                desc_keywords: None,
                additional_info_keywords: None,
            };

            let res = get_one(&pool, 1).await?;

            assert_eq!(res, s);
        }

        {
            let s = Subscription {
                id: 2,
                id_user: 1,
                min_price: None,
                max_price: None,
                title_keywords: None,
                desc_keywords: None,
                additional_info_keywords: None,
            };

            let res = get_one(&pool, 2).await?;

            assert_eq!(res, s);
        }

        {
            let res = get_one(&pool, 4).await;

            assert!(res.is_err());
        }

        Ok(())
    }

    #[ignore]
    #[sqlx::test(fixtures("users", "subscriptions"))]
    async fn test_get_paginated(pool: PgPool) -> Result<()> {
        {
            let s = Subscription {
                id: 1,
                id_user: 1,
                min_price: None,
                max_price: None,
                title_keywords: None,
                desc_keywords: None,
                additional_info_keywords: None,
            };

            let res = get_paginated(
                &pool,
                &Pagination {
                    start_index: 0,
                    count: 5,
                },
            )
            .await?;

            assert_eq!(res.len(), 3);
            assert!(res.contains(&s));
        }

        {
            let res = get_paginated(
                &pool,
                &Pagination {
                    start_index: 0,
                    count: 0,
                },
            )
            .await?;

            assert_eq!(res.len(), 0);
        }

        Ok(())
    }

    #[ignore]
    #[sqlx::test(fixtures("users", "subscriptions"))]
    async fn test_get_all_of_email(pool: PgPool) -> Result<()> {
        {
            let res = get_all_of_email(
                &pool,
                Email {
                    email: "test@test.test".into(),
                },
            )
            .await?;

            assert_eq!(res.len(), 2);
        }

        {
            let res = get_all_of_email(
                &pool,
                Email {
                    email: "foo@bar.com".into(),
                },
            )
            .await?;

            assert_eq!(res.len(), 1);
        }

        Ok(())
    }

    #[ignore]
    #[sqlx::test(fixtures("users", "subscriptions"))]
    async fn test_update(pool: PgPool) -> Result<()> {
        {
            let sub = Subscription {
                id: 1,
                id_user: 1,
                min_price: Some(100),
                max_price: None,
                title_keywords: Some(vec!["kw1".into(), "kw2".into(), "kw3".into()]),
                desc_keywords: None,
                additional_info_keywords: None,
            };

            let res = update(&pool, sub.clone()).await?;

            assert_eq!(res, sub);
        }

        {
            let sub = Subscription {
                id: 2,
                id_user: 1,
                min_price: None,
                max_price: None,
                title_keywords: None,
                desc_keywords: Some(vec![]),
                additional_info_keywords: None,
            };

            let res = update(&pool, sub.clone()).await?;

            assert_eq!(res.desc_keywords, None);
        }

        Ok(())
    }

    #[ignore]
    #[sqlx::test(fixtures("users", "subscriptions"))]
    async fn test_delete(pool: PgPool) -> Result<()> {
        {
            let sub = Subscription {
                id: 1,
                id_user: 1,
                min_price: None,
                max_price: None,
                title_keywords: None,
                desc_keywords: None,
                additional_info_keywords: None,
            };

            let res = delete(&pool, 1).await?;

            assert_eq!(res, sub);
        }

        {
            let res = delete(&pool, 4).await;

            assert!(res.is_err());
        }

        Ok(())
    }
}

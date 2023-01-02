pub mod handlers;
pub mod models;
pub mod utils;

use axum::{
    extract::State,
    routing::{delete, get, post, put},
    Router,
};
use shuttle_service::tracing::info;
use sqlx::{query, query_as, Executor, PgPool};
use sync_wrapper::SyncWrapper;

use crate::{
    handlers::{create_subscription, delete_subscription, get_subscriptions, update_subscription},
    models::Subscription,
};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_service::main]
async fn axum(/* #[shuttle_shared_db::Postgres] pool: PgPool */) -> shuttle_service::ShuttleAxum {
    let pool = PgPool::connect("postgres://postgres:postgres@localhost/seap_subscriptions")
        .await
        .unwrap();

    let _ = pool.execute(include_str!("../users.sql")).await.unwrap();
    let _ = pool
        .execute(include_str!("../subscriptions.sql"))
        .await
        .unwrap();
    info!("Succesfully loaded schema into db.");

    query!(
        "INSERT INTO users (email) VALUES ($1)",
        "galanlucian002@gmail.com"
    )
    .execute(&pool)
    .await
    .unwrap();

    let keywords = vec![
        "idk".to_string(),
        "something".to_string(),
        "ceva".to_string(),
    ];
    let keywords2 = vec!["lol".to_string()];

    let a = Subscription {
        id: 0,
        id_user: 1,
        min_price: Option::<i32>::None,
        max_price: Some(23),
        title_keywords: Some(keywords.clone()),
        desc_keywords: Option::<Vec<String>>::None,
        additional_info_keywords: Some(keywords2.clone()),
    };

    // let a = a.create(&pool).await;
    let a = Subscription::create(&pool, a).await;
    let resa = format!("{:?}", a);
    info!(resa);

    let a = a.unwrap();

    let b = Subscription {
        id: 0,
        id_user: 1,
        min_price: Some(5000),
        max_price: Some(620),
        title_keywords: Some(vec!["ceva".to_string(), "altceva".to_string()]),
        desc_keywords: Some(vec![
            "kw1".to_string(),
            "kw2".to_string(),
            "kw3".to_string(),
        ]),
        additional_info_keywords: Some(vec![]),
    };

    let b = Subscription::create(&pool, b).await;
    let resb = format!("{:?}", b);
    info!(resb);

    let mut b = b.unwrap();

    b.id = a.id;

    let c = Subscription::update(&pool, b).await;

    let resc = format!("{:?}", c);
    info!(resc);

    // let s = format!("{:?}", subscription);
    // info!(s);
    let router = Router::new()
        .route("/hello", get(hello_world))
        .route("/subscriptions/:id", get(get_subscriptions))
        .route("/subscriptions", get(get_subscriptions))
        .route("/subscriptions", post(create_subscription))
        .route("/subscriptions/:id", put(update_subscription))
        .route("/subscriptions/:id", delete(delete_subscription))
        .with_state(pool);

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}

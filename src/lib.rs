pub mod handlers;
pub mod models;
pub mod utils;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use shuttle_service::tracing::info;
use sqlx::{query, Executor, PgPool};
use sync_wrapper::SyncWrapper;
use tower_http::cors::{Any, CorsLayer};

#[shuttle_service::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_service::ShuttleAxum {
    // let pool = PgPool::connect("postgres://postgres:postgres@localhost/seap_subscriptions")
    //     .await
    //     .expect("Couldn't connect to the database.");

    pool.execute(include_str!("../migrations/20230203192555_users.sql"))
        .await
        .expect("Couldn't execute users.sql");

    pool.execute(include_str!(
        "../migrations/20230203192601_subscriptions.sql"
    ))
    .await
    .expect("Couldn't execute subscriptions.sql");

    info!("Succesfully loaded schema into db.");

    let router = Router::new()
        .route("/subscriptions", post(handlers::create_subscription))
        .route("/subscriptions", get(handlers::get_subscriptions))
        .route("/subscriptions/:id", get(handlers::get_subscription_by_id))
        .route("/subscriptions/:id", put(handlers::update_subscription))
        .route("/subscriptions/:id", delete(handlers::delete_subscription))
        //
        .route("/users", post(handlers::create_user))
        .route("/users/:id", get(handlers::get_user_by_id))
        .route("/users", get(handlers::get_users))
        .route("/users/:id", put(handlers::update_user))
        .route("/users/:id", delete(handlers::delete_user))
        .with_state(pool)
        .layer(CorsLayer::permissive());

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}

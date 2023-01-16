pub mod handlers;
pub mod models;
pub mod utils;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use handlers::get_subscription_by_id;
use models::{NoID, ID};
use shuttle_service::tracing::info;
use sqlx::{query, Executor, PgPool};
use sync_wrapper::SyncWrapper;

use crate::handlers::{
    create_subscription, create_user, delete_subscription, get_subscriptions, get_user_by_id,
    get_users, update_subscription,
};

#[shuttle_service::main]
async fn axum(/* #[shuttle_shared_db::Postgres] pool: PgPool */) -> shuttle_service::ShuttleAxum {
    let pool = PgPool::connect("postgres://postgres:postgres@localhost/seap_subscriptions")
        .await
        .expect("Couldn't connect to the database.");

    let _ = pool
        .execute(include_str!("../users.sql"))
        .await
        .expect("Couldn't execute users.sql");
    let _ = pool
        .execute(include_str!("../subscriptions.sql"))
        .await
        .expect("Couldn't execute subscriptions.sql");

    info!("Succesfully loaded schema into db.");

    let router = Router::new()
        .route("/subscriptions", post(create_subscription))
        .route("/subscriptions/:id", get(get_subscriptions))
        .route("/subscriptions", get(get_subscription_by_id))
        .route("/subscriptions/:id", put(update_subscription))
        .route("/subscriptions/:id", delete(delete_subscription))
        //
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user_by_id))
        .route("/users", get(get_users))
        .route("/users/:id", put(update_subscription))
        .route("/users/:id", delete(delete_subscription))
        .with_state(pool);

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}

use crate::models::subscription;
use crate::models::Subscription;
use axum::{extract::State, http::StatusCode, Json};
use axum_macros::debug_handler;
use serde_json::{json, Value};
use sqlx::PgPool;

#[debug_handler]
pub async fn create_subscription(
    State(pool): State<PgPool>,
    Json(payload): Json<Subscription>,
) -> (StatusCode, Json<Value>) {
    match subscription::create(&pool, &payload).await {
        Ok(sub) => match serde_json::to_value(sub) {
            Ok(val) => (StatusCode::ACCEPTED, Json(val)),
            Err(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"Error": err.to_string()})),
            ),
        },
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"Error": err.to_string()})),
        ),
    }
}

use crate::models::subscription;
use crate::models::Subscription;
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn update_subscription(
    State(pool): State<PgPool>,
    Path(id): Path<usize>,
    Json(sub): Json<Subscription>,
) -> (StatusCode, Json<Value>) {
    match subscription::update(&pool, sub).await {
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

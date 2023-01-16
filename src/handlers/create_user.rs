use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{models::user, utils::Email};

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(email): Json<Email>,
) -> (StatusCode, Json<Value>) {
    match user::create(&pool, email).await {
        Ok(user) => match serde_json::to_value(user) {
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

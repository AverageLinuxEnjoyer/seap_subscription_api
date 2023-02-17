use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::models::user;

pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<usize>,
) -> (StatusCode, Json<Value>) {
    match user::delete(&pool, id).await {
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

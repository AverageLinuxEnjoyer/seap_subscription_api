use crate::{
    models::{user, User},
    utils::Email,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use serde_json::{json, Value};
use sqlx::PgPool;
use time::OffsetDateTime;

#[debug_handler]
pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<usize>,
    Json(email): Json<Email>,
) -> (StatusCode, Json<Value>) {
    let email: String = match email.try_into() {
        Ok(val) => val,
        Err(err) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"Error": err.to_string()})),
            )
        }
    };

    let id = match id.try_into() {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"Error": "The provided id is too large."})),
            )
        }
    };

    let user = User {
        id,
        email,
        created_at: OffsetDateTime::now_utc(),
    };

    match user::update(&pool, user).await {
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

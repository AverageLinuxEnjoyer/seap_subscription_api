use crate::{
    models::{user, User, ID},
    utils::Email,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use sqlx::PgPool;
use time::{OffsetDateTime, PrimitiveDateTime};

pub async fn update_user(
    State(pool): State<PgPool>,
    Json(email): Json<Email>,
    Path(id): Path<ID>,
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

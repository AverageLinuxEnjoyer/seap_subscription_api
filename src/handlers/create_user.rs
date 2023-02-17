use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{models::user, utils::Email};

#[derive(Serialize, Deserialize)]
pub struct OrReturn {
    pub or_return: bool,
}

#[debug_handler]
pub async fn create_user(
    State(pool): State<PgPool>,
    or_return: Option<Query<OrReturn>>,
    Json(email): Json<Email>,
) -> (StatusCode, Json<Value>) {
    let or_return = or_return.map(|q| q.or_return).unwrap_or(false);

    let res = match or_return {
        true => user::create_or_return(&pool, email).await,
        false => user::create(&pool, email).await,
    };

    match res {
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

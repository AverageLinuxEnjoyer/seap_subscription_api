use anyhow::Error;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::models::Subscription;

pub async fn delete_subscription(
    State(pool): State<PgPool>,
    Path(id): Path<u32>,
) -> (StatusCode, Json<Value>) {
    match Subscription::delete(&pool, id).await {
        Ok(maybe_sub) => match maybe_sub {
            Some(sub) => match serde_json::to_value(sub) {
                Ok(val) => (StatusCode::ACCEPTED, Json(val)),
                Err(err) => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({"Error": err.to_string()})),
                ),
            },
            None => (
                StatusCode::NOT_FOUND,
                Json(json!({"Error": "No subscription found with this id."})),
            ),
        },
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"Error": err.to_string()})),
        ),
    }
}

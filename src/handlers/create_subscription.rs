use crate::models::Subscription;
use anyhow::Error;
use anyhow::Result;
use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn create_subscription(
    State(pool): State<PgPool>,
    payload: Json<Value>,
) -> (StatusCode, Json<Value>) {
    let payload = match preprocess_payload(payload.0, &pool) {
        Ok(val) => val,
        Err(err) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"Error": err.to_string()})),
            )
        }
    };

    let as_sub: Subscription = match serde_json::from_value(payload) {
        Ok(val) => val,
        Err(err) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"Error": err.to_string()})),
            )
        }
    };

    match Subscription::create(&pool, as_sub).await {
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

fn preprocess_payload(payload: Value, pool: &PgPool) -> Result<Value> {
    let mut payload = match payload.as_object() {
        Some(obj) => obj,
        None => {
            return Err(Error::msg(
                "Subscription couldn't be parsed from provided json.",
            ))
        }
    }
    .to_owned();

    if let None = payload.get("id_user") {
        // Subscription::create(&pool, sub)
        todo!();
    }

    payload.insert("id".to_string(), json!(0));

    Ok(json!(payload))
}

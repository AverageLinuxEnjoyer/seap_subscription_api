use crate::models::Subscription;
use anyhow::Error;
use anyhow::Result;
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn update_subscription(
    State(pool): State<PgPool>,
    Path(id): Path<u32>,
    Json(val): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let mut sub = match to_subscription(val) {
        Ok(sub) => sub,
        Err(err) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"Error": err.to_string()})),
            )
        }
    };

    sub.id = id as i32;

    match Subscription::update(&pool, sub).await {
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

fn to_subscription(val: Value) -> Result<Subscription> {
    let mut as_map = match val.as_object() {
        Some(obj) => obj,
        None => {
            return Err(Error::msg(
                "Subscription couldn't be parsed from provided json.",
            ))
        }
    }
    .to_owned();

    as_map.insert("id".to_string(), json!(0));
    let as_json = json!(as_map);

    Ok(serde_json::from_value::<Subscription>(as_json)?)
}

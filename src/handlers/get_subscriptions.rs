use crate::{
    models::Subscription,
    utils::{Email, Pagination},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn get_subscriptions(
    State(pool): State<PgPool>,
    pagination: Option<Query<Pagination>>,
    email: Option<Query<Email>>,
    id: Option<Path<u32>>,
) -> (StatusCode, Json<Value>) {
    match (pagination, email, id) {
        (None, None, Some(Path(id))) => {
            let sub = Subscription::get_one(&pool, id).await;

            match sub {
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
        (None, Some(Query(email)), None) => {
            let subs = Subscription::get_all_of_email(&pool, email).await;

            match subs {
                Ok(subs) => match serde_json::to_value(subs) {
                    Ok(val) => (StatusCode::ACCEPTED, Json(val)),
                    Err(err) => (
                        StatusCode::UNPROCESSABLE_ENTITY,
                        Json(json!({"Error": err.to_string()})),
                    ),
                },
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"Error": err.to_string() })),
                ),
            }
        }
        (Some(Query(pagination)), None, None) => {
            let subs = Subscription::get_all_paginated(&pool, &pagination).await;

            match subs {
                Ok(subs) => match serde_json::to_value(subs) {
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
        _ => {
            let msg = "Expected either: pagination query params, an email query param or an id path param.";

            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "Error": msg })),
            )
        }
    }
}

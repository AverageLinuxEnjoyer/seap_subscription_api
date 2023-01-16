use crate::{
    models::{subscription, ID},
    utils::{Email, Pagination},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn get_subscription_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<ID>,
) -> (StatusCode, Json<Value>) {
    match subscription::get_one(&pool, id).await {
        Ok(sub) => match serde_json::to_value(sub) {
            Ok(val) => (StatusCode::ACCEPTED, Json(val)),
            Err(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"Error": err.to_string()})),
            ),
        },
        Err(err) => (
            StatusCode::NOT_FOUND,
            Json(json!({"Error": err.to_string()})),
        ),
    }
}

pub async fn get_subscriptions(
    State(pool): State<PgPool>,
    pagination: Option<Query<Pagination>>,
    email: Option<Query<Email>>,
) -> (StatusCode, Json<Value>) {
    match (pagination, email) {
        (None, Some(Query(email))) => {
            let subs = subscription::get_all_of_email(&pool, email).await;

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
        (Some(Query(pagination)), None) => {
            let subs = subscription::get_paginated(&pool, &pagination).await;

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
            let msg = "Expected either an email or pagination query params.";

            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "Error": msg })),
            )
        }
    }
}

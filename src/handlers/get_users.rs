use crate::{
    models::user,
    utils::{Email, Pagination},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn get_user_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<usize>,
) -> (StatusCode, Json<Value>) {
    match user::get_one(&pool, id).await {
        Ok(user) => match serde_json::to_value(user) {
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

pub async fn get_users(
    State(pool): State<PgPool>,
    pagination: Option<Query<Pagination>>,
    email: Option<Query<Email>>,
) -> (StatusCode, Json<Value>) {
    match (pagination, email) {
        (None, Some(Query(email))) => match user::get_by_email(&pool, email).await {
            Ok(user) => match serde_json::to_value(user) {
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
        },
        (Some(Query(pagination)), None) => match user::get_paginated(&pool, pagination).await {
            Ok(users) => match serde_json::to_value(users) {
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
        },
        _ => {
            let msg = "Expected either an email or pagination query params.";

            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "Error": msg })),
            )
        }
    }
}

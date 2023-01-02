use serde::{Deserialize, Serialize};

mod create;
mod delete;
mod get_all_of_email;
mod get_all_paginated;
mod get_one;
mod update;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: i32,
    pub id_user: i32,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
    pub title_keywords: Option<Vec<String>>,
    pub desc_keywords: Option<Vec<String>>,
    pub additional_info_keywords: Option<Vec<String>>,
}

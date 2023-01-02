use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

mod create;
mod update;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub created_at: PrimitiveDateTime,
}

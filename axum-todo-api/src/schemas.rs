use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTodoSchema {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub completed: bool,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct TodoSchema {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

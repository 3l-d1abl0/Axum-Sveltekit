use crate::app_state::AppState;
use axum::extract::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::Router;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

//mod super::{app_state};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    id: i32,
    title: String,
    completed: bool,
    //image
}
/*
pub fn all_routes() -> Router {
    Router::new()
        .route("/create", post(create_todo))
        .with_state(app_state)
}
*/
pub async fn create_todo(
    State(app_state): State<Arc<AppState>>,
    Json(todo): Json<Todo>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, String)> {
    println!("Here !");

    //let config = &app_state.config;

    let created_todo = Todo {
        id: 1,
        title: todo.title,
        completed: todo.completed,
    };

    Ok((StatusCode::CREATED, Json(created_todo)))
}

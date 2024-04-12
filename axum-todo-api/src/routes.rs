use axum::extract::Json;
use axum::http::StatusCode;
use axum::routing::post;
use axum::Router;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
    //image
}

pub fn all_routes() -> Router {
    Router::new().route("/create", post(create_todo))
}

async fn create_todo(
    Json(todo): Json<Todo>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, String)> {
    println!("Here !");
    let created_todo = Todo {
        id: 1,
        title: todo.title,
        completed: todo.completed,
    };

    Ok((StatusCode::CREATED, Json(created_todo)))
}

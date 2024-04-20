use crate::app_state::AppState;
use axum::extract::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::schemas::{CreateTodoSchema, TodoSchema};
use serde_json::json;

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
    Json(body): Json<CreateTodoSchema>,
    //) -> Result<(StatusCode, Json<CreateTodoSchema>), (StatusCode, String)> {
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("Here !");

    let config = &app_state;

    //println!("{:?}", config);

    if let Some(pool) = &*app_state.db_pool.read().unwrap() {
        // Use the pool here

        let description = match body.description {
            Some(description) => description,
            None => "".to_string(),
        };

        let query_result = sqlx::query_as(
            r#"
            INSERT INTO todos (title, description)
            VALUES (?, ?)
            RETURNING id, created_at, updated_at
            "#,
        )
        .bind(body.title)
        .bind(description)
        .fetch_one(pool);

        let todo_result: Result<TodoSchema, sqlx::Error> = query_result.await;

        match todo_result {
            Ok(todo) => {
                let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                    "note": "Some_response".to_string()
                })});

                return Ok((StatusCode::CREATED, Json(note_response)));
                //Ok((StatusCode::CREATED, Json(JsonValue::from(todo))))
            }

            Err(err) => {
                let err_message = err.to_string();
                //let json_err = Json(JsonValue::String(err_message));

                let err_response = serde_json::json!({"status": "success","data": serde_json::json!({
                    "note": err_message
                })});
                return Ok((StatusCode::CREATED, Json(err_response)));
                /*match err {
                    Error::Database(dberr) => {
                        let status_code = match dberr.code().as_deref() {
                            Some("23000") => StatusCode::CONFLICT, // Duplicate key error
                            _ => StatusCode::INTERNAL_SERVER_ERROR,
                        };
                        Err((status_code, json_err))
                    }
                    _ => Err((StatusCode::INTERNAL_SERVER_ERROR, json_err)),
                }*/
            }
        }
    } else {
        //Handle it gracefully
        println!("Error Accessing Pool!!");

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!(" DB Error !")})),
        ));
    }

    let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "note": "Some_response".to_string()
    })});

    Ok((StatusCode::CREATED, Json(note_response)))
}

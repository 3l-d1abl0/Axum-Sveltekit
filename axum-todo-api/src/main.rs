use crate::app_state::AppState;
use crate::routes::create_todo;
use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::Serialize;
use tokio::net::TcpListener;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

mod app_state;
mod config;
mod routes;
mod schemas;

#[derive(Serialize, Clone)]
struct Hello {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Create new app state
    let new_app_state = AppState::new().await;

    println!("STATE {:?}", new_app_state);

    println!("DB: {}", new_app_state.config.database_url);

    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&new_app_state.config.database_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to Db !");
            pool
        }
        Err(err) => {
            println!("Failed to connect to Db: {:?}", err);
            std::process::exit(1);
        }
    };

    new_app_state.update_db_pool(pool).await;

    let hello = Hello {
        message: String::from("Hi"),
    };

    let app = Router::new()
        .route("/hello", get(|| async { Json(hello) }))
        .route("/create", axum::routing::post(create_todo))
        .fallback(handler_404)
        .with_state(new_app_state);

    match TcpListener::bind("127.0.0.1:8087").await {
        Ok(listener) => {
            println!("Server Listening on : {:?}\n", listener.local_addr());

            match axum::serve(listener, app.into_make_service()).await {
                Ok(_) => {
                    println!("Server started !");
                    Ok(())
                }
                Err(err) => {
                    eprintln!("Server encountered an error: {}", err);
                    Err(err.into())
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to bind TCP listener: {}", err);
            Err(err.into())
        }
    }
}

async fn handler_404() -> impl IntoResponse {
    println!("404");
    (StatusCode::NOT_FOUND, "nothing to see here")
}

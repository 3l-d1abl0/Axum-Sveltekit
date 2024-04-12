use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::Serialize;
use tokio::net::TcpListener;
mod routes;

#[derive(Serialize, Clone)]
struct Hello {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hello = Hello {
        message: String::from("Hi"),
    };

    let all_routes = routes::all_routes();
    let app = Router::new()
        .route("/hello", get(|| async { Json(hello) }))
        .nest("", all_routes)
        .fallback(handler_404);

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

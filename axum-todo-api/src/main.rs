use axum::extract::Json;
use axum::routing::get;
use axum::Router;
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize, Clone)]
struct Hello {
    message: String,
}

#[tokio::main]
async fn main() {
    let hello = Hello {
        message: String::from("Hi"),
    };

    let app = Router::new().route("/hello", get(|| async { Json(hello) }));

    //let listener = TcpListener::bind("127.0.0.1:8087").await.unwrap();

    match TcpListener::bind("127.0.0.1:8087").await {
        Ok(listener) => {
            println!("Server Listening on : {:?}\n", listener.local_addr());

            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        }
        Err(err) => {
            eprintln!("Failed to bind TCP listener: {}", err);
            return;
        }
    }
}

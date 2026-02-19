use axum::{
    routing::get,
    Json,
    Router,
    http::StatusCode,
};
use serde_json::json;
use tower_http::cors::{CorsLayer, Any};
use turso::Builder;

struct Token {
    token: String,
    user: i32
}

struct Theme {
    id: i32,
}

struct Badge {
    id: i32,
    image: String,
    title: String
}

struct User {
    id: i32,
    username: String,
    email: String,
    password: String,
    level: i32,
    badges: Vec<i32>,
    banner_img: String,
    theme: i32,
    reputation: i32,
    pinned_badges: Vec<i32>,
    avatar: String,
    verified: bool
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root_handler))
        .layer(cors);

//    test();

    println!("http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "test": "working",
            "number": 42
        })),
    )
}

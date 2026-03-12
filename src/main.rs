use std::sync::Arc;

use axum::{
    Extension, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use turso::{Builder, Connection, Row};

mod get_user;
mod post_user;
mod get_badge;
use crate::{get_badge::get_badge_id_handler, get_user::get_user_id_handler, post_user::post_user_handler};
/*
GET  /user {user_id} -> User{}
GET  /signup {username, email, password (hash)} -> token
GET  /region ?????
POST /login {email, password (hash)} -> token
POST /support {subject, description, user_id, timestamp}
POST /news {subject, description, img: [str], user_id, timestamp}
???? DELETE /user/:id/:token

token: jwt (id (userid), key (hash pass))
*/

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    id: i32,
    username: String,
    password: String,
}

#[allow(unused)]
struct Theme {
    id: i32,
}

#[derive(Debug, Serialize, Clone)]
struct Badge {
    id: i32,
    image: String,
    title: String,
}

impl Badge {
    fn from_row(row: Row) -> Self {
        Badge {
            id: row.get(0).unwrap(),
            image: row.get(1).unwrap(),
            title: row.get(2).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    handle: String,
    bio: String,
    email: String,
    password: String,
    level: i32,
    badges: Vec<i32>,
    banner_img: String,
    theme: i32,
    reputation: i32,
    pinned_badges: Vec<i32>,
    avatar: String,
    verified: bool,
}
impl User {
    fn from_row(row: Row) -> Self {
        User {
            id: row.get(0).unwrap(),
            username: row.get(1).unwrap(),
            handle: row.get(2).unwrap(),
            bio: row.get(3).unwrap(),
            email: "".to_owned(),    //row.get(4).unwrap(),
            password: "".to_owned(), //row.get(5).unwrap(),
            level: row.get(6).unwrap(),
            badges: serde_json::from_str(&row.get::<String>(7).unwrap()).unwrap(),
            banner_img: row.get(8).unwrap(),
            theme: row.get(9).unwrap(),
            reputation: row.get(10).unwrap(),
            pinned_badges: serde_json::from_str(&row.get::<String>(11).unwrap()).unwrap(),
            avatar: row.get(12).unwrap(),
            verified: row.get(13).unwrap(),
        }
    }
}

struct State {
    connection: Connection,
}

#[tokio::main]
async fn main() {
    // networking
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let db = Builder::new_local("./db.sqlite")
        .build()
        .await
        .expect("issues");
    let connection = db.connect().unwrap();
    let state = Arc::new(State { connection });

    let app = Router::new()
        //.route("/", get(root_handler))
        .route("/badge/{id}", get(get_badge_id_handler))
        .route("/user/{id}", get(get_user_id_handler))
        .route("/register", post(post_user_handler))
        .layer(Extension(state))
        .layer(cors);

    println!("http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct Id {
    id: i32,
}

pub async fn collect_rows<T>(mut rows: turso::Rows, convert: fn(turso::Row) -> T) -> Vec<T> {
    let mut out = Vec::new();

    while let Some(row) = rows.next().await.unwrap() {
        out.push(convert(row));
    }

    out
}
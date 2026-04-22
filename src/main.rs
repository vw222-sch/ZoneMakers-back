use std::sync::Arc;

use axum::{
    Extension, Router,
    routing::{get, patch, post, delete},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use turso::{Builder, Connection, Row};

mod endpoints;
use crate::endpoints::{
    auth::{post_login::post_login_handler, post_user::post_user_handler},
    users::{
        delete_user::delete_user_handler, get_user::get_user_id_handler,
        get_user_handle::get_user_handle_handler, patch_avatar::patch_avatar_handler,
        patch_banner::patch_banner_handler, patch_bio::patch_bio_handler,
        patch_email::patch_email_handler, patch_handle::patch_handle_handler,
        patch_name::patch_name_handler, patch_password::patch_password_handler,
        patch_pinned_badges::patch_pinned_badges_handler,
    },
    badges::{
        delete_badge::delete_badge_handler, get_badge::get_badge_id_handler,
        post_badge::post_badge_handler,
    },
    support::{
        delete_admin_support::delete_admin_support_handler,
        get_admin_support_all::get_admin_support_all_handler, post_support::post_support_handler,
    },
    notifications::{
        delete_notification::delete_notification_handler,
        get_notifications::get_notifications_handler,
        patch_notification_read::patch_notification_read_handler,
        post_notification::post_notification_handler,
    },
    posts::{
        delete_post::delete_post_handler, get_post_replies::get_post_replies_handler,
        get_posts::get_posts_handler, patch_post::patch_post_handler,
        post_post::post_post_handler,
    },
};

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    id: i32,
    username: String,
    exp: u64,
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
    admin: bool,
}
fn token_to_claims(token: &str) -> Option<TokenClaims> {
    let valid = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret("super secret key placeholder".as_ref()),
        &Validation::default(),
    );
    let claims = match valid {
        Ok(data) => data.claims,
        Err(_) => return None,
    };
    return Some(claims);
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
            admin: row.get::<i32>(14).unwrap() == 1,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: String,
    pub author_id: i32,
    pub content: String,
    pub image: String,
    pub replies_count: i32,
    pub created_at: String,
    pub region: i32,
    pub reply_id: String,
}

impl Post {
    pub fn from_row(row: Row) -> Self {
        Post {
            id: row.get(0).unwrap(),
            author_id: row.get(1).unwrap(),
            content: row.get(2).unwrap(),
            image: row.get(3).unwrap(),
            replies_count: row.get(4).unwrap(),
            created_at: row.get(5).unwrap(),
            region: row.get(6).unwrap(),
            reply_id: row.get(7).unwrap(),
        }
    }
}

struct State {
    connection: Connection,
}

#[tokio::main]
async fn main() {
    // networking
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let db = Builder::new_local("./db.sqlite")
        .build()
        .await
        .expect("Could not open database"); // should be impossible, but who knows
    let connection = db.connect().unwrap();
    let state = Arc::new(State { connection });

    let app = Router::new()
        //.route("/", get(root_handler))
        .route("/badge", post(post_badge_handler))
        .route("/badge/{id}", get(get_badge_id_handler).delete(delete_badge_handler))
        .route("/user/{id}", get(get_user_id_handler).delete(delete_user_handler))
        .route("/user/handle/{handle}", get(get_user_handle_handler))
        .route("/register", post(post_user_handler))
        .route("/login", post(post_login_handler))
        .route("/user/handle", patch(patch_handle_handler))
        .route("/user/name", patch(patch_name_handler))
        .route("/user/email", patch(patch_email_handler))
        .route("/user/avatar", patch(patch_avatar_handler))
        .route("/user/banner", patch(patch_banner_handler))
        .route("/user/bio", patch(patch_bio_handler))
        .route("/user/pinned_badges", patch(patch_pinned_badges_handler))
        .route("/user/password", patch(patch_password_handler))
        .route("/support", post(post_support_handler))
        .route("/admin/support/all", get(get_admin_support_all_handler))
        .route("/admin/support/{id}", delete(delete_admin_support_handler))
        .route("/notifications", get(get_notifications_handler).post(post_notification_handler))
        .route("/notifications/read/{id}", patch(patch_notification_read_handler))
        .route("/notifications/{id}", delete(delete_notification_handler))
        .route("/posts/{region}/{page}", get(get_posts_handler))
        .route("/posts/{id}/replies", get(get_post_replies_handler))
        .route("/posts", post(post_post_handler))
        .route("/posts/{id}", patch(patch_post_handler).delete(delete_post_handler))
        // .route("/user", put(put_user_handler))
        // .route("/user/{token}", delete(delete_user_handler))
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

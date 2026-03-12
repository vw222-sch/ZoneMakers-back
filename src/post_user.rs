use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims, User, collect_rows};

#[derive(Deserialize, Debug)]
pub struct RegisterUser {
    email: String,
    username: String,
    password: String,
}
pub async fn post_user_handler(
    state: Extension<Arc<State>>,
    payload: Json<RegisterUser>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let rows = connection
        .query(
            "select * from users where email=? or username=?",
            (payload.email.clone(), payload.username.clone()),
        )
        .await
        .unwrap();
    let v = collect_rows(rows, User::from_row).await;
    if v.len() > 0 {
        return (
            StatusCode::CONFLICT,
            Json(json!({"error": "user already exists"})),
        );
    }

    let mut rows = connection
        .query("select max(id) + 1 as id from users", ())
        .await
        .unwrap();
    let id = rows.next().await.unwrap().unwrap().get::<i32>(0).unwrap();
    let claims = TokenClaims {
        id: id,
        // email: payload.email.clone(),
        username: payload.username.clone(),
        password: payload.password.clone(),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("super secret key placeholder".as_ref()),
    )
    .unwrap();
    // insert into users (id, username, handle, bio, email, password, level, badges, banner_img, theme, reputation, pinned_badges, avatar, verified);
    let _insert = connection
        .execute(
            "insert into users values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            (
                id,
                payload.username.clone(),
                payload.username.clone(), // bio
                "Unset Bio",
                payload.email.clone(),
                payload.password.clone(),
                0,
                "[]",
                "",
                0,
                1,
                "[]",
                "",
                0,
            ),
        )
        .await
        .unwrap();

    (StatusCode::OK, Json(json!(token)))
}

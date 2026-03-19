use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims, User, collect_rows};

#[derive(Deserialize)]
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
        .query("select * from users where email=? or username=?", (payload.email.clone(), payload.username.clone()))
        .await
        .unwrap();
    let v = collect_rows(rows, User::from_row).await;
    if v.len() > 0 {
        return (StatusCode::CONFLICT, Json(json!({"error": "user already exists"})));
    }

    let rows = connection.query("select max(id) + 1 as id from users", ()).await;
    let id: i32;
    match rows {
        Ok(mut rows) => id = rows.next().await.unwrap().unwrap().get::<i32>(0).unwrap(),
        Err(_) => id = 1,
    }
    let claims = TokenClaims {
        id: id,
        username: payload.username.clone(),
        password: payload.password.clone(),
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("super secret key placeholder".as_ref()))
        .unwrap();

    let new_user_params = (
        id,
        payload.username.clone(), // username
        payload.username.clone(), // handle
        "Unset Bio",
        payload.email.clone(),
        payload.password.clone(),
        0,    // level
        "[]", // badges
        "",   // banner_img
        0,    // theme
        1,    // reputation
        "[]", // pinned_badges
        "",   // avatar
        0,    // verified
    );
    connection
        .execute("insert into users values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", new_user_params)
        .await
        .unwrap();
    connection
        .execute("insert into tokens (id, token) values (?, ?)", (id, token.clone()))
        .await
        .unwrap();

    (StatusCode::OK, Json(json!(token)))
}

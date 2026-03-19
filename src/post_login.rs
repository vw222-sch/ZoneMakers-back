use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims, User, collect_rows};

#[derive(Deserialize)]
pub struct LoginUser {
    username: String,
    password: String,
}

pub async fn post_login_handler(
    state: Extension<Arc<State>>,
    payload: Json<LoginUser>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;
    let rows = connection
        .query("select * from users where username=? and password=?", (payload.username.clone(), payload.password.clone()))
        .await;
    let user: User;
    match rows {
        Ok(mut rows) => user = User::from_row(rows.next().await.unwrap().unwrap()),
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Incorrect credentials"}))),
    }
    // let v = collect_rows(rows, User::from_row).await;

    (StatusCode::OK, Json(json!(user)))
}

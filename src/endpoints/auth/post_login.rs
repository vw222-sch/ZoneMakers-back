use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::json;

use crate::{State, token_to_claims};

#[derive(Deserialize)]
pub struct LoginUser {
    handle: String,
    password: String,
}

pub async fn post_login_handler(
    state: Extension<Arc<State>>,
    payload: Json<LoginUser>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;
    let value = connection
        .query("select token, user from tokens where tokens.user = (select id from users where handle=? and password=? limit 1);", (payload.handle.clone(), payload.password.clone()))
        .await
        .unwrap()
        .next()
        .await
        .unwrap();
    let token: String;
    match value {
        Some(token_ok) => token = token_ok.get(0).unwrap(),
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Incorrect credentials"}))),
    }
    let claims = token_to_claims(&token);

    (StatusCode::OK, Json(json!({"token": token, "id": claims.unwrap().id})))
}

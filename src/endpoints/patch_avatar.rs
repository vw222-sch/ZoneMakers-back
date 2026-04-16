use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims, token_to_claims};

#[derive(Deserialize)]
pub struct PatchAvatar {
    token: String,
    new_avatar: String,
}

pub async fn patch_avatar_handler(
    state: Extension<Arc<State>>,
    payload: Json<PatchAvatar>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let opt = token_to_claims(&payload.token);
    let claims: TokenClaims;
    match opt {
        Some(data) => claims = data,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Incorrect credentials"}))),
    };

    let params = (payload.new_avatar.clone(), claims.id);
    connection
        .execute("update users set avatar = ? where id = ?", params)
        .await
        .unwrap();

    (StatusCode::OK, Json(json!("OK")))
}

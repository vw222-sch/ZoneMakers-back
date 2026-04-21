// Assuming it is as simple as this.
use std::sync::Arc;

use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims, token_to_claims};

#[derive(Deserialize)]
pub struct PatchPassword {
    new_password: String,
}

pub async fn patch_password_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    payload: Json<PatchPassword>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let token = match headers.get("Token").and_then(|h| h.to_str().ok()) {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Missing token"}))),
    };

    let opt = token_to_claims(token);
    let claims: TokenClaims;
    match opt {
        Some(data) => claims = data,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid token"}))),
    };

    let params = (payload.new_password.clone(), claims.id);
    connection
        .execute("update users set password = ? where id = ?", params)
        .await
        .unwrap();

    (StatusCode::OK, Json(json!("OK")))
}

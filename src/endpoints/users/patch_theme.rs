use std::sync::Arc;

use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims, token_to_claims};

#[derive(Deserialize)]
pub struct PatchTheme {
    new_theme: String,
}

pub async fn patch_theme_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    payload: Json<PatchTheme>,
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

    let params = (payload.new_theme.clone(), claims.id);
    connection
        .execute("update users set theme = ? where id = ?", params)
        .await
        .unwrap();

    (StatusCode::OK, Json(json!("OK")))
}

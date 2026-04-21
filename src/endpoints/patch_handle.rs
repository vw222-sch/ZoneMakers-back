use std::sync::Arc;

use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde::Deserialize;
use serde_json::json;

use crate::{State, token_to_claims};

#[derive(Deserialize)]
pub struct PatchHandle {
    new_handle: String,
}

pub async fn patch_handle_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Json(payload): Json<PatchHandle>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let token = match headers.get("Token").and_then(|h| h.to_str().ok()) {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Missing token"}))),
    };

    let claims = match token_to_claims(token) {
        Some(data) => data,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid token"}))),
    };

    let handle_check = connection
        .query("select id from users where handle = ? and id != ?", (payload.new_handle.clone(), claims.id))
        .await
        .unwrap()
        .next()
        .await
        .unwrap();

    if handle_check.is_some() {
        return (StatusCode::CONFLICT, Json(json!({"error": "Handle already taken"})));
    }

    let params = (payload.new_handle.clone(), claims.id);
    let result = connection
        .execute("update users set handle = ? where id = ?", params)
        .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(json!("OK"))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

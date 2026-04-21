use std::sync::Arc;

use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde_json::json;

use crate::{State, TokenClaims, token_to_claims};

pub async fn delete_user_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
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

    connection
        .execute("delete from tokens where user=?", (claims.id,))
        .await
        .unwrap();
    connection
        .execute("delete from users where id=?", (claims.id,))
        .await
        .unwrap();
    (StatusCode::OK, Json(json!("Deleted")))
}

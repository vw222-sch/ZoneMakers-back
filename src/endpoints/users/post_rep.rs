use std::sync::Arc;

use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde::Deserialize;
use serde_json::json;

use crate::{State, token_to_claims};

#[derive(Deserialize)]
pub struct RepPayload {
    pub user_id: i32,
    pub rep: i32,
}

pub async fn post_admin_rep_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Json(payload): Json<RepPayload>,
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

    // ensure requester is admin
    let admin_row_result = connection
        .query("select admin from users where id=?", (claims.id,))
        .await;

    let admin_row = match admin_row_result {
        Ok(mut rows) => rows.next().await.unwrap(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    };
    
    let is_admin = match admin_row {
        Some(r) => r.get::<i32>(0).unwrap() == 1,
        None => false,
    };

    if !is_admin {
        return (StatusCode::FORBIDDEN, Json(json!({"error": "Admin only"})));
    }

    // Update user's reputation
    let update_result = connection
        .execute(
            "update users set reputation = reputation + ? where id = ?",
            (payload.rep, payload.user_id),
        )
        .await;

    match update_result {
        Ok(_) => (StatusCode::OK, Json(json!("OK"))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

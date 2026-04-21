use std::sync::Arc;

use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde::Deserialize;
use serde_json::json;

use crate::{State, token_to_claims};

#[derive(Deserialize)]
pub struct PostBadgePayload {
    image: String,
    title: String,
}

pub async fn post_badge_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Json(payload): Json<PostBadgePayload>,
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

    let admin_row = connection
        .query("select admin from users where id=?", (claims.id,))
        .await
        .unwrap()
        .next()
        .await
        .unwrap();
    
    let is_admin = match admin_row {
        Some(r) => r.get::<i32>(0).unwrap() == 1,
        None => false,
    };

    if !is_admin {
        return (StatusCode::FORBIDDEN, Json(json!({"error": "Admin only"})));
    }

    let result = connection
        .execute("insert into badges (image, title) values (?, ?)", (payload.image, payload.title))
        .await;

    match result {
        Ok(_) => (StatusCode::CREATED, Json(json!({"message": "Badge created"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

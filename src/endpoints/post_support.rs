use std::sync::Arc;

use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims, token_to_claims};

#[derive(Deserialize)]
pub struct PostSupport {
    subject: String,
    topic: i32,
    description: String,
}

pub async fn post_support_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    payload: Json<PostSupport>,
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

    let params = (
        payload.subject.clone(),
        payload.topic,
        payload.description.clone(),
        claims.id,
    );

    let res = connection
        .execute(
            "insert into support (subject, topic, description, userid) values (?, ?, ?, ?)",
            params,
        )
        .await;

    match res {
        Ok(_) => (StatusCode::OK, Json(json!("OK"))),
        Err(_) => (StatusCode::BAD_REQUEST, Json(json!({"error": "Could not create support request"}))),
    }
}

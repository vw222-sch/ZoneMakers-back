use crate::{State, token_to_claims};
use axum::{
    Extension, Json,
    extract::Path,
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct PatchPost {
    pub content: String,
}

pub async fn patch_post_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(payload): Json<PatchPost>,
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

    // only the user can change its own posts
    let post_row = connection
        .query("select author_id from posts where id=?", (id.clone(),))
        .await
        .unwrap()
        .next()
        .await
        .unwrap();

    let author_id = match post_row {
        Some(r) => r.get::<i32>(0).unwrap(),
        None => return (StatusCode::NOT_FOUND, Json(json!({"error": "Post not found"}))),
    };

    if author_id != claims.id {
        return (StatusCode::FORBIDDEN, Json(json!({"error": "Not the author"})));
    }

    let result = connection
        .execute("update posts set content=? where id=?", (payload.content, id))
        .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Post updated"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

use std::sync::Arc;
use axum::{Extension, Json, http::{StatusCode, HeaderMap}, extract::Path};
use serde_json::json;
use crate::{State, token_to_claims};

pub async fn delete_travel_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Path(id): Path<i32>,
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

    let travel_row = connection
        .query("select user_id from travel where id=?", (id,))
        .await
        .unwrap()
        .next()
        .await
        .unwrap();

    let author_id = match travel_row {
        Some(r) => r.get::<i32>(0).unwrap(),
        None => return (StatusCode::NOT_FOUND, Json(json!({"error": "Travel log not found"}))),
    };

    if author_id != claims.id {
        return (StatusCode::FORBIDDEN, Json(json!({"error": "Unauthorized"})));
    }

    let result = connection.execute("delete from travel where id=?", (id,)).await;

    match result {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Travel log deleted"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

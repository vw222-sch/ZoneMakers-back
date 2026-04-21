use std::sync::Arc;
use axum::{Extension, Json, extract::Path, http::{StatusCode, HeaderMap}};
use serde_json::json;
use crate::{State, token_to_claims, Id};

pub async fn patch_notification_read_handler(
    payload: Path<Id>,
    state: Extension<Arc<State>>,
    headers: HeaderMap,
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

    let result = connection
        .execute(
            "update notifications set is_read = 1 where id = ? and user_id = ?",
            (payload.id, claims.id),
        )
        .await;

    match result {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                (StatusCode::NOT_FOUND, Json(json!({"error": "Notification not found or access denied"})))
            } else {
                (StatusCode::OK, Json(json!({"message": "Notification marked as read"})))
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

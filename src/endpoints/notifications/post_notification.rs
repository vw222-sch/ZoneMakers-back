use std::sync::Arc;
use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde::Deserialize;
use serde_json::json;
use crate::{State, token_to_claims};

#[derive(Deserialize)]
pub struct PostNotification {
    title: String,
    message: String,
    #[serde(rename = "type")]
    notification_type: String,
    user_id: i32,
}

pub async fn post_notification_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Json(payload): Json<PostNotification>,
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

    // Check if user is admin
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

    // Get current time in ISO format
    let now = std::time::SystemTime::now();
    let datetime: chrono::DateTime<chrono::Utc> = now.into();
    let timestamp = datetime.to_rfc3339();

    let result = connection
        .execute(
            "insert into notifications (title, message, type, timestamp, is_read, user_id) values (?, ?, ?, ?, 0, ?)",
            (payload.title, payload.message, payload.notification_type, timestamp, payload.user_id),
        )
        .await;

    match result {
        Ok(_) => (StatusCode::CREATED, Json(json!({"message": "Notification created"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

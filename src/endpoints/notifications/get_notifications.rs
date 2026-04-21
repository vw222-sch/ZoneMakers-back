use std::sync::Arc;
use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde::Serialize;
use serde_json::json;
use turso::Row;
use crate::{State, token_to_claims, collect_rows};

#[derive(Serialize)]
pub struct Notification {
    id: i32,
    title: String,
    message: String,
    #[serde(rename = "type")]
    notification_type: String,
    timestamp: String,
    is_read: bool,
    user_id: i32,
}

impl Notification {
    fn from_row(row: Row) -> Self {
        Notification {
            id: row.get(0).unwrap(),
            title: row.get(1).unwrap(),
            message: row.get(2).unwrap(),
            notification_type: row.get(3).unwrap(),
            timestamp: row.get(4).unwrap(),
            is_read: row.get::<i32>(5).unwrap() == 1,
            user_id: row.get(6).unwrap(),
        }
    }
}

pub async fn get_notifications_handler(
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

    let rows = connection
        .query("select * from notifications where user_id = ?", (claims.id,))
        .await
        .unwrap();

    let notifications = collect_rows(rows, Notification::from_row).await;

    (StatusCode::OK, Json(json!(notifications)))
}

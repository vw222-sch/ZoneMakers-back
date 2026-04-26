use std::sync::Arc;
use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde::Deserialize;
use serde_json::json;
use crate::{State, token_to_claims};

#[derive(Deserialize)]
pub struct CreateTravel {
    pub title: String,
    pub message: String,
    #[serde(rename = "type")]
    pub travel_type: String,
}

pub async fn post_travel_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Json(payload): Json<CreateTravel>,
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

    let now = std::time::SystemTime::now();
    let datetime: chrono::DateTime<chrono::Utc> = now.into();
    let created_at = datetime.to_rfc3339();

    let insert_result = connection
        .execute(
            "insert into travel (title, message, type, timestamp, user_id) values (?, ?, ?, ?, ?)",
            (
                payload.title,
                payload.message,
                payload.travel_type,
                created_at,
                claims.id,
            ),
        )
        .await;

    match insert_result {
        Ok(_) => (StatusCode::CREATED, Json(json!({"message": "Travel log created"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

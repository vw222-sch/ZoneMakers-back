use std::sync::Arc;
use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde::Deserialize;
use serde_json::json;
use crate::{State, token_to_claims};

#[derive(Deserialize)]
pub struct CreateReport {
    pub reason: String,
    pub report_id: String,
}

async fn post_report(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    payload: CreateReport,
    report_type: i32,
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

    let insert_result = connection
        .execute(
            "insert into reports (reason, type, user_id, report_id) values (?, ?, ?, ?)",
            (
                payload.reason,
                report_type,
                claims.id,
                payload.report_id,
            ),
        )
        .await;

    match insert_result {
        Ok(_) => (StatusCode::CREATED, Json(json!({"message": "Report created"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    }
}

pub async fn post_report_zone_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Json(payload): Json<CreateReport>,
) -> (StatusCode, Json<serde_json::Value>) {
    post_report(state, headers, payload, 1).await
}

pub async fn post_report_user_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Json(payload): Json<CreateReport>,
) -> (StatusCode, Json<serde_json::Value>) {
    post_report(state, headers, payload, 2).await
}

pub async fn post_report_post_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Json(payload): Json<CreateReport>,
) -> (StatusCode, Json<serde_json::Value>) {
    post_report(state, headers, payload, 3).await
}

use std::sync::Arc;
use axum::{Extension, Json, http::{StatusCode, HeaderMap}, extract::Path};
use serde_json::json;
use crate::{State, token_to_claims};

async fn delete_report(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    report_id: i32,
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

    let delete_result = connection
        .execute("delete from reports where id = ?", (report_id,))
        .await;

    match delete_result {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Report deleted"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    }
}

pub async fn delete_report_zone_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> (StatusCode, Json<serde_json::Value>) {
    delete_report(state, headers, id).await
}

pub async fn delete_report_user_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> (StatusCode, Json<serde_json::Value>) {
    delete_report(state, headers, id).await
}

pub async fn delete_report_post_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> (StatusCode, Json<serde_json::Value>) {
    delete_report(state, headers, id).await
}

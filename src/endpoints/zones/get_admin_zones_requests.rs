use std::sync::Arc;
use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde_json::json;
use crate::{State, Zone, token_to_claims, collect_rows};

pub async fn get_admin_zones_requests_handler(
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

    // ensure user is admin
    let admin_row_result = connection
        .query("select admin from users where id=?", (claims.id,))
        .await;

    let admin_row = match admin_row_result {
        Ok(mut rows) => rows.next().await.unwrap(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    };
    
    let is_admin = match admin_row {
        Some(r) => r.get::<i32>(0).unwrap() == 1,
        None => false,
    };

    if !is_admin {
        return (StatusCode::FORBIDDEN, Json(json!({"error": "Admin only"})));
    }

    let rows_result = connection
        .query("select * from zones where is_request = 1", ())
        .await;

    match rows_result {
        Ok(rows) => {
            let zones = collect_rows(rows, Zone::from_row).await;
            (StatusCode::OK, Json(json!(zones)))
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    }
}

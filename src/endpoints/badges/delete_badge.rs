use std::sync::Arc;

use axum::{Extension, Json, extract::Path, http::{StatusCode, HeaderMap}};
use serde_json::json;

use crate::{State, token_to_claims, Id};

pub async fn delete_badge_handler(
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

    let result = connection
        .execute("delete from badges where id=?", (payload.id,))
        .await;

    match result {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                (StatusCode::NOT_FOUND, Json(json!({"error": "Badge not found"})))
            } else {
                (StatusCode::OK, Json(json!({"message": "Badge deleted"})))
            }
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

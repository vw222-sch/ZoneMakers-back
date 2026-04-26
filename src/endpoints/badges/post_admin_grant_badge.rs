use std::sync::Arc;
use axum::{Extension, Json, http::{StatusCode, HeaderMap}, extract::Path};
use serde::Deserialize;
use serde_json::json;
use crate::{State, token_to_claims};

#[derive(Deserialize)]
pub struct GrantBadgePath {
    id: i32,
    badge_id: i32,
}

pub async fn post_admin_grant_badge_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Path(params): Path<GrantBadgePath>,
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

    // Fetch user's badges
    let user_row_result = connection
        .query("select badges from users where id=?", (params.id,))
        .await;

    let user_row = match user_row_result {
        Ok(mut rows) => rows.next().await.unwrap(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    };

    let badges_json = match user_row {
        Some(r) => r.get::<String>(0).unwrap(),
        None => return (StatusCode::NOT_FOUND, Json(json!({"error": "User not found"}))),
    };

    let mut badges: Vec<i32> = serde_json::from_str(&badges_json).unwrap_or_default();

    if !badges.contains(&params.badge_id) {
        badges.push(params.badge_id);
        let updated_badges = serde_json::to_string(&badges).unwrap();
        
        let update_result = connection
            .execute("update users set badges = ? where id = ?", (updated_badges, params.id))
            .await;

        if let Err(e) = update_result {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})));
        }
    }

    (StatusCode::OK, Json(json!("OK")))
}

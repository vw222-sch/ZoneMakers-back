use std::sync::Arc;

use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims, token_to_claims};

#[derive(Deserialize)]
pub struct PatchRegion {
    new_region: i32,
}

pub async fn patch_region_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    payload: Json<PatchRegion>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let token = match headers.get("Token").and_then(|h| h.to_str().ok()) {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Missing token"}))),
    };

    let opt = token_to_claims(token);
    let claims: TokenClaims;
    match opt {
        Some(data) => claims = data,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid token"}))),
    };

    // Fetch current region and badges
    let row_result = connection
        .query("select region, badges from users where id = ?", (claims.id,))
        .await;

    let row = match row_result {
        Ok(mut rows) => rows.next().await.unwrap(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    };

    if let Some(r) = row {
        let old_region: i32 = r.get(0).unwrap();
        let badges_str: String = r.get(1).unwrap();
        let mut badges: Vec<i32> = serde_json::from_str(&badges_str).unwrap_or_default();

        // swap badges
        badges.retain(|&x| x != old_region);
        if !badges.contains(&payload.new_region) {
            badges.push(payload.new_region);
        }

        let new_badges_str = serde_json::to_string(&badges).unwrap();

        let params = (payload.new_region, new_badges_str, claims.id);
        let update_result = connection
            .execute("update users set region = ?, badges = ? where id = ?", params)
            .await;

        match update_result {
            Ok(_) => (StatusCode::OK, Json(json!("OK"))),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
        }
    } else {
        (StatusCode::NOT_FOUND, Json(json!({"error": "User not found"})))
    }
}

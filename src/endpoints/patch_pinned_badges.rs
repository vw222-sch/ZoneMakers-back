use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims, token_to_claims};

#[derive(Deserialize)]
pub struct PatchPinnedBadges {
    token: String,
    new_pinned_badges: Vec<i32>,
}

pub async fn patch_pinned_badges_handler(
    state: Extension<Arc<State>>,
    payload: Json<PatchPinnedBadges>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let opt = token_to_claims(&payload.token);
    let claims: TokenClaims;
    match opt {
        Some(data) => claims = data,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Incorrect credentials"}))),
    };

    let params = (serde_json::to_string(&payload.new_pinned_badges).unwrap(), claims.id);
    connection
        .execute("update users set pinned_badges = ? where id = ?", params)
        .await
        .unwrap();

    (StatusCode::OK, Json(json!("OK")))
}

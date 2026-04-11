use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims};

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

    let valid = decode::<TokenClaims>(
        payload.token.clone(),
        &DecodingKey::from_secret("super secret key placeholder".as_ref()),
        &Validation::default(),
    );
    let claims = match valid {
        Ok(data) => data.claims,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Incorrect credentials"}))),
    };

    let params = (serde_json::to_string(&payload.new_pinned_badges).unwrap(), claims.id);
    connection
        .execute("update users set pinned_badges = ? where id = ?", params)
        .await
        .unwrap();

    (StatusCode::OK, Json(json!("OK")))
}

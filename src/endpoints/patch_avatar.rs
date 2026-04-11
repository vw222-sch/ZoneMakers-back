use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims};

#[derive(Deserialize)]
pub struct PatchAvatar {
    token: String,
    new_avatar: String,
}

pub async fn patch_avatar_handler(
    state: Extension<Arc<State>>,
    payload: Json<PatchAvatar>,
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

    let params = (payload.new_avatar.clone(), claims.id);
    connection
        .execute("update users set avatar = ? where id = ?", params)
        .await
        .unwrap();

    (StatusCode::OK, Json(json!("OK")))
}

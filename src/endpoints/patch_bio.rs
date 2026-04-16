use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims, token_to_claims};

#[derive(Deserialize)]
pub struct PatchBio {
    token: String,
    new_bio: String,
}

pub async fn patch_bio_handler(
    state: Extension<Arc<State>>,
    payload: Json<PatchBio>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let opt = token_to_claims(&payload.token);
    let claims: TokenClaims;
    match opt {
        Some(data) => claims = data,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Incorrect credentials"}))),
    };

    let params = (payload.new_bio.clone(), claims.id);
    connection
        .execute("update users set bio = ? where id = ?", params)
        .await
        .unwrap();

    (StatusCode::OK, Json(json!("OK")))
}

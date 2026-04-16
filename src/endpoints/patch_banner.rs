use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims, token_to_claims};

#[derive(Deserialize)]
pub struct PatchBanner {
    token: String,
    new_banner: String,
}

pub async fn patch_banner_handler(
    state: Extension<Arc<State>>,
    payload: Json<PatchBanner>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let opt = token_to_claims(&payload.token);
    let claims: TokenClaims;
    match opt {
        Some(data) => claims = data,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Incorrect credentials"}))),
    };

    let params = (payload.new_banner.clone(), claims.id);
    connection
        .execute("update users set banner_img = ? where id = ?", params)
        .await
        .unwrap();

    (StatusCode::OK, Json(json!("OK")))
}

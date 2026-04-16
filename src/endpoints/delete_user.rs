use std::sync::Arc;

use axum::{Extension, Json, extract::Path, http::StatusCode};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims, token_to_claims};

#[derive(Deserialize)]
pub struct Token {
    id: String, // technically token, figure a way out to not conflict with GET /user
}

pub async fn delete_user_handler(
    state: Extension<Arc<State>>,
    payload: Path<Token>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let opt = token_to_claims(&payload.id);
    let claims: TokenClaims;
    match opt {
        Some(data) => claims = data,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Incorrect credentials"}))),
    };

    connection
        .execute("delete from tokens where user=?", (claims.id,))
        .await
        .unwrap();
    connection
        .execute("delete from users where id=?", (claims.id,))
        .await
        .unwrap();
    (StatusCode::OK, Json(json!("Deleted")))
}

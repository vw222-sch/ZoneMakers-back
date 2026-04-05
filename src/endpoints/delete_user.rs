use std::sync::Arc;

use axum::{Extension, Json, extract::Path, http::StatusCode};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims};

#[derive(Deserialize)]
pub struct Token {
    id: String, // technically token, figure a way out to not conflict with GET /user
}

pub async fn delete_user_handler(
    state: Extension<Arc<State>>,
    payload: Path<Token>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let valid = decode::<TokenClaims>(
        payload.id.clone(),
        &DecodingKey::from_secret("super secret key placeholder".as_ref()),
        &Validation::default(),
    );
    let id = match valid {
        Ok(data) => data.claims.id,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Incorrect credentials"}))),
    };

    connection
        .execute("delete from tokens where user=?", (id,))
        .await
        .unwrap();
    connection.execute("delete from users where id=?", (id,)).await.unwrap();
    (StatusCode::OK, Json(json!("Deleted")))
}

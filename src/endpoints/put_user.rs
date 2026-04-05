use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::Deserialize;
use serde_json::json;

use crate::{State, TokenClaims};

#[derive(Deserialize)]
pub struct PutUser {
    token: String,
    email: String,
    password: String,
    username: String,
    bio: String,
    pinned_badges: String,
}
/*

doesnt work for some reason, but the command does??

*/
pub async fn put_user_handler(
    state: Extension<Arc<State>>,
    payload: Json<PutUser>,
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

    connection
        .execute(
            "update users set email=?, password=?, username=?, bio=?, pinned_badges=? where id=?",
            (
                payload.email.clone(),
                payload.password.clone(),
                payload.username.clone(),
                payload.bio.clone(),
                payload.pinned_badges.clone(),
                claims.id,
            ),
        )
        .await
        .unwrap();

    // println!(
    //     "update users set email='{}', password='{}', username='{}', bio='{}', pinned_badges='{}' where id={}",
    //     payload.email.clone(),
    //     payload.password.clone(),
    //     payload.username.clone(),
    //     payload.bio.clone(),
    //     payload.pinned_badges.clone(),
    //     claims.id,
    // );
    (StatusCode::OK, Json(json!("User details changed")))
}

use std::sync::Arc;
use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde::Deserialize;
use serde_json::json;
use crate::{State, token_to_claims};

#[derive(Deserialize)]
pub struct CreateZoneRequest {
    pub name: String,
    pub coordinates: String,
    pub hazard_level: String,
    pub description: String,
    pub hazards: String,
    pub images: String,
}

pub async fn post_zone_request_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Json(payload): Json<CreateZoneRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let token = match headers.get("Token").and_then(|h| h.to_str().ok()) {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Missing token"}))),
    };

    if token_to_claims(token).is_none() {
        return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid token"})));
    }

    let insert_result = connection
        .execute(
            "insert into zones (name, coordinates, hazard_level, description, hazards, images, is_request) values (?, ?, ?, ?, ?, ?, 1)",
            (
                payload.name,
                payload.coordinates,
                payload.hazard_level,
                payload.description,
                payload.hazards,
                payload.images,
            ),
        )
        .await;

    match insert_result {
        Ok(_) => (StatusCode::CREATED, Json(json!({"message": "Zone request created"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    }
}

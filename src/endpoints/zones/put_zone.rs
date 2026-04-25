use std::sync::Arc;
use axum::{Extension, Json, http::{StatusCode, HeaderMap}, extract::Path};
use serde::Deserialize;
use serde_json::json;
use crate::{State, token_to_claims, Zone};

#[derive(Deserialize)]
pub struct PutZoneRequest {
    pub name: Option<String>,
    pub coordinates: Option<String>,
    pub hazard_level: Option<String>,
    pub description: Option<String>,
    pub hazards: Option<String>,
    pub images: Option<String>,
}

pub async fn put_zone_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Path(id): Path<i32>,
    Json(payload): Json<PutZoneRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let token = match headers.get("Token").and_then(|h| h.to_str().ok()) {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Missing token"}))),
    };

    let claims = match token_to_claims(token) {
        Some(data) => data,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid token"}))),
    };

    // fetch zone to check author
    let zone_row_result = connection
        .query("select * from zones where id = ?", (id,))
        .await;

    let zone = match zone_row_result {
        Ok(mut rows) => {
            if let Some(row) = rows.next().await.unwrap() {
                Zone::from_row(row)
            } else {
                return (StatusCode::NOT_FOUND, Json(json!({"error": "Zone not found"})));
            }
        }
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    };

    // check if admin
    let admin_row_result = connection
        .query("select admin from users where id=?", (claims.id,))
        .await;

    let is_admin = match admin_row_result {
        Ok(mut rows) => {
            if let Some(row) = rows.next().await.unwrap() {
                row.get::<i32>(0).unwrap() == 1
            } else {
                false
            }
        }
        Err(_) => false,
    };

    if !is_admin && zone.author != claims.id {
        return (StatusCode::FORBIDDEN, Json(json!({"error": "Only admins and the author can update this zone"})));
    }

    // update all fields for simplicity
    let update_result = connection
        .execute(
            "update zones set name = ?, coordinates = ?, hazard_level = ?, description = ?, hazards = ?, images = ?, is_request = 1 where id = ?",
            (
                payload.name.unwrap_or(zone.name),
                payload.coordinates.unwrap_or(zone.coordinates),
                payload.hazard_level.unwrap_or(zone.hazard_level),
                payload.description.unwrap_or(zone.description),
                payload.hazards.unwrap_or(zone.hazards),
                payload.images.unwrap_or(zone.images),
                id,
            ),
        )
        .await;

    match update_result {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Zone updated and set to request status"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    }
}

use std::sync::Arc;
use axum::{Extension, Json, http::StatusCode};
use crate::{State, collect_rows};
use serde::Serialize;
use turso::Row;

#[derive(Serialize)]
pub struct ZoneSummary {
    pub id: i32,
    pub hazard_level: String,
    pub coordinates: String,
}

impl ZoneSummary {
    pub fn from_row(row: Row) -> Self {
        ZoneSummary {
            id: row.get(0).unwrap(),
            hazard_level: row.get(1).unwrap(),
            coordinates: row.get(2).unwrap(),
        }
    }
}

pub async fn get_zones_handler(
    state: Extension<Arc<State>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let rows_result = connection
        .query(
            "select id, hazard_level, coordinates from zones where is_request = 0",
            (),
        )
        .await;

    match rows_result {
        Ok(rows) => {
            let zones = collect_rows(rows, ZoneSummary::from_row).await;
            (StatusCode::OK, Json(serde_json::json!(zones)))
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()})))
    }
}

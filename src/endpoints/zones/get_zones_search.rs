use std::sync::Arc;
use axum::{Extension, Json, http::StatusCode, extract::Path};
use crate::{State, collect_rows};
use super::get_zones::ZoneSummary;

pub async fn get_zones_search_handler(
    state: Extension<Arc<State>>,
    Path(query): Path<String>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;
    let query_param = format!("%{}%", query);

    let rows_result = connection
        .query(
            "select id, hazard_level, coordinates from zones where is_request = 0 and (name like ? or description like ?)",
            (query_param.clone(), query_param),
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

use std::sync::Arc;
use axum::{Extension, Json, http::StatusCode, extract::Path};
use crate::{State, Travel, collect_rows};

pub async fn get_travel_handler(
    state: Extension<Arc<State>>,
    Path(page): Path<i32>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;
    
    let limit = 5;
    let offset = (page - 1) * 5;

    let rows_result = connection
        .query(
            "select * from travel order by timestamp desc limit ? offset ?",
            (limit, offset),
        )
        .await;

    match rows_result {
        Ok(rows) => {
            let logs = collect_rows(rows, Travel::from_row).await;
            (StatusCode::OK, Json(serde_json::json!(logs)))
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()})))
    }
}

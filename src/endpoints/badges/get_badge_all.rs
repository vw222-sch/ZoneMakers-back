use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode};
use serde_json::json;

use crate::{State, Badge, collect_rows};

pub async fn get_badge_all_handler(
    state: Extension<Arc<State>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let rows = connection
        .query("select * from badges", ())
        .await
        .unwrap();
    let v = collect_rows(rows, Badge::from_row).await;

    (StatusCode::OK, Json(json!(v)))
}

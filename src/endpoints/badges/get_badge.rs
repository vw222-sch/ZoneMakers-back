use std::sync::Arc;

use axum::{Extension, Json, extract::Path, http::StatusCode};
use serde_json::json;

use crate::{Id, State, Badge, collect_rows};
pub async fn get_badge_id_handler(
    payload: Path<Id>,
    state: Extension<Arc<State>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let rows = connection
        .query("select * from badges where id=?", (payload.id,))
        .await
        .unwrap();
    let v = collect_rows(rows, Badge::from_row).await;
    match v.len() {
        0 => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "id not found"})),
        ),
        1 => (StatusCode::OK, Json(json!(v[0]))),
        _ => panic!("impossible"),
    }
}

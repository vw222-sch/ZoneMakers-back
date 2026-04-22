use std::sync::Arc;
use axum::{Extension, Json, http::StatusCode, extract::Path};
use crate::{State, Zone};

pub async fn get_zone_id_handler(
    state: Extension<Arc<State>>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let rows_result = connection
        .query(
            "select * from zones where id = ?",
            (id,),
        )
        .await;

    match rows_result {
        Ok(mut rows) => {
            if let Some(row) = rows.next().await.unwrap() {
                let zone = Zone::from_row(row);
                (StatusCode::OK, Json(serde_json::json!(zone)))
            } else {
                (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Zone not found"})))
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()})))
    }
}

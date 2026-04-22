use std::sync::Arc;
use axum::{Extension, Json, http::StatusCode, extract::Path};
use crate::{State, Post, collect_rows};

pub async fn get_post_replies_handler(
    state: Extension<Arc<State>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let rows_result = connection
        .query(
            "select * from posts where reply_id = ? order by created_at asc",
            (id,),
        )
        .await;

    match rows_result {
        Ok(rows) => {
            let posts = collect_rows(rows, Post::from_row).await;
            (StatusCode::OK, Json(serde_json::json!(posts)))
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()})))
    }
}

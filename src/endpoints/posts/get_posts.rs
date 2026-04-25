use std::sync::Arc;
use axum::{Extension, Json, http::StatusCode, extract::Path};
use crate::{State, Post, collect_rows};

pub async fn get_posts_handler(
    state: Extension<Arc<State>>,
    Path((region, page)): Path<(i32, i32)>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;
    
    let limit = if page == 1 { 50 } else { 25 };
    let offset = if page == 1 { 0 } else { 50 + (page - 2) * 25 };

    let rows_result = connection
        .query(
            "select p.*, u.username, u.handle, u.avatar, u.verified from posts p join users u on p.author_id = u.id where p.reply_id = '0' and p.region = ? order by p.created_at desc limit ? offset ?",
            (region, limit, offset),
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

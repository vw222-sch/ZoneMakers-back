use std::sync::Arc;
use axum::{Extension, Json, http::{StatusCode, HeaderMap}, extract::Path};
use serde_json::json;
use crate::{State, token_to_claims};

pub async fn delete_post_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Path(id): Path<String>,
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

    // post and user admin status
    let post_row = connection
        .query("select author_id, reply_id from posts where id=?", (id.clone(),))
        .await
        .unwrap()
        .next()
        .await
        .unwrap();

    let (author_id, parent_id) = match post_row {
        Some(r) => (r.get::<i32>(0).unwrap(), r.get::<String>(1).unwrap()),
        None => return (StatusCode::NOT_FOUND, Json(json!({"error": "Post not found"}))),
    };

    let user_row = connection
        .query("select admin from users where id=?", (claims.id,))
        .await
        .unwrap()
        .next()
        .await
        .unwrap();
    
    let is_admin = match user_row {
        Some(r) => r.get::<i32>(0).unwrap() == 1,
        None => false,
    };

    if !is_admin && author_id != claims.id {
        return (StatusCode::FORBIDDEN, Json(json!({"error": "Unauthorized"})));
    }

    // Delete replies then the post
    let _ = connection.execute("delete from posts where reply_id=?", (id.clone(),)).await;
    let result = connection.execute("delete from posts where id=?", (id.clone(),)).await;

    if let Ok(_) = result {
        // decrement the parent's replies_count
        if parent_id != "0" {
            let _ = connection.execute("update posts set replies_count = replies_count - 1 where id=?", (parent_id,)).await;
        }
        (StatusCode::OK, Json(json!({"message": "Post deleted"})))
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to delete post"})))
    }
}

use std::sync::Arc;
use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde::Deserialize;
use serde_json::json;
use crate::{State, token_to_claims};

#[derive(Deserialize)]
pub struct CreatePost {
    pub content: String,
    pub image: Option<String>,
    pub region: i32,
    pub reply_id: Option<String>,
}

pub async fn post_post_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Json(payload): Json<CreatePost>,
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

    let post_id = uuid::Uuid::new_v4().to_string();
    let reply_id = payload.reply_id.unwrap_or("0".to_string());
    
    let now = std::time::SystemTime::now();
    let datetime: chrono::DateTime<chrono::Utc> = now.into();
    let created_at = datetime.to_rfc3339();

    let insert_result = connection
        .execute(
            "insert into posts (id, author_id, content, image, replies_count, created_at, region, reply_id) values (?, ?, ?, ?, 0, ?, ?, ?)",
            (
                post_id.clone(),
                claims.id,
                payload.content,
                payload.image.unwrap_or_default(),
                created_at,
                payload.region,
                reply_id.clone(),
            ),
        )
        .await;

    if let Err(e) = insert_result {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})));
    }

    // "every time a new post contains the post's id in the reply_id field, the replies_count is increased by 1"
    if reply_id != "0" {
        let update_result = connection
            .execute(
                "update posts set replies_count = replies_count + 1 where id = ?",
                (reply_id,),
            )
            .await;
        
        if let Err(e) = update_result {
            // Note: In a real app we might want to rollback the previous insert if this fails, 
            // but for simplicity we'll just return the error.
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to update parent: {}", e)})));
        }
    }

    (StatusCode::CREATED, Json(json!({"id": post_id})))
}

use std::sync::Arc;

use axum::{Extension, Json, http::{StatusCode, HeaderMap}};
use serde_json::json;
use turso::Row;

use crate::{State, TokenClaims, token_to_claims, collect_rows};

#[derive(serde::Serialize)]
pub struct Support {
    id: i32,
    subject: String,
    topic: i32,
    description: String,
    userid: i32,
    timestamp: String,
    state: i32,
}

impl Support {
    fn from_row(row: Row) -> Self {
        Support {
            id: row.get(0).unwrap(),
            subject: row.get(1).unwrap(),
            topic: row.get(2).unwrap(),
            description: row.get(3).unwrap(),
            userid: row.get(4).unwrap(),
            timestamp: row.get(5).unwrap(),
            state: row.get(6).unwrap(),
        }
    }
}

pub async fn get_admin_support_all_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let token = match headers.get("Token").and_then(|h| h.to_str().ok()) {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Missing token"}))),
    };

    let opt = token_to_claims(token);
    let claims: TokenClaims;
    match opt {
        Some(data) => claims = data,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid token"}))),
    };

    // Check if user is admin
    let admin_row = connection
        .query("select admin from users where id=?", (claims.id,))
        .await
        .unwrap()
        .next()
        .await
        .unwrap();
    
    let is_admin = match admin_row {
        Some(r) => r.get::<i32>(0).unwrap() == 1,
        None => false,
    };

    if !is_admin {
        return (StatusCode::FORBIDDEN, Json(json!({"error": "Admin only"})));
    }

    let rows = connection
        .query("select * from support limit 50", ())
        .await
        .unwrap();
    let v = collect_rows(rows, Support::from_row).await;

    (StatusCode::OK, Json(json!(v)))
}

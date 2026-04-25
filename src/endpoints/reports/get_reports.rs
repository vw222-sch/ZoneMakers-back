use std::sync::Arc;
use axum::{Extension, Json, http::{StatusCode, HeaderMap}, extract::Path};
use serde_json::json;
use turso::Row;
use crate::{State, token_to_claims, collect_rows};

#[derive(serde::Serialize)]
pub struct Report {
    id: i32,
    reason: String,
    report_type: i32,
    user_id: i32,
    report_id: String,
}

impl Report {
    fn from_row(row: Row) -> Self {
        Report {
            id: row.get(0).unwrap(),
            reason: row.get(1).unwrap(),
            report_type: row.get(2).unwrap(),
            user_id: row.get(3).unwrap(),
            report_id: row.get(4).unwrap(),
        }
    }
}

async fn get_reports(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    page: i32,
    report_type: i32,
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

    let offset = (page - 1) * 25;
    let rows = connection
        .query("select id, reason, type, user_id, report_id from reports where type = ? limit 25 offset ?", (report_type, offset))
        .await
        .unwrap();
    let v = collect_rows(rows, Report::from_row).await;

    (StatusCode::OK, Json(json!(v)))
}

pub async fn get_reports_zone_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Path(page): Path<i32>,
) -> (StatusCode, Json<serde_json::Value>) {
    get_reports(state, headers, page, 1).await
}

pub async fn get_reports_user_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Path(page): Path<i32>,
) -> (StatusCode, Json<serde_json::Value>) {
    get_reports(state, headers, page, 2).await
}

pub async fn get_reports_post_handler(
    state: Extension<Arc<State>>,
    headers: HeaderMap,
    Path(page): Path<i32>,
) -> (StatusCode, Json<serde_json::Value>) {
    get_reports(state, headers, page, 3).await
}

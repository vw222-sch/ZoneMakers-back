use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode, extract::{Path, Query}};
use serde_json::json;

use crate::{State, TokenClaims, token_to_claims, Id};
use crate::endpoints::get_admin_support_all::AdminQuery;

pub async fn delete_admin_support_handler(
    state: Extension<Arc<State>>,
    Path(payload): Path<Id>,
    Query(query): Query<AdminQuery>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let opt = token_to_claims(&query.token);
    let claims: TokenClaims;
    match opt {
        Some(data) => claims = data,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Incorrect credentials"}))),
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

    let res = connection
        .execute("delete from support where id=?", (payload.id,))
        .await;

    match res {
        Ok(_) => (StatusCode::OK, Json(json!("Deleted"))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to delete support request"}))),
    }
}

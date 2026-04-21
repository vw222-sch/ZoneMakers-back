use std::sync::Arc;

use axum::{Extension, Json, extract::Path, http::StatusCode};
use serde::{Serialize, Deserialize};
use serde_json::json;
use turso::Row;

use crate::{State, collect_rows};

#[derive(Serialize)]
struct GottenUser {
    id: i32,
    username: String,
    handle: String,
    bio: String,
    level: i32,
    badges: Vec<i32>,
    banner_img: String,
    theme: i32,
    reputation: i32,
    pinned_badges: Vec<i32>,
    avatar: String,
    verified: bool,
    admin: bool,
}

impl GottenUser {
    fn from_row(row: Row) -> Self {
        GottenUser {
            id: row.get(0).unwrap(),
            username: row.get(1).unwrap(),
            handle: row.get(2).unwrap(),
            bio: row.get(3).unwrap(),
            level: row.get(6).unwrap(),
            badges: serde_json::from_str(&row.get::<String>(7).unwrap()).unwrap(),
            banner_img: row.get(8).unwrap(),
            theme: row.get(9).unwrap(),
            reputation: row.get(10).unwrap(),
            pinned_badges: serde_json::from_str(&row.get::<String>(11).unwrap()).unwrap(),
            avatar: row.get(12).unwrap(),
            verified: row.get(13).unwrap(),
            admin: row.get::<i32>(14).unwrap() == 1,
        }
    }
}

#[derive(Deserialize)]
pub struct HandlePath {
    pub handle: String,
}

pub async fn get_user_handle_handler(
    payload: Path<HandlePath>,
    state: Extension<Arc<State>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let rows = connection
        .query("select * from users where handle=?", (payload.handle.clone(),))
        .await
        .unwrap();
    let v = collect_rows(rows, GottenUser::from_row).await;
    match v.len() {
        0 => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "handle not found"})),
        ),
        1 => (StatusCode::OK, Json(json!(v[0]))),
        _ => panic!("impossible"),
    }
}

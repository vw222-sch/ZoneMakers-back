use std::sync::Arc;

use axum::{Extension, Json, extract::Path, http::StatusCode};
use serde::Serialize;
use serde_json::json;
use turso::Row;

use crate::{Id, State, collect_rows};
#[derive(Serialize)]
struct GottenUser {
    id: i32,
    username: String,
    handle: String,
    bio: String,
    badges: Vec<i32>,
    banner_img: String,
    theme: i32,
    reputation: i32,
    pinned_badges: Vec<i32>,
    avatar: String,
    verified: bool,
    admin: bool,
    region: i32,
}
impl GottenUser {
    fn from_row(row: Row) -> Self {
        GottenUser {
            id: row.get(0).unwrap(),
            username: row.get(1).unwrap(),
            handle: row.get(2).unwrap(),
            bio: row.get(3).unwrap(),
            badges: serde_json::from_str(&row.get::<String>(6).unwrap()).unwrap(),
            banner_img: row.get(7).unwrap(),
            theme: row.get(8).unwrap(),
            reputation: row.get(9).unwrap(),
            pinned_badges: serde_json::from_str(&row.get::<String>(10).unwrap()).unwrap(),
            avatar: row.get(11).unwrap(),
            verified: row.get(12).unwrap(),
            admin: row.get::<i32>(13).unwrap() == 1,
            region: row.get(14).unwrap(),
        }
    }
}

pub async fn get_user_id_handler(
    payload: Path<Id>,
    state: Extension<Arc<State>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let connection = &state.connection;

    let rows = connection
        .query("select * from users where id=?", (payload.id,))
        .await
        .unwrap();
    let v = collect_rows(rows, GottenUser::from_row).await;
    match v.len() {
        0 => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "id not found"})),
        ),
        1 => (StatusCode::OK, Json(json!(v[0]))),
        _ => panic!("impossible"),
    }
}

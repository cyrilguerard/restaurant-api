use rocket::http::Status;
use rocket::*;

use crate::data::connection::SqliteConnection;
use crate::data::entity::MenuItem;
use crate::data::repository::menu_items as db;
use crate::routes::response::ApiError;
use crate::routes::response::ApiResponse;
use crate::services::menu_items as service;

#[get("/menu-items")]
pub fn get_menu_items_rt(conn: SqliteConnection) -> ApiResponse<Vec<MenuItem>, ApiError> {
    let find_items = || db::find_all(&conn);
    match service::find_all(find_items) {
        Ok(items) => ApiResponse::ok(items),
        Err(_) => ApiResponse::error(Status::InternalServerError, String::new()),
    }
}

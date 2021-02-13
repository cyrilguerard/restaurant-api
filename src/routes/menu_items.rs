use rocket::*;

use crate::data::connection::SqliteConnection;
use crate::data::entity::MenuItem;
use crate::data::repository::menu_items;
use crate::routes::response::ApiError;
use crate::routes::response::ApiResponse;

#[get("/menu-items")]
pub fn get_menu_items_rt(conn: SqliteConnection) -> ApiResponse<Vec<MenuItem>, ApiError> {
    ApiResponse::ok(menu_items::find_all(&conn).unwrap())
}

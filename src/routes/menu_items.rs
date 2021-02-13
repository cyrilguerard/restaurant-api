use rocket::*;
use rocket_contrib::databases::rusqlite::Result;
use rocket_contrib::json::Json;

use crate::data::connection::SqliteConnection;
use crate::data::entity::MenuItem;
use crate::data::repository::menu_items;

#[get("/menu-items")]
pub fn get_menu_items_rt(conn: SqliteConnection) -> Json<Vec<MenuItem>> {
    get_menu_items(|| menu_items::find_all(&conn))
}

fn get_menu_items<F>(func: F) -> Json<Vec<MenuItem>>
where
    F: FnOnce() -> Result<Vec<MenuItem>>,
{
    Json(func().unwrap())
}

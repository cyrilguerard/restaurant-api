use rocket::http::Status;
use rocket::*;

use crate::data::connection::SqliteConnection;
use crate::data::entity::Order;
use crate::data::repository::*;
use crate::routes::response::ApiError;
use crate::routes::response::ApiResponse;

const ITEM_NOT_FOUND: &str = "Menu item not found";
const TABLE_NOT_FOUND: &str = "Table not found";
const ORDER_NOT_FOUND: &str = "Order not found";

#[get("/tables/<table_id>/orders")]
pub fn get_orders_rt(conn: SqliteConnection, table_id: u16) -> ApiResponse<Vec<Order>, ApiError> {
    if tables::exists(&conn, table_id).unwrap() {
        ApiResponse::ok(orders::find_all(&conn, table_id).unwrap())
    } else {
        ApiResponse::error(Status::NotFound, String::from(TABLE_NOT_FOUND))
    }
}

#[get("/tables/<table_id>/orders/<order_id>")]
pub fn get_order_by_id_rt(
    conn: SqliteConnection,
    table_id: u16,
    order_id: u32,
) -> ApiResponse<Order, ApiError> {
    if tables::exists(&conn, table_id).unwrap() {
        match orders::find_by_id(&conn, table_id, order_id).unwrap() {
            Some(order) => ApiResponse::ok(order),
            None => ApiResponse::error(Status::NotFound, String::from(ORDER_NOT_FOUND)),
        }
    } else {
        ApiResponse::error(Status::NotFound, String::from(TABLE_NOT_FOUND))
    }
}

#[post("/tables/<table_id>/orders?<item_id>")]
pub fn create_order_rt(
    conn: SqliteConnection,
    table_id: u16,
    item_id: u16,
) -> ApiResponse<Order, ApiError> {
    if tables::exists(&conn, table_id).unwrap() {
        match menu_items::find_by_id(&conn, item_id).unwrap() {
            Some(item) => {
                ApiResponse::created(orders::save(&conn, table_id, Order::new(item)).unwrap())
            }
            None => ApiResponse::error(Status::NotFound, String::from(ITEM_NOT_FOUND)),
        }
    } else {
        ApiResponse::error(Status::NotFound, String::from(TABLE_NOT_FOUND))
    }
}

#[delete("/tables/<table_id>/orders/<order_id>")]
pub fn delete_order_rt(
    conn: SqliteConnection,
    table_id: u16,
    order_id: u32,
) -> ApiResponse<String, ApiError> {
    if tables::exists(&conn, table_id).unwrap() {
        if orders::delete_by_id(&conn, table_id, order_id).unwrap() {
            ApiResponse::no_content(String::new())
        } else {
            ApiResponse::error(Status::NotFound, String::from(ORDER_NOT_FOUND))
        }
    } else {
        ApiResponse::error(Status::NotFound, String::from(TABLE_NOT_FOUND))
    }
}

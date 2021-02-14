use rocket::http::Status;
use rocket::*;

use crate::data::connection::SqliteConnection;
use crate::data::entity::Order;
use crate::data::repository;
use crate::routes::response::ApiError;
use crate::routes::response::ApiResponse;
use crate::services;
use crate::utils::error::Error;

#[get("/tables/<table_id>/orders")]
pub fn get_orders_rt(conn: SqliteConnection, table_id: u16) -> ApiResponse<Vec<Order>, ApiError> {
    let check_table = |table_id| repository::tables::exists(&conn, table_id);
    let find_orders = |table_id| repository::orders::find_all(&conn, table_id);

    match services::tables::find_all(table_id, check_table, find_orders) {
        Ok(orders) => ApiResponse::ok(orders),
        Err(Error::InvalidArgumentError(e)) => ApiResponse::error(Status::NotFound, e),
        Err(_) => ApiResponse::error(Status::InternalServerError, String::new()),
    }
}

#[get("/tables/<table_id>/orders/<order_id>")]
pub fn get_order_by_id_rt(
    conn: SqliteConnection,
    table_id: u16,
    order_id: u32,
) -> ApiResponse<Order, ApiError> {
    let check_table = |table_id| repository::tables::exists(&conn, table_id);
    let find_order = |order_id| repository::orders::find_by_id(&conn, table_id, order_id);

    match services::tables::find_by_id(table_id, check_table, order_id, find_order) {
        Ok(Some(order)) => ApiResponse::ok(order),
        Ok(None) => ApiResponse::error(
            Status::NotFound,
            String::from(services::tables::ORDER_NOT_FOUND),
        ),
        Err(Error::InvalidArgumentError(e)) => ApiResponse::error(Status::NotFound, e),
        Err(_) => ApiResponse::error(Status::InternalServerError, String::new()),
    }
}

#[post("/tables/<table_id>/orders?<item_id>")]
pub fn create_order_rt(
    conn: SqliteConnection,
    table_id: u16,
    item_id: u16,
) -> ApiResponse<Order, ApiError> {
    let check_table = |table_id| repository::tables::exists(&conn, table_id);
    let find_item = |item_id| repository::menu_items::find_by_id(&conn, item_id);
    let create = |table_id, order| repository::orders::save(&conn, table_id, order);

    match services::tables::create_order(table_id, check_table, item_id, find_item, create) {
        Ok(order) => ApiResponse::created(order),
        Err(Error::InvalidArgumentError(e)) => ApiResponse::error(Status::NotFound, e),
        Err(_) => ApiResponse::error(Status::InternalServerError, String::new()),
    }
}

#[delete("/tables/<table_id>/orders/<order_id>")]
pub fn delete_order_rt(
    conn: SqliteConnection,
    table_id: u16,
    order_id: u32,
) -> ApiResponse<String, ApiError> {
    let check_table = |table_id| repository::tables::exists(&conn, table_id);
    let delete = |table_id, order_id| repository::orders::delete_by_id(&conn, table_id, order_id);

    match services::tables::delete_order(table_id, check_table, order_id, delete) {
        Ok(_) => ApiResponse::no_content(String::new()),
        Err(Error::InvalidArgumentError(e)) => ApiResponse::error(Status::NotFound, e),
        Err(_) => ApiResponse::error(Status::InternalServerError, String::new()),
    }
}

use rocket::response::status::NotFound;
use rocket::*;

use rocket_contrib::json::Json;

use crate::data::connection::SqliteConnection;
use crate::data::entity::Order;
use crate::data::repository::orders;

#[get("/tables/<table_id>/orders")]
pub fn get_orders_rt(conn: SqliteConnection, table_id: u16) -> Json<Vec<Order>> {
    Json(orders::find_orders(&conn, table_id).unwrap())
}

#[get("/tables/<table_id>/orders/<order_id>")]
pub fn get_order_by_id_rt(conn: SqliteConnection, table_id: u16, order_id: u32) -> Json<Order> {
    Json(orders::find_order_by_id(&conn, table_id, order_id).unwrap())
}

#[post("/tables/<table_id>/orders?<item_id>")]
pub fn create_order_rt(conn: SqliteConnection, table_id: u16, item_id: u16) -> Json<Order> {
    Json(orders::save_order(&conn, table_id, Order::new(item_id)).unwrap())
}

#[delete("/tables/<table_id>/orders/<order_id>")]
pub fn delete_order_rt(
    conn: SqliteConnection,
    table_id: u16,
    order_id: u16,
) -> std::result::Result<(), NotFound<String>> {
    let count = orders::delete_order_by_id(&conn, table_id, order_id);
    match count {
        Ok(1) => Ok(()),
        Ok(_) => Err(NotFound(String::from("Order not found"))),
        Err(_) => panic!(),
    }
}

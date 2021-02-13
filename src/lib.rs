#![feature(proc_macro_hygiene, decl_macro)]
use rocket::routes;

use std::fs;

pub mod data;
pub mod routes;

fn populate_database(rocket: &rocket::Rocket) {
    let conn = data::connection::SqliteConnection::get_one(&rocket).unwrap();
    let script = fs::read_to_string("schema.sql").unwrap();
    conn.execute_batch(&script).unwrap();
}

pub fn ignite_rocket() -> rocket::Rocket {
    let rocket = rocket::ignite()
        .attach(data::connection::SqliteConnection::fairing())
        .mount(
            "/api",
            routes![
                routes::menu_items::get_menu_items_rt,
                routes::tables::get_orders_rt,
                routes::tables::get_order_by_id_rt,
                routes::tables::create_order_rt,
                routes::tables::delete_order_rt,
            ],
        );
    populate_database(&rocket);
    rocket
}

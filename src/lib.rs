#![feature(proc_macro_hygiene, decl_macro)]
use rocket::routes;

use std::fs;

pub mod data;
pub mod routes;

pub fn initialize_database(rocket: &rocket::Rocket, sql_script: &str) {
    let conn = data::connection::SqliteConnection::get_one(&rocket).unwrap();
    let script = fs::read_to_string(sql_script).unwrap();
    conn.execute_batch(&script).unwrap();
}

pub fn ignite_rocket<F>(post_ignite: F) -> rocket::Rocket
where
    F: Fn(&rocket::Rocket) -> (),
{
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
    post_ignite(&rocket);
    rocket
}

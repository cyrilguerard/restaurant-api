use restaurant_api::ignite_rocket;
use restaurant_api::initialize_database;

use restaurant_api::data::entity::MenuItem;
use restaurant_api::data::entity::Order;

use std::thread;
use std::time::Duration;

use rand::Rng;
use threadpool::ThreadPool;

const API_URL: &str = "http://localhost:8000/api/v1";

fn random_table_id() -> u16 {
    return rand::thread_rng().gen_range(1..=100) as u16;
}

fn random_item_id() -> u16 {
    return rand::thread_rng().gen_range(1..=5) as u16;
}

fn call_get_menu_items() -> Vec<MenuItem> {
    reqwest::get(&format!("{}/menu-items", API_URL))
        .map(|mut r| r.json::<Vec<MenuItem>>())
        .unwrap()
        .unwrap()
}

fn call_get_table_orders(table_id: u16) -> Vec<Order> {
    let mut url = format!("{}/tables/", API_URL);
    url.push_str(&table_id.to_string());
    url.push_str(&"/orders");
    reqwest::get(&url)
        .map(|mut r| r.json::<Vec<Order>>())
        .unwrap()
        .unwrap()
}

fn call_create_order(table_id: u16, item_id: u16) -> Order {
    let mut url = format!("{}/tables/", API_URL);
    url.push_str(&table_id.to_string());
    url.push_str(&"/orders?item_id=");
    url.push_str(&item_id.to_string());

    let client = reqwest::Client::new();
    client
        .post(&url)
        .send()
        .map(|mut r| r.json::<Order>())
        .unwrap()
        .unwrap()
}

fn call_delete_order(table_id: u16) -> () {
    let orders = call_get_table_orders(table_id);
    if orders.len() > 0 {
        let mut url = format!("{}/tables/", API_URL);
        url.push_str(&random_table_id().to_string());
        url.push_str(&"/orders/");
        url.push_str(&orders[0].id.unwrap().to_string());
        let client = reqwest::Client::new();
        client.delete(&url).send().unwrap();
    }
}

fn main() {
    // start clients
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(5000));
        println!("Start clients");

        let pool = ThreadPool::new(20);
        loop {
            pool.execute(move || {
                let wait_time = rand::thread_rng().gen_range(500..3000);

                let call = match wait_time % 4 {
                    0 => || {
                        call_get_menu_items();
                        ()
                    },
                    1 => || {
                        call_get_table_orders(random_table_id());
                        ()
                    },
                    2 => || {
                        call_create_order(random_table_id(), random_item_id());
                        ()
                    },
                    3 => || {
                        call_delete_order(random_table_id());
                        ()
                    },
                    _ => || {},
                };
                call();

                thread::sleep(Duration::from_millis(wait_time));
            });
        }
    });

    ignite_rocket(|rocket| {
        //initialize_database(rocket, &"sql/drop.sql");
        initialize_database(rocket, &"sql/schema.sql");
        initialize_database(rocket, &"sql/data.sql");
    })
    .launch();
}

use lazy_static::lazy_static;

use rocket::http::{ContentType, Status};
use rocket::local::Client;

use serde;
use serde_json;

use restaurant_api::data::entity::MenuItem;
use restaurant_api::ignite_rocket;
use restaurant_api::initialize_database;

lazy_static! {
    static ref CLIENT: Client = Client::new(ignite_rocket(|rocket| {
        initialize_database(rocket, &"sql/schema.sql");
        initialize_database(rocket, &"sql/test-data.sql");
    }))
    .unwrap();
}

#[test]
fn get_menu_items_test() {
    let mut response = CLIENT.get("/api/menu-items").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().unwrap();
    let items: Vec<MenuItem> = serde_json::from_str(&body.as_str()).unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].id, 1);
    assert_eq!(items[0].name, Some(String::from("Sushi")));
}

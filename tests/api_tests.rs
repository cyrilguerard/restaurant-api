use lazy_static::lazy_static;

use rocket::http::{ContentType, Status};
use rocket::local::Client;
use rocket::local::LocalRequest;

use serde::de::DeserializeOwned;
use serde_json;

use restaurant_api::data::entity::MenuItem;
use restaurant_api::data::entity::Order;
use restaurant_api::ignite_rocket;
use restaurant_api::initialize_database;
use restaurant_api::routes::response::ApiError;

lazy_static! {
    static ref CLIENT: Client = Client::new(ignite_rocket(|rocket| {
        initialize_database(rocket, &"sql/drop.sql");
        initialize_database(rocket, &"sql/schema.sql");
        initialize_database(rocket, &"sql/test-data.sql");
    }))
    .unwrap();
}

fn get_then_assert<F, T: DeserializeOwned>(
    url: &str,
    status: Status,
    content_type: ContentType,
    assert_body: F,
) where
    F: FnOnce(T) -> (),
{
    assert_response(CLIENT.get(url), status, content_type, assert_body)
}

fn post_then_assert<F, T: DeserializeOwned>(
    url: &str,
    status: Status,
    content_type: ContentType,
    assert_body: F,
) where
    F: FnOnce(T) -> (),
{
    assert_response(CLIENT.post(url), status, content_type, assert_body)
}

fn delete_then_assert<F, T: DeserializeOwned>(
    url: &str,
    status: Status,
    content_type: ContentType,
    assert_body: F,
) where
    F: FnOnce(T) -> (),
{
    assert_response(CLIENT.delete(url), status, content_type, assert_body)
}

fn assert_response<F, T: DeserializeOwned>(
    request: LocalRequest,
    status: Status,
    content_type: ContentType,
    assert_body: F,
) where
    F: FnOnce(T) -> (),
{
    let mut response = request.dispatch();
    assert_eq!(response.status(), status);
    assert_eq!(response.content_type(), Some(content_type));
    let body = response.body_string().unwrap();
    assert_body(serde_json::from_str(&body.as_str()).unwrap());
}

fn assert_item(item: &MenuItem, id: u16, name: &str) {
    assert_eq!(item.id, id);
    assert_eq!(item.name, Some(String::from(name)));
}

#[test]
fn get_menu_items_test() {
    get_then_assert(
        &"/api/v1/menu-items",
        Status::Ok,
        ContentType::JSON,
        |items: Vec<MenuItem>| {
            assert_eq!(items.len(), 2);
            assert_item(&items[0], 1, "Sushi");
            assert_item(&items[1], 2, "Cheese Burger");
        },
    )
}

#[test]
fn get_orders_test() {
    get_then_assert(
        &"/api/v1/tables/1/orders",
        Status::Ok,
        ContentType::JSON,
        |orders: Vec<Order>| {
            assert_eq!(orders.len(), 3);
            assert_eq!(orders[0].id, Some(1));
            assert_item(&orders[0].item, 1, "Sushi");
            assert_ne!(orders[0].ready_at, None);
            assert_eq!(orders[1].id, Some(2));
            assert_item(&orders[1].item, 2, "Cheese Burger");
            assert_ne!(orders[1].ready_at, None);
        },
    )
}

#[test]
fn get_orders_non_existing_table_test() {
    get_then_assert(
        &"/api/v1/tables/1000/orders",
        Status::NotFound,
        ContentType::JSON,
        |error: ApiError| {
            assert_eq!(error.reason, "Not Found");
            assert_eq!(error.message, "Table not found");
        },
    )
}

#[test]
fn get_one_order_test() {
    get_then_assert(
        &"/api/v1/tables/1/orders/1",
        Status::Ok,
        ContentType::JSON,
        |order: Order| {
            assert_eq!(order.id, Some(1));
            assert_item(&order.item, 1, "Sushi");
            assert_ne!(order.ready_at, None);
        },
    )
}

#[test]
fn get_one_order_non_existing_table_test() {
    get_then_assert(
        &"/api/v1/tables/1000/orders/1",
        Status::NotFound,
        ContentType::JSON,
        |error: ApiError| {
            assert_eq!(error.reason, "Not Found");
            assert_eq!(error.message, "Table not found");
        },
    )
}

#[test]
fn get_one_order_non_existing_order_test() {
    get_then_assert(
        &"/api/v1/tables/1/orders/1000",
        Status::NotFound,
        ContentType::JSON,
        |error: ApiError| {
            assert_eq!(error.reason, "Not Found");
            assert_eq!(error.message, "Order not found");
        },
    )
}

#[test]
fn create_order_test() {
    post_then_assert(
        &"/api/v1/tables/3/orders?item_id=1",
        Status::Created,
        ContentType::JSON,
        |new_order: Order| {
            assert_ne!(new_order.id, None);
            let mut url = String::from("/api/v1/tables/3/orders/");
            url.push_str(&new_order.id.unwrap().to_string());
            get_then_assert(&url, Status::Ok, ContentType::JSON, |order: Order| {
                assert_eq!(order.id, new_order.id);
                assert_item(&order.item, 1, "Sushi");
                assert_ne!(order.ready_at, None);
            });
        },
    );
}

#[test]
fn create_order_non_existing_table_test() {
    post_then_assert(
        &"/api/v1/tables/1000/orders?item_id=1",
        Status::NotFound,
        ContentType::JSON,
        |error: ApiError| {
            assert_eq!(error.reason, "Not Found");
            assert_eq!(error.message, "Table not found");
        },
    );
}

#[test]
fn create_order_non_existing_menu_item_test() {
    post_then_assert(
        &"/api/v1/tables/1/orders?item_id=1000",
        Status::NotFound,
        ContentType::JSON,
        |error: ApiError| {
            assert_eq!(error.reason, "Not Found");
            assert_eq!(error.message, "Menu item not found");
        },
    );
}

#[test]
fn delete_order_test() {
    get_then_assert(
        &"/api/v1/tables/1/orders/1",
        Status::Ok,
        ContentType::JSON,
        |order: Order| {
            assert_eq!(order.id, Some(1));
            delete_then_assert(
                &"/api/v1/tables/1/orders/1",
                Status::NoContent,
                ContentType::JSON,
                |body: String| {
                    assert_eq!(&body, "");
                    get_then_assert(
                        &"/api/v1/tables/1/orders/1",
                        Status::NotFound,
                        ContentType::JSON,
                        |error: ApiError| {
                            assert_eq!(error.reason, "Not Found");
                            assert_eq!(error.message, "Order not found");
                        },
                    );
                },
            );
        },
    );
}

#[test]
fn delete_order_non_existing_table_test() {
    delete_then_assert(
        &"/api/v1/tables/1000/orders/1",
        Status::NotFound,
        ContentType::JSON,
        |error: ApiError| {
            assert_eq!(error.reason, "Not Found");
            assert_eq!(error.message, "Table not found");
        },
    );
}

#[test]
fn delete_order_non_existing_order_test() {
    delete_then_assert(
        &"/api/v1/tables/1/orders/1000",
        Status::NotFound,
        ContentType::JSON,
        |error: ApiError| {
            assert_eq!(error.reason, "Not Found");
            assert_eq!(error.message, "Order not found");
        },
    );
}

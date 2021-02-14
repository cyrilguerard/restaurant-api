use crate::data::entity::*;
use crate::utils::error::Error;

const ITEM_NOT_FOUND: &str = "Menu item not found";
const TABLE_NOT_FOUND: &str = "Table not found";
pub const ORDER_NOT_FOUND: &str = "Order not found";

pub fn find_all<T, O>(table_id: u16, check_table: T, find_orders: O) -> Result<Vec<Order>, Error>
where
    T: Fn(u16) -> Result<bool, Error>,
    O: Fn(u16) -> Result<Vec<Order>, Error>,
{
    if check_table(table_id)? {
        find_orders(table_id)
    } else {
        Err(Error::InvalidArgumentError(String::from(TABLE_NOT_FOUND)))
    }
}

pub fn find_by_id<T, O>(
    table_id: u16,
    check_table: T,
    order_id: u32,
    find_order: O,
) -> Result<Option<Order>, Error>
where
    T: Fn(u16) -> Result<bool, Error>,
    O: Fn(u32) -> Result<Option<Order>, Error>,
{
    if check_table(table_id)? {
        find_order(order_id)
    } else {
        Err(Error::InvalidArgumentError(String::from(TABLE_NOT_FOUND)))
    }
}

pub fn create_order<T, I, O>(
    table_id: u16,
    check_table: T,
    item_id: u16,
    find_item: I,
    create: O,
) -> Result<Order, Error>
where
    T: Fn(u16) -> Result<bool, Error>,
    I: Fn(u16) -> Result<Option<MenuItem>, Error>,
    O: Fn(u16, Order) -> Result<Order, Error>,
{
    if check_table(table_id)? {
        match find_item(item_id)? {
            Some(item) => create(table_id, Order::new(item)),
            None => Err(Error::InvalidArgumentError(String::from(ITEM_NOT_FOUND))),
        }
    } else {
        Err(Error::InvalidArgumentError(String::from(TABLE_NOT_FOUND)))
    }
}

pub fn delete_order<T, O>(
    table_id: u16,
    check_table: T,
    order_id: u32,
    delete: O,
) -> Result<(), Error>
where
    T: Fn(u16) -> Result<bool, Error>,
    O: Fn(u16, u32) -> Result<bool, Error>,
{
    if check_table(table_id)? {
        if delete(table_id, order_id)? {
            Ok(())
        } else {
            Err(Error::InvalidArgumentError(String::from(ORDER_NOT_FOUND)))
        }
    } else {
        Err(Error::InvalidArgumentError(String::from(TABLE_NOT_FOUND)))
    }
}

// #[delete("/tables/<table_id>/orders/<order_id>")]
// pub fn delete_order_rt(
//     conn: SqliteConnection,
//     table_id: u16,
//     order_id: u32,
// ) -> ApiResponse<String, ApiError> {
//     if tables::exists(&conn, table_id).unwrap() {
//         if orders::delete_by_id(&conn, table_id, order_id).unwrap() {
//             ApiResponse::no_content(String::new())
//         } else {
//             ApiResponse::error(Status::NotFound, String::from(ORDER_NOT_FOUND))
//         }
//     } else {
//         ApiResponse::error(Status::NotFound, String::from(TABLE_NOT_FOUND))
//     }
// }

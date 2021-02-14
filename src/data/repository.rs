pub mod menu_items {

    use rocket_contrib::databases::rusqlite::Error as RusqliteError;
    use rocket_contrib::databases::rusqlite::Row;

    use std::time::Duration;

    use crate::data::connection::SqliteConnection;
    use crate::data::entity::MenuItem;
    use crate::utils::error::Error;

    fn map_menu_item(row: &Row) -> MenuItem {
        MenuItem {
            id: row.get::<_, u16>(0),
            name: row.get(1),
            min_cook_time: row
                .get::<_, Option<u32>>(2)
                .map(|min| Duration::from_secs((min * 60) as u64)),
            max_cook_time: row
                .get::<_, Option<u32>>(3)
                .map(|min| Duration::from_secs((min * 60) as u64)),
        }
    }

    pub fn find_by_id(conn: &SqliteConnection, item_id: u16) -> Result<Option<MenuItem>, Error> {
        match conn.prepare("SELECT item_id, name, min_cook_time_min, max_cook_time_min FROM menu_items WHERE item_id = ?") {
            Ok(mut stmt) => match stmt.query_row(&[&item_id], map_menu_item) {
                Ok(item) => Ok(Some(item)),
                Err(RusqliteError::QueryReturnedNoRows) => Ok(None),
                Err(err) => Err(Error::DatabaseAccessError(err)),
            },
            Err(err) => Err(Error::DatabaseAccessError(err)),
        }
    }

    pub fn find_all(conn: &SqliteConnection) -> Result<Vec<MenuItem>, Error> {
        match conn
            .prepare("SELECT item_id, name, min_cook_time_min, max_cook_time_min FROM menu_items")
        {
            Ok(mut stmt) => match stmt.query_map(&[], map_menu_item) {
                Ok(items) => Ok(items.map(|r| r.unwrap()).collect()),
                Err(err) => Err(Error::DatabaseAccessError(err)),
            },
            Err(err) => Err(Error::DatabaseAccessError(err)),
        }
    }
}

pub mod tables {

    use rocket_contrib::databases::rusqlite::Error as RusqliteError;

    use crate::data::connection::SqliteConnection;
    use crate::utils::error::Error;

    pub fn exists(conn: &SqliteConnection, table_id: u16) -> Result<bool, Error> {
        match conn.prepare("SELECT table_id FROM tables WHERE table_id = ?") {
            Ok(mut stmt) => match stmt.query_row(&[&table_id], |row| row.get::<_, u16>(0)) {
                Ok(_) => Ok(true),
                Err(RusqliteError::QueryReturnedNoRows) => Ok(false),
                Err(err) => Err(Error::DatabaseAccessError(err)),
            },
            Err(err) => Err(Error::DatabaseAccessError(err)),
        }
    }
}

pub mod orders {

    use rocket_contrib::databases::rusqlite::Error as RusqliteError;
    use rocket_contrib::databases::rusqlite::Row;

    use crate::data::connection::SqliteConnection;
    use crate::data::entity::MenuItem;
    use crate::data::entity::Order;
    use crate::utils::error::Error;

    fn map_order(row: &Row) -> Order {
        Order {
            id: row.get(0),
            item: MenuItem {
                id: row.get::<_, u16>(1),
                name: row.get(2),
                min_cook_time: None,
                max_cook_time: None,
            },
            ready_at: row.get(3),
        }
    }

    pub fn find_all(conn: &SqliteConnection, table_id: u16) -> Result<Vec<Order>, Error> {
        match conn.prepare("SELECT order_id, mi.item_id, mi.name, ready_at FROM orders o JOIN menu_items mi ON o.item_id = mi.item_id WHERE table_id = ? ORDER BY 1") {
            Ok(mut stmt) => match stmt.query_map(&[&table_id], map_order) {
                Ok(orders) => Ok(orders.map(|r| r.unwrap()).collect()),
                Err(err) => Err(Error::DatabaseAccessError(err)),
            },
            Err(err) => Err(Error::DatabaseAccessError(err)),
        }
    }

    pub fn find_by_id(
        conn: &SqliteConnection,
        table_id: u16,
        order_id: u32,
    ) -> Result<Option<Order>, Error> {
        match conn.prepare("SELECT order_id, mi.item_id, mi.name, ready_at FROM orders o JOIN menu_items mi ON o.item_id = mi.item_id WHERE table_id = ? AND order_id = ?") {
            Ok(mut stmt) => match stmt.query_row(&[&table_id, &order_id], map_order) {
                Ok(order) => Ok(Some(order)),
                Err(RusqliteError::QueryReturnedNoRows) => Ok(None),
                Err(err) => Err(Error::DatabaseAccessError(err)),
            },
            Err(err) => Err(Error::DatabaseAccessError(err)),
        }
    }

    pub fn save(conn: &SqliteConnection, table_id: u16, mut order: Order) -> Result<Order, Error> {
        match conn.prepare("INSERT INTO orders (table_id, item_id, ready_at) VALUES (?, ?, ?)") {
            Ok(mut stmt) => match stmt.insert(&[&table_id, &order.item.id, &order.ready_at]) {
                Ok(id) => {
                    order.id = Some(id as u32);
                    Ok(order)
                }
                Err(err) => Err(Error::DatabaseAccessError(err)),
            },
            Err(err) => Err(Error::DatabaseAccessError(err)),
        }
    }

    pub fn delete_by_id(
        conn: &SqliteConnection,
        table_id: u16,
        order_id: u32,
    ) -> Result<bool, Error> {
        match conn.prepare("DELETE FROM orders WHERE table_id = ? and order_id = ?") {
            Ok(mut stmt) => match stmt.execute(&[&table_id, &order_id]) {
                Ok(cnt) => Ok(cnt == 1),
                Err(err) => Err(Error::DatabaseAccessError(err)),
            },
            Err(err) => Err(Error::DatabaseAccessError(err)),
        }
    }
}

pub mod menu_items {

    use rocket_contrib::databases::rusqlite::Row;
    use rocket_contrib::databases::rusqlite::Result;
    use rocket_contrib::databases::rusqlite::Error;

    use std::time::Duration;

    use crate::data::connection::SqliteConnection;
    use crate::data::entity::MenuItem;

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
    
    pub fn find_by_id(conn: &SqliteConnection, item_id: u16) -> Result<Option<MenuItem>> {
        let mut stmt = conn.prepare("SELECT item_id, name, min_cook_time_min, max_cook_time_min FROM menu_items WHERE item_id = ?")?;
        match stmt.query_row(&[&item_id], map_menu_item) {
            Ok(item) => Ok(Some(item)),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub fn find_all(conn: &SqliteConnection) -> Result<Vec<MenuItem>> {
        let mut stmt = conn.prepare("SELECT item_id, name, min_cook_time_min, max_cook_time_min FROM menu_items")?;
        let res = stmt.query_map(&[], map_menu_item)?;
        Ok(res.map(|r| r.unwrap()).collect())
    }
}

pub mod tables {

    use rocket_contrib::databases::rusqlite::Result;
    use rocket_contrib::databases::rusqlite::Error;

    use crate::data::connection::SqliteConnection;

    pub fn exists(conn: &SqliteConnection, table_id: u16) -> Result<bool> {
        let mut stmt = conn.prepare("SELECT table_id FROM tables WHERE table_id = ?")?;
        match stmt.query_row(&[&table_id], |row|row.get::<_, u16>(0)) {
            Ok(_) => Ok(true),
            Err(Error::QueryReturnedNoRows) => Ok(false),
            Err(err) => Err(err),
        }
    }
}

pub mod orders {

    use rocket_contrib::databases::rusqlite::Row;
    use rocket_contrib::databases::rusqlite::Result;
    use rocket_contrib::databases::rusqlite::Error;

    use crate::data::connection::SqliteConnection;
    use crate::data::entity::MenuItem;
    use crate::data::entity::Order;

    fn map_order(row: &Row) -> Order {
        Order {
            id: row.get(0),
            item: MenuItem {
                id: row.get::<_, u16>(1),
                name: row.get(2),
                min_cook_time: None,
                max_cook_time: None,
            },
            ready_at: row.get(3)
        }
    }

    pub fn find_all(conn: &SqliteConnection, table_id: u16) -> Result<Vec<Order>> {
        let mut stmt = conn.prepare("SELECT order_id, mi.item_id, mi.name, ready_at FROM orders o JOIN menu_items mi ON o.item_id = mi.item_id WHERE table_id = ? ORDER BY 1")?;
        let res = stmt.query_map(&[&table_id], map_order)?;
        Ok(res.map(|r| r.unwrap()).collect())
    }

    pub fn find_by_id(conn: &SqliteConnection, table_id: u16, order_id: u32) -> Result<Option<Order>> {
        let mut stmt = conn.prepare("SELECT order_id, mi.item_id, mi.name, ready_at FROM orders o JOIN menu_items mi ON o.item_id = mi.item_id WHERE table_id = ? AND order_id = ?")?;
        match stmt.query_row(&[&table_id, &order_id], map_order) {
            Ok(order) => Ok(Some(order)),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub fn save(conn: &SqliteConnection, table_id: u16, mut order: Order) -> Result<Order> {
        let mut stmt = conn.prepare("INSERT INTO orders (table_id, item_id, ready_at) VALUES (?, ?, ?)")?;
        let id = stmt.insert(&[&table_id, &order.item.id, &order.ready_at])?;
        order.id = Some(id as u32);
        Ok(order)
    }

    pub fn delete_by_id(conn: &SqliteConnection, table_id: u16, order_id: u32) ->  Result<bool> {
        let mut stmt = conn.prepare("DELETE FROM orders WHERE order_id = ? and table_id = ?")?;
        match stmt.execute(&[&table_id, &order_id]) {
            Ok(cnt) => Ok(cnt == 1),
            Err(err) => Err(err),
        }
    }
}

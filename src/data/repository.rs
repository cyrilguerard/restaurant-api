use crate::data::connection::SqliteConnection;
use rocket_contrib::databases::rusqlite::Result;
use rocket_contrib::databases::rusqlite::Row;
use rocket_contrib::databases::rusqlite::types::ToSql;

fn query<T, F>(conn: &SqliteConnection, sql: &str, params: &[&dyn ToSql], map_row: F) -> Result<Vec<T>>
where
    F: FnMut(&Row) -> T,
{
    let mut stmt = conn.prepare(sql)?;
    let res = stmt.query_map(params, map_row)?;
    Ok(res.map(|r| r.unwrap()).collect())
}

pub mod menu_items {

    use rocket_contrib::databases::rusqlite::Result;
    
    use std::time::Duration;

    use crate::data::connection::SqliteConnection;
    use crate::data::entity::MenuItem;

    pub fn find_all(conn: &SqliteConnection) -> Result<Vec<MenuItem>> {
        super::query(
            conn,
            "SELECT item_id, name, min_cook_time_min, max_cook_time_min FROM menu_items",
            &[],
            |row| MenuItem {
                id: row.get::<_, u16>(0),
                name: row.get(1),
                min_cook_time: row
                    .get::<_, Option<u32>>(2)
                    .map(|secs| Duration::from_secs(secs as u64)),
                max_cook_time: row
                    .get::<_, Option<u32>>(3)
                    .map(|secs| Duration::from_secs(secs as u64)),
            },
        )
    }
}

pub mod orders {

    use rocket_contrib::databases::rusqlite::Result;

    use crate::data::connection::SqliteConnection;
    use crate::data::entity::MenuItem;
    use crate::data::entity::Order;

    pub fn find_orders(conn: &SqliteConnection, table_id: u16) -> Result<Vec<Order>> {
        super::query(
            conn,
            "SELECT order_id, mi.item_id, mi.name, ready_at FROM orders o JOIN menu_items mi ON o.item_id = mi.item_id WHERE table_id = ? ORDER BY 1",
            &[&table_id],
            |row| Order {
                id: row.get(0),
                item: MenuItem {
                    id: row.get::<_, u16>(1),
                    name: row.get(2),
                    min_cook_time: None,
                    max_cook_time: None,
                },
                ready_at: None
            }
        )
    }

    pub fn find_order_by_id(conn: &SqliteConnection, table_id: u16, order_id: u32) -> Result<Order> {

        let mut stmt = conn.prepare("SELECT order_id, mi.item_id, mi.name, ready_at FROM orders o JOIN menu_items mi ON o.item_id = mi.item_id WHERE table_id = ? AND order_id = ?")?;
        stmt.query_row(&[&table_id, &order_id], |row| Order {
                id: row.get(0),
                item: MenuItem {
                    id: row.get::<_, u16>(1),
                    name: row.get(2),
                    min_cook_time: None,
                    max_cook_time: None,
                },
                ready_at: None
            }
        )

    }

    pub fn save_order(conn: &SqliteConnection, table_id: u16, mut order: Order) -> Result<Order> {
        let mut stmt = conn.prepare("INSERT INTO orders (table_id, item_id) VALUES (?, ?)")?;
        let id = stmt.insert(&[&table_id, &order.item.id])?;
        order.id = Some(id as u32);
        Ok(order)
    }

    pub fn delete_order_by_id(conn: &SqliteConnection, table_id: u16, order_id: u16) -> Result<usize> {
        let mut stmt = conn.prepare("DELETE FROM orders WHERE order_id = ? and table_id = ?")?;
        let cnt = stmt.execute(&[&table_id, &order_id])?;
        Ok(cnt)
    }

}

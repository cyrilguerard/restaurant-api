use std::time::Duration;

use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, Default)]
pub struct MenuItem {
    pub id: u16,
    pub name: Option<String>,
    #[serde(skip)]
    pub min_cook_time: Option<Duration>,
    #[serde(skip)]
    pub max_cook_time: Option<Duration>,
}

pub struct Table {
    pub id: u16,
    pub orders: Vec<Order>,
}

#[derive(Serialize, Clone)]
pub struct Order {
    pub id: Option<u32>,
    pub item: MenuItem,
    pub ready_at: Option<NaiveDateTime>,
}

impl Order {
    pub fn new(item_id: u16) -> Order {
        Order {
            id: None,
            item: MenuItem {
                id: item_id,
                ..Default::default()
            },
            ready_at: None,
        }
    }
}

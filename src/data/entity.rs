use std::time::Duration;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MenuItem {
    pub id: u16,
    pub name: Option<String>,
    #[serde(skip)]
    pub min_cook_time: Option<Duration>,
    #[serde(skip)]
    pub max_cook_time: Option<Duration>,
}

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: Option<u32>,
    pub item: MenuItem,
    pub ready_at: Option<NaiveDateTime>,
}

impl Order {
    pub fn new(item: MenuItem) -> Order {
        Order {
            id: None,
            item: item,
            ready_at: None,
        }
    }
}

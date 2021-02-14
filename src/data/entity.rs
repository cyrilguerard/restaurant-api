use std::time::Duration;
use rand::Rng;

use chrono::prelude::*;
use chrono::NaiveDateTime;
use chrono::Duration as CDuration;
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

fn compute_ready_time(item: &MenuItem) -> Option<NaiveDateTime> {
    let min = item.min_cook_time.map(|d| d.as_secs()).unwrap_or(300);
    let max = item.max_cook_time.map(|d| d.as_secs()).unwrap_or(900);

    let cooking_time = rand::thread_rng().gen_range(min..max) as i64;
    Local::now().checked_add_signed(CDuration::seconds(cooking_time)).map(|d| d.naive_local())
}

impl Order {

    pub fn new(item: MenuItem) -> Order {
        let ready_at = compute_ready_time(&item);
        Order {
            id: None,
            item: item,
            ready_at: ready_at,
        }
    }

}

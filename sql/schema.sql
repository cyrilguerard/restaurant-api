PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS menu_items (
	item_id INTEGER PRIMARY KEY AUTOINCREMENT,
   	name TEXT NOT NULL,
    min_cook_time_min INTEGER NOT NULL,
    max_cook_time_min INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS tables (
	  table_id INTEGER PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS orders (
    order_id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_id INTEGER,
    item_id INTEGER,
    ready_at DATETIME,
    FOREIGN KEY (table_id) REFERENCES tables (table_id),
    FOREIGN KEY (item_id) REFERENCES menu_items (item_id)
);

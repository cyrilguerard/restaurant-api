PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS menu_items (
	item_id INTEGER PRIMARY KEY AUTOINCREMENT,
   	name TEXT NOT NULL,
    min_cook_time_min INTEGER NOT NULL,
    max_cook_time_min INTEGER NOT NULL
);

INSERT OR IGNORE INTO menu_items (item_id, name,min_cook_time_min,max_cook_time_min) VALUES (1, 'Sushi', 5, 15);
INSERT OR IGNORE INTO menu_items (item_id, name,min_cook_time_min,max_cook_time_min) VALUES (2, 'Cheese Burger', 2, 7);
INSERT OR IGNORE INTO menu_items (item_id, name,min_cook_time_min,max_cook_time_min) VALUES (3, 'Pizza', 15, 30);
INSERT OR IGNORE INTO menu_items (item_id, name,min_cook_time_min,max_cook_time_min) VALUES (4, 'Steak', 20, 15);
INSERT OR IGNORE INTO menu_items (item_id, name,min_cook_time_min,max_cook_time_min) VALUES (5, 'Ice Cream', 1, 2);

CREATE TABLE IF NOT EXISTS tables (
	  table_id INTEGER PRIMARY KEY
);

INSERT OR IGNORE INTO tables
WITH RECURSIVE cnt(id) AS 
(
    VALUES(1) UNION ALL 
    SELECT id+1 FROM cnt WHERE id<=100
)
SELECT * FROM cnt;

CREATE TABLE IF NOT EXISTS orders (
    order_id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_id INTEGER,
    item_id INTEGER,
    ready_at DATETIME,
    FOREIGN KEY (table_id) REFERENCES tables (table_id),
    FOREIGN KEY (item_id) REFERENCES menu_items (item_id)
);


DELETE FROM orders;
DELETE FROM tables;
DELETE FROM menu_items;

INSERT OR IGNORE INTO menu_items (item_id, name,min_cook_time_min,max_cook_time_min) VALUES (1, 'Sushi', 5, 15);
INSERT OR IGNORE INTO menu_items (item_id, name,min_cook_time_min,max_cook_time_min) VALUES (2, 'Cheese Burger', 2, 7);

INSERT OR IGNORE INTO tables
WITH RECURSIVE cnt(id) AS 
(
    VALUES(1) UNION ALL 
    SELECT id+1 FROM cnt WHERE id<=100
)
SELECT * FROM cnt;

INSERT OR IGNORE INTO orders (order_id, table_id, item_id, ready_at) VALUES (1, 1, 1, '2021-01-01T01:01:01');
INSERT OR IGNORE INTO orders (order_id, table_id, item_id, ready_at) VALUES (2, 1, 2, '2021-02-03T05:06:07');
INSERT OR IGNORE INTO orders (order_id, table_id, item_id, ready_at) VALUES (3, 1, 2, '2021-02-10T09:35:22');
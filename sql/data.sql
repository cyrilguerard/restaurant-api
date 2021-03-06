INSERT OR IGNORE INTO menu_items (item_id, name,min_cook_time_min,max_cook_time_min) VALUES (1, 'Sushi', 5, 15);
INSERT OR IGNORE INTO menu_items (item_id, name,min_cook_time_min,max_cook_time_min) VALUES (2, 'Cheese Burger', 2, 7);
INSERT OR IGNORE INTO menu_items (item_id, name,min_cook_time_min,max_cook_time_min) VALUES (3, 'Pizza', 15, 30);
INSERT OR IGNORE INTO menu_items (item_id, name,min_cook_time_min,max_cook_time_min) VALUES (4, 'Steak', 10, 25);
INSERT OR IGNORE INTO menu_items (item_id, name,min_cook_time_min,max_cook_time_min) VALUES (5, 'Ice Cream', 1, 2);

INSERT OR IGNORE INTO tables
WITH RECURSIVE cnt(id) AS 
(
    VALUES(1) UNION ALL 
    SELECT id+1 FROM cnt WHERE id<=100
)
SELECT * FROM cnt;

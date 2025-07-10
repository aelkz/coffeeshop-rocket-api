-- Your SQL goes here
CREATE TABLE order_item_extras (
   id TEXT PRIMARY KEY,
   order_item_id TEXT NOT NULL,
   extra_id TEXT NOT NULL,
   FOREIGN KEY (order_item_id) REFERENCES order_items(id),
   FOREIGN KEY (extra_id) REFERENCES extras(id)
);
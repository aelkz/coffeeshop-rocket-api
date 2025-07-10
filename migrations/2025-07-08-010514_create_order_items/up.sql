-- Your SQL goes here
CREATE TABLE order_items (
     id TEXT PRIMARY KEY,
     order_id TEXT NOT NULL,
     drink_id TEXT NOT NULL,
     size TEXT NOT NULL CHECK(size IN ('small', 'medium', 'large', 'standard')),
     total_price TEXT NOT NULL, -- Decimal
     FOREIGN KEY (order_id) REFERENCES orders(id),
     FOREIGN KEY (drink_id) REFERENCES drinks(id)
);
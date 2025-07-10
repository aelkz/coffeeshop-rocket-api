-- Your SQL goes here
CREATE TABLE orders (
    id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL,
    employee_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('pending', 'paid', 'preparing', 'ready', 'completed', 'cancelled')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (customer_id) REFERENCES customers(id),
    FOREIGN KEY (employee_id) REFERENCES employees(id)
);
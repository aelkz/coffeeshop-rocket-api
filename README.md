```
•	GET /drinks — list available drinks
•	GET /extras — list available extras
•	GET /customers/{id}
•	POST /orders — place new order
•	GET /orders/{id} — view status and price breakdown
•	PATCH /orders/{id}/status — change status (queued, brewing, etc.)
```

SQLite doesn’t have native UUID support, but it’s fine to use TEXT and uuid::Uuid::new_v4().to_string().

### UUID generation

```
use uuid::Uuid;

let new_drink = NewDrink {
    id: Uuid::new_v4().to_string(),
    name: "Latte".to_string(),
    base_price: Decimal::new(399, 2), // e.g. 3.99
};
```

### Handling datetime in SQLite

Why use TEXT instead of INTEGER?

•	SQLite doesn’t have a dedicated DATETIME type — it’s just TEXT, REAL, or INTEGER.<br>
•	Diesel + chrono::NaiveDateTime expects TEXT formatted in a valid datetime string format (like "2024-07-06 15:30:00").<br>
•	NaiveDateTime has no timezone — matches SQLite’s behavior well.

### Data Types

### 🧩 Rust to SQLite Type Mapping

| Rust Type             | SQLite Type | Notes                                  |
|-----------------------|-------------|----------------------------------------|
| `String` (UUID)       | `TEXT`      | Used for IDs (UUID stored as text)     |
| `Decimal`             | `TEXT`      | Requires `rust_decimal`; store as text |
| `NaiveDateTime`       | `TEXT`      | ISO 8601 format datetime                |
| `NaiveDate`           | `TEXT`      | ISO 8601 format date only              |
| `bool`                | `BOOLEAN`   | Stored as `0` (false) or `1` (true)    |
| Enum (e.g. `OrderStatus`) | `TEXT`  | Stored as enum variant string          |

### Final model structure

```
models/
├── infra/
│   ├── mod.rs
│   └── sqlite_types.rs (handles all custom types)
├── drink_size.rs
├── order_status.rs
├── order.rs
├── order_item.rs
├── order_item_extra.rs
├── drink.rs
├── customer.rs
├── employee.rs
├── extra.rs
└── mod.rs
```

### Running the app

Try automatically fixing some warnings:
```
cargo fix --bin "coffeeshop-rocket-api" --allow-dirty
```

```
cargo clean
cargo build
cargo run
```
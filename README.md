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
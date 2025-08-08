# ☕ Coffee Shop API - Rocket Edition

A modern REST API for coffee shop management built with **Rust**, **Rocket**, and **Diesel ORM**. This project demonstrates clean architecture, type safety, and robust database integration using SQLite.

## 🚀 Features

- **REST API Endpoints**: Full CRUD operations for customers, drinks, orders, and more
- **Type-Safe Database**: Custom SQLite types with compile-time validation
- **Modern Rust**: Uses Rust 2024 edition with latest best practices
- **Rocket Framework**: High-performance async web framework
- **Diesel ORM**: Type-safe database queries with migrations
- **UUID Support**: Proper unique identifiers for all entities
- **Soft Deletes**: Data preservation with logical deletion
- **Financial Precision**: Decimal types for accurate price calculations

## 📡 API Endpoints

### Currently Implemented
- **GET** `/` - Health check endpoint
- **GET** `/api/customers` - List all customers  
- **GET** `/api/customers/{id}` - Get customer by ID
- **POST** `/api/customers` - Create new customer
- **GET** `/api/drinks` - List all available drinks
- **GET** `/api/drinks/{id}` - Get drink by ID  
- **POST** `/api/drinks` - Create new drink

### Planned Endpoints
- **GET** `/api/extras` - List available extras
- **POST** `/api/orders` - Place new order
- **GET** `/api/orders/{id}` - View order status and price breakdown
- **PATCH** `/api/orders/{id}/status` - Change order status (queued, brewing, ready, etc.)
- **GET** `/api/employees` - List employees
- **POST** `/api/employees` - Create new employee

## 🏗️ Architecture

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

## 🔧 Diesel Configuration & Best Practices

### Backend Validation with `check_for_backend`

All our Diesel models use the `#[diesel(check_for_backend(diesel::sqlite::Sqlite))]` attribute. This is **critical** for our project because:

#### **What it does:**
- **Compile-time Type Safety**: Validates that all struct fields are compatible with SQLite at compile time
- **Better Error Messages**: Instead of cryptic runtime errors, you get clear, actionable compile-time feedback
- **Custom Type Validation**: Ensures our custom types (`SqliteDateTime`, `SqliteDecimal`, `SqliteDrinkSize`, etc.) work correctly with SQLite
- **Backend Compatibility**: Prevents accidental use of PostgreSQL or MySQL-specific types

#### **Why it's important for us:**
```rust
// Without check_for_backend - potential runtime errors
#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = customers)]
pub struct Customer {
    pub id: String,                    // ❓ Will this work with SQLite?
    pub created_at: SqliteDateTime,    // ❓ Is this type compatible?
}

// With check_for_backend - compile-time safety
#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = customers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]  // ✅ Validates at compile time
pub struct Customer {
    pub id: String,                    // ✅ Confirmed compatible
    pub created_at: SqliteDateTime,    // ✅ Confirmed compatible
}
```

#### **Real-world benefits:**
- **Prevents Production Bugs**: Catches type mismatches before deployment
- **Faster Development**: Clear error messages help debug issues quickly  
- **Refactoring Safety**: Ensures changes don't break database compatibility
- **Team Collaboration**: New developers get immediate feedback on type issues

#### **Example Error Prevention:**
```rust
// This would fail at compile time with check_for_backend:
pub struct BadExample {
    pub id: i32,                    // ❌ SQLite uses TEXT for our IDs
    pub price: f64,                 // ❌ We use Decimal for precision
    pub created_at: SystemTime,     // ❌ We use NaiveDateTime for SQLite
}
```

### Custom SQLite Types

Our project uses custom wrapper types in `src/models/infra/sqlite_types.rs`:

- **`SqliteDateTime`**: Wraps `NaiveDateTime` for proper SQLite TEXT storage
- **`SqliteDecimal`**: Wraps `Decimal` for precise financial calculations  
- **`SqliteDate`**: Wraps `NaiveDate` for date-only fields
- **`SqliteDrinkSize`**: Wraps `DrinkSize` enum for type-safe size handling
- **`SqliteOrderStatus`**: Wraps `OrderStatus` enum for order state management

These types ensure **data integrity** and **type safety** across the entire application.

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

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+ (with 2024 edition support)
- SQLite 3.x

### Running the Application

1. **Clone and setup:**
   ```bash
   git clone <repository-url>
   cd coffeeshop-rocket-api
   ```

2. **Run database migrations:**
   ```bash
   # Migrations run automatically on startup, but you can also run manually:
   diesel migration run
   ```

3. **Start the server:**
   ```bash
   cargo run
   ```

4. **Test the API:**
   ```bash
   # Health check
   curl http://127.0.0.1:8000/
   
   # List customers
   curl http://127.0.0.1:8000/api/customers
   
   # Create a customer
   curl -X POST http://127.0.0.1:8000/api/customers \
     -H "Content-Type: application/json" \
     -d '{"name": "John Doe", "email": "john@example.com"}'
   ```

### Development Commands

```bash
# Check compilation without running
cargo check

# Run with auto-reload (requires cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Fix some warnings automatically
cargo fix --bin "coffeeshop-rocket-api" --allow-dirty

# Clean build
cargo clean && cargo build
```
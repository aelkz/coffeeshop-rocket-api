// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Nullable<Text>,
        name -> Text,
        email -> Text,
        created_at -> Text,
        updated_at -> Text,
        deleted_at -> Nullable<Text>,
    }
}

diesel::table! {
    drinks (id) {
        id -> Nullable<Text>,
        name -> Text,
        base_price -> Text,
        created_at -> Text,
        updated_at -> Text,
        deleted_at -> Nullable<Text>,
    }
}

diesel::table! {
    employees (id) {
        id -> Nullable<Text>,
        name -> Text,
        email -> Text,
        birth_date -> Text,
        created_at -> Text,
        updated_at -> Text,
        deleted_at -> Nullable<Text>,
    }
}

diesel::table! {
    extras (id) {
        id -> Nullable<Text>,
        name -> Text,
        extra_price -> Text,
        is_available -> Bool,
    }
}

diesel::table! {
    order_item_extras (id) {
        id -> Nullable<Text>,
        order_item_id -> Text,
        extra_id -> Text,
    }
}

diesel::table! {
    order_items (id) {
        id -> Nullable<Text>,
        order_id -> Text,
        drink_id -> Text,
        size -> Text,
        total_price -> Text,
    }
}

diesel::table! {
    orders (id) {
        id -> Nullable<Text>,
        customer_id -> Text,
        employee_id -> Text,
        status -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::joinable!(order_item_extras -> extras (extra_id));
diesel::joinable!(order_item_extras -> order_items (order_item_id));
diesel::joinable!(order_items -> drinks (drink_id));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(orders -> customers (customer_id));
diesel::joinable!(orders -> employees (employee_id));

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    drinks,
    employees,
    extras,
    order_item_extras,
    order_items,
    orders,
);

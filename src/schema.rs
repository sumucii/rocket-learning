use diesel::table;

// src/schema.rs
table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        age -> Integer,
    }
}
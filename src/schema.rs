use diesel::table;

// src/schema.rs
table! {    // 定义一个表
    users (id) {    // 表名为 users，主键为 id
        id -> Integer,      // id 字段为 Integer 类型
        name -> Varchar,    // name 字段为 Varchar 类型
        age -> Integer,     // age 字段为 Integer 类型
    }
}
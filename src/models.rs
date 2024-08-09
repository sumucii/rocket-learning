// src/models.rs
use diesel::prelude::*;
use crate::schema::users;
use serde::{Deserialize, Serialize}; // 导入 serde 的 Deserialize 和 Serialize 特性

#[derive(Queryable, Insertable, Deserialize, Serialize)] // 为 User 结构体实现 Deserialize 和 Serialize 特性
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
use diesel::prelude::{Queryable, Insertable};
use crate::schema::users;
use serde::{Deserialize, Serialize}; // 导入 serde 的 Deserialize 和 Serialize 特性

#[derive(Queryable, Insertable, Deserialize, Serialize)] // 为 User 结构体实现 Deserialize 和 Serialize 特性
#[diesel(table_name = users)] // 指定 User 结构体对应的表名为 users
pub struct User {   // 定义一个 User 结构体
    pub id: i32,    // id 字段为 i32 类型
    pub name: String,   // name 字段为 String 类型
    pub age: i32,   // age 字段为 i32 类型
}
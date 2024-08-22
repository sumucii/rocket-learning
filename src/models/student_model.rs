// 引入 Diesel 预导入模块
use crate::schema::student;
// 引入序列化和反序列化模块
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
// 引入 schema 中的 student 表

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)] // 派生查询、插入、更新、序列化和反序列化特性
#[table_name = "student"] // 指定表名
pub struct Student { // 定义学生结构体
    pub id: i32, // 学生 ID
    pub name: String, // 学生姓名
    pub age: i32, // 学生年龄
}
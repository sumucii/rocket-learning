// src/models/mod.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]  // 为 User 结构体派生序列化和反序列化特性
pub struct User {  // 定义 User 结构体
    pub id: u32,  // 用户 ID，类型为 u32
    pub name: String,  // 用户名，类型为 String
}

#[derive(Serialize, Deserialize)]  // 为 UserLogin 结构体派生序列化和反序列化特性
pub struct LoginUser { // 定义 LoginUser 结构体
    pub name: String,
    pub password: String,
}
// src/controllers/mod.rs
use rocket::serde::json::Json;
use crate::models::basic_model::User;

pub fn index() -> &'static str {  // 定义 index 控制器函数
    "Hello, world!"  // 返回 "Hello, world!" 字符串
}

pub fn hello(name: &str) -> String {  // 定义 hello 控制器函数
    format!("Hello, {}!", name)  // 返回格式化后的字符串
}

pub fn create_user(user: Json<User>) -> Json<User> {  // 定义 create_user 控制器函数
    user  // 返回接收到的用户数据
}

pub fn update_user(user: Json<User>) -> Json<User> {  // 定义 update_user 控制器函数
    user  // 返回接收到的用户数据
}

pub fn delete_user(user: Json<User>) -> Json<User> {  // 定义 delete_user 控制器函数
    user  // 返回接收到的用户数据
}
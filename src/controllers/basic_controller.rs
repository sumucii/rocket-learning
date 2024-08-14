// src/controllers/basic_controller.rs
use rocket::serde::json::Json; // 引入 Json 序列化模块
use crate::models::basic_model::{User, LoginUser}; // 引入 User 和 LoginUser 模型

pub fn index() -> &'static str { // 定义 index 控制器函数
    "Hello, world!" // 返回 "Hello, world!" 字符串
}

pub fn hello(name: &str) -> String { // 定义 hello 控制器函数
    format!("Hello, {}!", name) // 返回格式化后的字符串
}

pub fn create_user(user: Json<User>) -> Json<User> { // 定义 create_user 控制器函数
    user // 返回接收到的用户数据
}

pub fn update_user(user: Json<User>) -> Json<User> { // 定义 update_user 控制器函数
    user // 返回接收到的用户数据
}

pub fn delete_user(user: Json<User>) -> Json<User> { // 定义 delete_user 控制器函数
    user // 返回接收到的用户数据
}

pub fn login_user(loginuser: Json<LoginUser>) -> Json<LoginUser> { // 定义 login_user 控制器函数
    loginuser // 返回接收到的用户数据
}
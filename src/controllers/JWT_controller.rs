// src/controllers/JWT_controller.rs
use crate::models::JWT_model::Claims;
// 引入 Claims 结构体
use rocket::serde::json::Json;
// 引入 Json 序列化模块


pub fn generate_jwt(user: Json<Claims>) -> String { // 定义 generate_jwt 函数
    let secret = "secret"; // 定义密钥
    user.generate_jwt(secret) // 生成 JWT
}

pub fn validate_jwt(token: String) -> bool { // 定义 validate_jwt 函数
    let secret = "secret"; // 定义密钥
    Claims::validate_jwt(&token, secret) // 验证 JWT
}
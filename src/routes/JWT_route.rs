// 引入 Json 序列化模块
use crate::controllers::JWT_controller::{generate_jwt, validate_jwt};
// 引入 JWT 控制器函数
use crate::models::JWT_model::Claims;
use rocket::serde::json::Json;
// 引入 Claims 结构体

#[post("/generate", format = "json", data = "<user>")] // 定义生成 JWT 的 POST 路由
pub fn generate_jwt_route(user: Json<Claims>) -> String { // 定义 generate_jwt_route 函数
    generate_jwt(user) // 调用 generate_jwt 控制器函数
}

#[post("/validate", format = "json", data = "<token>")] // 定义验证 JWT 的 POST 路由
pub fn validate_jwt_route(token: String) -> Json<bool> { // 定义 validate_jwt_route 函数，返回类型为 Json<bool>
    Json(validate_jwt(token)) // 调用 validate_jwt 控制器函数，并将结果包装在 Json 中
}
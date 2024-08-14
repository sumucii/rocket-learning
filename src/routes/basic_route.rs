// src/routes/basic_route.rs
use rocket::serde::json::Json;
// 引入 Json 序列化模块
use crate::controllers::basic_controller::{create_user, delete_user, hello, index, login_user, update_user};
// 引入控制器函数
use crate::models::basic_model::User;
// 引入 User 模型
use crate::models::basic_model::LoginUser;
// 引入 UserLogin 模型

#[get("/")] // 定义 GET 路由
pub fn index_route() -> &'static str { // 定义 index_route 函数
    index() // 调用 index 控制器函数
}

#[get("/hello/<name>")] // 定义带参数的 GET 路由
pub fn hello_route(name: &str) -> String { // 定义 hello_route 函数
    hello(name) // 调用 hello 控制器函数
}

#[post("/post-user", format = "json", data = "<user>")] // 定义 POST 路由
pub fn create_user_route(user: Json<User>) -> Json<User> { // 定义 create_user_route 函数
    create_user(user) // 调用 create_user 控制器函数
}

#[put("/put-user", format = "json", data = "<user>")] // 定义 PUT 路由
pub fn update_user_route(user: Json<User>) -> Json<User> { // 定义 update_user_route 函数
    update_user(user) // 调用 update_user 控制器函数
}

#[delete("/delete-user", format = "json", data = "<user>")] // 定义 DELETE 路由
pub fn delete_user_route(user: Json<User>) -> Json<User> { // 定义 delete_user_route 函数
    delete_user(user) // 调用 delete_user 控制器函数
}

//定义login_user路由，使用POST方法，用UserLogin模型接收数据
#[post("/login-user", format = "json", data = "<loginuser>")] // 定义 POST 路由
pub fn user_login(loginuser: Json<LoginUser>) -> Json<LoginUser> { // 定义 user_login 函数
    login_user(loginuser) // 调用 login_user 控制器函数
}
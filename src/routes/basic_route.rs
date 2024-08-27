// 引入 content 模块
use crate::controllers::basic_controller::{create_user, delete_user, hello, index, login_user, update_user};
// 引入控制器函数
use crate::models::basic_model::{LoginUser, User};
// 引入 Json 序列化模块
use rocket::response::content;
// src/routes/basic_route.rs
use rocket::serde::json::Json;
// 引入 User 和 LoginUser 模型

#[get("/")] // 定义 GET 路由
pub fn index_route() -> &'static str { // 定义 index_route 函数
    index() // 调用 index 控制器函数

}

#[get("/index")] // 定义 index 路由
pub fn index_view() -> content::RawHtml<&'static str> { // 定义 index_view 函数
    content::RawHtml(include_str!("../views/index.html")) // 返回 index.html 页面内容
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

#[post("/login-user", format = "json", data = "<loginuser>")] // 定义 POST 路由
pub fn user_login_route(loginuser: Json<LoginUser>) -> Json<LoginUser> { // 定义 user_login_route 函数
    login_user(loginuser) // 调用 login_user 控制器函数
}

#[get("/index.html")] // 定义 GET 路由
pub fn basic_view() -> content::RawHtml<&'static str> { // 定义 basic_view 函数
    content::RawHtml(include_str!("../views/index.html")) // 返回 index.html 页面内容
}
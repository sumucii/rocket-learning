#[macro_use]  // 使用宏
extern crate rocket;  // 引入 Rocket 库

mod routes;  // 引入 routes 模块
mod controllers;  // 引入 controllers 模块
mod models;  // 引入 models 模块

// 引入 index_route、hello_route、create_user_route、update_user_route、delete_user_route 函数
use routes::basic_route::{index_route, hello_route, create_user_route, update_user_route, delete_user_route};

#[launch]  // 定义 Rocket 启动函数
fn rocket() -> _ {  // 定义 rocket 函数，返回类型为隐式
    rocket::build()  // 构建 Rocket 实例
        .mount("/", routes![index_route, hello_route, create_user_route, update_user_route, delete_user_route])  // 挂载路由
}
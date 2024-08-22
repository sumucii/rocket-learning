// 引入 Rocket 库
extern crate pq_sys;
#[macro_use]  // 使用宏
extern crate rocket;
// 引入 `pq_sys` crate

mod routes;  // 引入 routes 模块
mod controllers;  // 引入 controllers 模块
mod models;
mod views;
mod schema;
// 引入 models 模块

use crate::models::database::StudentsDbConn;
// 引入 index_route、hello_route、create_user_route、update_user_route、delete_user_route 函数
use routes::basic_route::{basic_view, create_user_route, delete_user_route, hello_route, index_route, update_user_route, user_login_route};
// 引入 generate_jwt_route、validate_jwt_route 函数
use routes::student_route::{create_student_route, delete_student_route, read_student_route, update_student_route};
use routes::JWT_route::{generate_jwt_route, validate_jwt_route};


#[launch]  // 定义 Rocket 启动函数
fn rocket() -> _ {  // 定义 rocket 函数，返回类型为隐式
    rocket::build()  // 构建 Rocket 实例
        .mount("/", routes![index_route, hello_route, create_user_route, update_user_route, delete_user_route])  // 挂载路由
        .mount("/", routes![user_login_route])    // 挂载登录路由
        .mount("/", routes![basic_view])  // 挂载视图
        .mount("/jwt", routes![generate_jwt_route, validate_jwt_route])  // 挂载 JWT 路由   路由地址为 /jwt

        .attach(StudentsDbConn::fairing()) // 附加数据库连接池
        .mount("/", routes![create_student_route, read_student_route, update_student_route, delete_student_route]) // 挂载路由
}
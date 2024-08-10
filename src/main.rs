#[macro_use]  // 使用宏
extern crate rocket;  // 引入 Rocket 库

use rocket::serde::json::Json;  // 引入 Rocket 的 JSON 模块
use serde::{Deserialize, Serialize};  // 引入 Serde 的反序列化和序列化模块

#[derive(Serialize, Deserialize)]  // 为 User 结构体派生序列化和反序列化特性
struct User {  // 定义 User 结构体
    id: u32,  // 用户 ID，类型为 u32
    name: String,  // 用户名，类型为 String
}

#[get("/")]  // 定义 GET 请求的路由
fn index() -> &'static str {  // 定义 index 处理函数，返回静态字符串
    "Hello, world!"  // 返回 "Hello, world!" 字符串
}

#[get("/hello/<name>")]  // 定义带参数的 GET 请求的路由
fn hello(name: &str) -> String {  // 定义 hello 处理函数，参数为字符串切片
    format!("Hello, {}!", name)  // 返回格式化后的字符串
}

#[post("/post-user", format = "json", data = "<user>")]  // 定义 POST 请求的路由，数据格式为 JSON
fn create_user(user: Json<User>) -> Json<User> {  // 定义 create_user 处理函数，参数为 JSON 格式的 User
    user  // 返回接收到的用户数据
}

//put请求示例
#[put("/put-user", format = "json", data = "<user>")]  // 定义 PUT 请求的路由，数据格式为 JSON
fn update_user(user: Json<User>) -> Json<User> {  // 定义 update_user 处理函数，参数为 JSON 格式的 User
    user  // 返回接收到的用户数据
}

//delete请求示例
#[delete("/delete-user", format = "json", data = "<user>")]  // 定义 DELETE 请求的路由，数据格式为 JSON
fn delete_user(user: Json<User>) -> Json<User> {  // 定义 delete_user 处理函数，参数为 JSON 格式的 User
    user  // 返回接收到的用户数据
}

#[launch]  // 定义 Rocket 启动函数
fn rocket() -> _ {  // 定义 rocket 函数，返回类型为隐式
    rocket::build()  // 构建 Rocket 实例
        .mount("/", routes![index, hello, create_user])  // 挂载路由
        .mount("/", routes![update_user, delete_user])  // 挂载路由
}
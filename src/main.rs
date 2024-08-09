#[macro_use]
extern crate rocket; // 导入 Rocket 宏
#[macro_use]
extern crate diesel; // 导入 Diesel 宏

use rocket::serde::json::Json; // 导入 Rocket 的 JSON 模块
use diesel::prelude::*; // 导入 Diesel 的预导入模块
use dotenvy::dotenv; // 导入 dotenvy 模块
use std::env; // 导入标准库的 env 模块
use diesel::mysql::MysqlConnection; // 导入 Diesel 的 MysqlConnection 类型
use mysql::OptsBuilder; // 导入 mysql 的 OptsBuilder 类型
use mysql::SslOpts; // 导入 mysql 的 SslOpts 类型

mod schema; // 导入 schema 模块
mod models; // 导入 models 模块

use models::User; // 导入 models 模块中的 User 类型

#[get("/")] // 定义一个 GET 请求的处理函数，路径为 "/"
fn index() -> &'static str { // 处理函数，返回静态字符串
    "Hello, world!" // 返回 "Hello, world!"
}

#[get("/hello")] // 定义一个 GET 请求的处理函数，路径为 "/hello"
fn hello() -> &'static str { // 处理函数，返回静态字符串
    "Hello, Rocket!" // 返回 "Hello, Rocket!"
}

#[get("/hello/<name>")] // 定义一个 GET 请求的处理函数，路径为 "/hello/<name>"
fn hello_name(name: &str) -> String { // 处理函数，返回字符串
    format!("Hello, {}!", name) // 返回 "Hello, <name>!"
}

#[get("/hello/<name>/<age>")] // 定义一个 GET 请求的处理函数，路径为 "/hello/<name>/<age>"
fn hello_name_age(name: &str, age: u8) -> String { // 处理函数，返回字符串
    format!("Hello, {}! You are {} years old.", name, age) // 返回 "Hello, <name>! You are <age> years old."
}

#[post("/post", data = "<name>")] // 定义一个 POST 请求的处理函数，路径为 "/post"
fn post(name: String) -> String { // 处理函数，返回字符串
    format!("Hello, {}! 这是一个post请求", name) // 返回 "Hello, <name>!"
}

#[put("/put", data = "<name>")] // 定义一个 PUT 请求的处理函数，路径为 "/put"
fn put(name: String) -> String { // 处理函数，返回字符串
    format!("Hello, {}! 这是一个put请求", name) // 返回 "Hello, <name>!"
}

#[delete("/delete/<name>")] // 定义一个 DELETE 请求的处理函数，路径为 "/delete/<name>"
fn delete(name: &str) -> String { // 处理函数，返回字符串
    format!("{}! 这是一个delete请求", name) // 返回 "Hello, <name>!"
}

#[post("/add_user", data = "<user>")] // 定义一个 POST 请求的处理函数，路径为 "/add_user"
fn add_user(user: Json<User>) -> Result<&'static str, String> { // 处理函数，接收 JSON 格式的用户数据，返回 Result 类型
    use schema::users::dsl::*; // 使用 users 表的 DSL
    let mut connection = establish_connection().map_err(|e| e.to_string())?; // 获取数据库连接的可变引用，并处理错误
    diesel::insert_into(users) // 插入数据到 users 表
        .values(&*user) // 设置插入的数据
        .execute(&mut connection) // 执行插入操作
        .map_err(|e| e.to_string())?; // 如果插入失败，返回错误信息
    Ok("用户添加成功") // 返回成功信息
}

fn establish_connection() -> Result<MysqlConnection, Box<dyn std::error::Error>> { // 定义一个函数，返回 Result 类型的 MysqlConnection 实例
    dotenv().ok(); // 加载 .env 文件中的环境变量
    let database_url = env::var("DATABASE_URL")?; // 获取 DATABASE_URL 环境变量

    // 配置 SSL 连接
    let opts = OptsBuilder::new() // 创建 OptsBuilder 实例
        .ip_or_hostname(Some(database_url)) // 设置数据库 URL
        .ssl_opts(SslOpts::default()); // 设置 SSL 选项

    let connection = MysqlConnection::establish(&opts.to_string())?; // 建立数据库连接
    Ok(connection) // 返回数据库连接
}

#[launch] // 定义应用程序的入口点
fn rocket() -> _ { // 返回一个 Rocket 实例
    rocket::build() // 构建 Rocket 实例
        .mount("/", routes![index]) // 将处理函数挂载到路径 "/"
        .mount("/", routes![hello]) // 将处理函数挂载到路径 "/hello"
        .mount("/", routes![hello_name]) // 将处理函数挂载到路径 "/hello/<name>"
        .mount("/", routes![hello_name_age]) // 将处理函数挂载到路径 "/hello/<name>/<age>"
        .mount("/", routes![post]) // 将处理函数挂载到路径 "/post"
        .mount("/", routes![put]) // 将处理函数挂载到路径 "/put"
        .mount("/", routes![delete]) // 将处理函数挂载到路径 "/delete/<name>"
        .mount("/", routes![add_user]) // 将处理函数挂载到路径 "/add_user"
}
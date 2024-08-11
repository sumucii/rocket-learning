use rocket::serde::json::Json;
use crate::controllers::basic_controller::{index, hello, create_user, update_user, delete_user};
use crate::models::basic_model::User;

#[get("/")]  // 定义 GET 请求的路由
pub fn index_route() -> &'static str {  // 定义 index 路由处理函数
    index()  // 调用控制器中的 index 函数
}

#[get("/hello/<name>")]  // 定义带参数的 GET 请求的路由
pub fn hello_route(name: &str) -> String {  // 定义 hello 路由处理函数
    hello(name)  // 调用控制器中的 hello 函数
}

#[post("/post-user", format = "json", data = "<user>")]  // 定义 POST 请求的路由
pub fn create_user_route(user: Json<User>) -> Json<User> {  // 定义 create_user 路由处理函数
    create_user(user)  // 调用控制器中的 create_user 函数
}

#[put("/put-user", format = "json", data = "<user>")]  // 定义 PUT 请求的路由
pub fn update_user_route(user: Json<User>) -> Json<User> {  // 定义 update_user 路由处理函数
    update_user(user)  // 调用控制器中的 update_user 函函数
}

#[delete("/delete-user", format = "json", data = "<user>")]  // 定义 DELETE 请求的路由
pub fn delete_user_route(user: Json<User>) -> Json<User> {  // 定义 delete_user 路由处理函数
    delete_user(user)  // 调用控制器中的 delete_user 函数
}
#[macro_use] // 使用宏
extern crate rocket; // 引入 Rocket 库

#[get("/")] // 定义一个 GET 请求的处理函数，路径为 "/"
fn index() -> &'static str { // 处理函数，返回静态字符串
    "Hello, world!" // 返回 "Hello, world!"
}

//自定义一个函数，返回一个 Rocket 实例
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

//post请求
#[post("/post",
    data = "<name>"
)] // 定义一个 POST 请求的处理函数，路径为 "/post"  // data = "<name>" 表示接收请求体中的数据，并将其绑定到 name 参数上  // 该参数的类型是 String
fn post(name: String) -> String { // 处理函数，返回字符串
    format!("Hello, {}! 这是一个post请求", name) // 返回 "Hello, <name>!"
}

//put请求，与post请求类似，只是请求方式不同
#[put("/put", data = "<name>")] // 定义一个 PUT 请求的处理函数，路径为 "/put"
fn put(name: String) -> String { // 处理函数，返回字符串
    format!("Hello, {}! 这是一个put请求", name) // 返回 "Hello, <name>!" format! 宏用于格式化字符串
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
}
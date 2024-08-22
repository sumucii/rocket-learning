// 引入 Diesel 预导入模块
use crate::controllers::student_controller::{create_student, delete_student, read_student, update_student};
// 引入 Json 序列化模块
use crate::models::database::StudentsDbConn;
// 引入数据库连接池
use crate::models::student_model::Student;
// 引入学生模型
use diesel::prelude::*;
use rocket::serde::json::Json;
// 引入学生控制器中的函数

#[post("/students", format = "json", data = "<student>")] // 定义添加学生的 POST 路由
pub async fn create_student_route(conn: StudentsDbConn, student: Json<Student>) -> Json<Student> { // 定义 create_student_route 函数
    create_student(conn, student).await // 调用 create_student 函数并等待其完成
}

#[get("/students/<id>", format = "json")] // 定义查找学生的 GET 路由
pub async fn read_student_route(conn: StudentsDbConn, id: i32) -> Json<Student> { // 定义 read_student_route 函数
    read_student(conn, id).await // 调用 read_student 函数并等待其完成
}

#[put("/students/<id>", format = "json", data = "<student>")] // 定义更新学生的 PUT 路由
pub async fn update_student_route(conn: StudentsDbConn, id: i32, student: Json<Student>) -> Json<Student> { // 定义 update_student_route 函数
    update_student(conn, id, student).await // 调用 update_student 函数并等待其完成
}

#[delete("/students/<id>", format = "json")] // 定义删除学生的 DELETE 路由
pub async fn delete_student_route(conn: StudentsDbConn, id: i32) -> Json<bool> { // 定义 delete_student_route 函数
    delete_student(conn, id).await // 调用 delete_student 函数并等待其完成
}
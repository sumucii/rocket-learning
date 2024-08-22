use rocket::serde::json::Json;
// 引入 Json 序列化模块
use crate::models::database::StudentsDbConn;
// 引入数据库连接池
use crate::models::student_model::Student;
// 引入学生模型
use diesel::prelude::*;
// 引入 Diesel 预导入模块


pub async fn create_student(conn: StudentsDbConn, student: Json<Student>) -> Json<Student> { // 定义 create_student 函数
    conn.run(|c| { // 在数据库连接池中运行
        diesel::insert_into(crate::schema::student::table) // 插入到学生表
            .values(&student.into_inner()) // 插入学生数据
            .get_result(c) // 获取插入结果
    }).await.map(Json).unwrap() // 返回插入结果
}


pub async fn read_student(conn: StudentsDbConn, id: i32) -> Json<Student> { // 定义 read_student 函数
    conn.run(move |c| { // 在数据库连接池中运行
        crate::schema::student::table.find(id) // 查找学生
            .get_result(c) // 获取查找结果
    }).await.map(Json).unwrap() // 返回查找结果
}


pub async fn update_student(conn: StudentsDbConn, id: i32, student: Json<Student>) -> Json<Student> { // 定义 update_student 函数
    conn.run(move |c| { // 在数据库连接池中运行
        diesel::update(crate::schema::student::table.find(id)) // 更新学生
            .set(&student.into_inner()) // 设置更新数据
            .get_result(c) // 获取更新结果
    }).await.map(Json).unwrap() // 返回更新结果
}


pub async fn delete_student(conn: StudentsDbConn, id: i32) -> Json<bool> { // 定义 delete_student 函数
    conn.run(move |c| { // 在数据库连接池中运行
        diesel::delete(crate::schema::student::table.find(id)) // 删除学生
            .execute(c) // 执行删除操作
    }).await.map(|_| Json(true)).unwrap() // 返回删除结果
}
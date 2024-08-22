use rocket_sync_db_pools::{database, diesel};
// 引入数据库连接池和 Diesel ORM

#[database("students")] // 数据库连接池注解
pub struct StudentsDbConn(diesel::PgConnection); // 定义数据库连接池结构体
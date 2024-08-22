use diesel::table;

// 在 `src/schema.rs` 文件中定义学生表的模式
table! {
    student (id) { // 定义学生表 表名为 student
        id -> Int4, // 学生 ID
        name -> Varchar, // 学生姓名
        age -> Int4, // 学生年龄
    }
}
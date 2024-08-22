fn main() {
    println!("cargo:rustc-link-search=native=C:\\PostgreSQL\\16\\lib"); // 指定 libpq.lib 文件所在的目录
    println!("cargo:rustc-link-lib=dylib=libpq"); // 链接 libpq 库
}
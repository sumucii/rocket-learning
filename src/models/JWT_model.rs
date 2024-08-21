// 引入序列化和反序列化模块
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
// 引入时间模块

#[derive(Serialize, Deserialize)] // 为 Claims 结构体派生序列化和反序列化特性
pub struct Claims { // 定义 Claims 结构体
    pub sub: String, // 用户标识
    pub exp: usize, // 过期时间
}

impl Claims {
    pub fn new(sub: String, exp: usize) -> Self { // 定义 new 方法
        Claims { sub, exp } // 返回 Claims 实例
    }

    pub fn generate_jwt(&self, secret: &str) -> String { // 定义生成 JWT 的方法
        let encoding_key = EncodingKey::from_secret(secret.as_ref()); // 创建编码密钥
        encode(&Header::default(), self, &encoding_key).unwrap() // 生成 JWT
    }

    pub fn validate_jwt(token: &str, secret: &str) -> bool { // 定义验证 JWT 的方法
        let decoding_key = DecodingKey::from_secret(secret.as_ref()); // 创建解码密钥
        decode::<Claims>(token, &decoding_key, &Validation::default()).is_ok() // 验证 JWT
    }
}
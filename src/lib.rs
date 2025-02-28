pub mod easy_map;

use rand::rngs::OsRng;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs1::DecodeRsaPublicKey;
// use rsa::pkcs1::EncodeRsaPrivateKey; // 用于导出 PKCS#1 PEM 格式
// use rsa::pkcs1::EncodeRsaPublicKey;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey}; // 用于从 PEM 解析私钥
use std::error::Error;

// sample
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// 解密函数
pub fn decrypt_data(
    private_pem_str: &str,
    encrypted_data_base64: &str,
) -> Result<String, Box<dyn Error>> {
    // 从 PEM 字符串加载私钥
    let private_key = RsaPrivateKey::from_pkcs1_pem(private_pem_str)?;

    // 将 Base64 编码的加密数据解码为字节数组
    let encrypted_data = base64::decode(encrypted_data_base64)?;

    // 解密
    let padding = PaddingScheme::new_pkcs1v15_encrypt(); // 使用与加密时相同的填充方案
    let decrypted_data = private_key
        .decrypt(padding, &encrypted_data)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    // 将解密后的字节数据转换为字符串
    let decrypted_string = String::from_utf8(decrypted_data)?;

    Ok(decrypted_string)
}

// 加密函数
pub fn encrypt_data(public_pem_str: &str, data: &str) -> Result<String, Box<dyn Error>> {
    // 从 PEM 字符串加载公钥
    let public_key = RsaPublicKey::from_pkcs1_pem(public_pem_str)?;

    // 加密
    let mut rng = OsRng;
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let encrypted_data = public_key
        .encrypt(&mut rng, padding, data.as_bytes())
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    // 将加密后的数据转换为 Base64 字符串
    let encrypted_base64 = base64::encode(&encrypted_data);
    Ok(encrypted_base64)
}

pub fn gen_rand(len: usize) -> String {
    let rng = thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

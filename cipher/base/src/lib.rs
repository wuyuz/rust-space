#![allow(unused)]
pub mod utils;

extern crate crypto;
use argon2::{self, Config};
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};
use rand_core::{RngCore, OsRng};

type Aes128Cbc = Cbc<Aes128, Pkcs7>;
type HmacSha256 = Hmac<Sha256>;

// 随机字符串算法
pub fn argon(password: &str, salt: &[u8]) -> Result<(String, String),std::io::Error> {
    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap();
    let mackey = hash.get(..32).unwrap();
    let aeskey = hash.get(32..48).unwrap();
    Ok((aeskey.to_owned(),mackey.to_owned()))
}

// 加密
pub fn encrypt(password:&str, data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut salt = [0u8; 10];
    let mut iv = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut iv);
    // 加密
    let (aeskey, mackey) = argon(password, &salt).unwrap();
    let cipher = Aes128Cbc::new_from_slices(aeskey.as_bytes(), &iv).unwrap();
    let mut cipherdata = cipher.encrypt_vec(data);
    // 加签
    let mut mac = HmacSha256::new_from_slice(mackey.as_bytes()).expect("HMAC can take key of any size");
    mac.update(&salt);
    mac.update(&iv);
    mac.update(&cipherdata);
    let mut r_mac = mac.finalize().into_bytes();
    // 申请数组
    let mut buf:Vec<u8> = Vec::with_capacity(salt.len() + iv.len() +r_mac.len() + data.len());
    buf.append(&mut salt.to_vec());
    buf.append(&mut iv.to_vec());
    buf.append(&mut r_mac.to_vec());
    buf.append(&mut cipherdata);
    Ok(buf)
}

// 解密
pub fn decrypt(password:&str, data: &[u8]) -> Result<String,std::io::Error> {
    let (salt,right) = data.split_at(10);
    let (iv,right) = right.split_at(16);
    let (h_mac,en_data) = right.split_at(32);
    let (aeskey, mackey) = argon(password, &salt).unwrap();
    // 验签
    let mut mac = HmacSha256::new_from_slice(mackey.as_bytes()).expect("HMAC can take key of any size");
    mac.update(&salt);
    mac.update(&iv);
    mac.update(en_data);
    mac.verify(h_mac).expect("Mac Error");
    // 解密
    let cipher = Aes128Cbc::new_from_slices(aeskey.as_bytes(), &iv).expect("Ar Error");
    let decrypted = cipher.decrypt_vec(&en_data).expect("decrypt Error");
    // println!("{:?}",String::from_utf8(decrypted))
    Ok(String::from_utf8(decrypted).unwrap())
}



mod test {
    use argon2::{self, Config};
    use crate::utils::helper;
    #[test]
    fn argon2_work() {
        let password = b"password";
        let salt = helper::rand_6_int(10);
        let config = Config::default();
        let hash = argon2::hash_encoded(password, salt.as_bytes(), &config).unwrap();
        println!("{}",hash);
        let matches = argon2::verify_encoded(&hash, password).unwrap();
        assert!(matches);
    }

    #[test]
    fn verify_hash() {
        let h = "$argon2i$v=19$m=4096,t=3,p=1$NzVYcExka0FW$YWNfIAPfP+AWj7u0shz/WM27ylgoxavEEhHNrircpHI";
        let password = b"password";
        let matches = argon2::verify_encoded(&h, password).unwrap();
        // println!("{}",matches);
        assert!(matches);
    }

    #[test]
    fn run_argon() {
        use super::argon;
        let (r,m) = argon("12233","sdfsf".as_bytes()).unwrap();
        // println!("r:{:?}, m: {:?}",r,m)
    }

    #[test]
    fn encrypt_test() {
        use super::encrypt;
        use super::decrypt;
        let s = "asdqwqwdqwd";
        use std::time::Instant;
        let start = Instant::now();
        let e = encrypt("12333", s.as_bytes());
        decrypt("12333", &e.unwrap());
        println!("time cost: {:?} ms", start.elapsed().as_millis());// ms
    }
}
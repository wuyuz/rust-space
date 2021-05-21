#![allow(unused)]
mod utils;

use argon2::{self, Config};
use crate::utils::helper;
use std::str;
extern crate crypto;

use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, NewBlockCipher,
    generic_array::GenericArray,
};


// 随机字符串算法
pub fn argon(password: &str, salt: &str) -> Result<(String, String),std::io::Error> {
    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
    let mackey = hash.get(..36).unwrap();
    let aeskey = hash.get(32..48).unwrap();
    Ok((aeskey.to_owned(),mackey.to_owned()))
}

// 加密算法
pub fn encrypt(password:&str, data: &[u8]) {
    let salt = helper::rand_6_int(10);
    let iv = helper::rand_6_int(8);
    println!("{},{}", salt, iv);

    let (aeskey, mackey) = argon(password, &salt).unwrap();
    let key = GenericArray::clone_from_slice(aeskey.as_bytes());

    let cipher = Aes128::new(&key);
    let mut vec: Vec<u8> = Vec::with_capacity(data.len());

    for block in data.chunks(16) {
        let mut cipher_block = GenericArray::clone_from_slice(&block);
        println!("1{:?}", String::from_utf8(cipher_block.to_vec()));
        cipher.encrypt_block(&mut cipher_block);
        println!("2{:?}", cipher_block);

        cipher.decrypt_block(&mut cipher_block);
        println!("3{:?}", String::from_utf8(cipher_block.to_vec()))
    }

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
        println!("{}",matches);
        assert!(matches);
    }


    #[test]
    fn run_argon() {
        use super::argon;
        let (r,m) = argon("12233","sdfsf").unwrap();
        println!("r:{:?}, m: {:?}",r,m)
    }

    #[test]
    fn encrypt_test() {
        use super::encrypt;
        encrypt("12333", "Rust122222222222".as_bytes());
    }
}
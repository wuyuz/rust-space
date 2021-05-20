// use argon2::{self, Config, ThreadMode, Variant, Version};
// use rand::prelude::*;
// use crate::utils::helper;

// pub fn argon2(p :String) {

// }




mod test {
    use argon2::{self, Config, ThreadMode, Variant, Version};
    use rand::prelude::*;
    use String;
    use crate::utils::helper;

    #[test]
    fn argon2_work() {
        let password = b"password";
        let salt = b"othersalt";
        // let data = helper::rand_6_int();
        // println!("{:?}",data.as_bytes());
        // let config = Config {
        //     variant: Variant::Argon2i,
        //     version: Version::Version13,
        //     mem_cost: 65536,
        //     time_cost: 10,
        //     lanes: 4,
        //     thread_mode: ThreadMode::Parallel,
        //     secret: &[],
        //     ad: &[],
        //     hash_length: 64
        // };
        let config = Config::default();
        // let hash = argon2::hash_encoded(password, data.as_bytes(), &config).unwrap();
        let hash = argon2::hash_encoded(password, salt, &config).unwrap();
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
}
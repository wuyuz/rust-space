// 很多时候系统只需要拥有唯一的一个对象， 单例对象的类只允许一个实例的存在
// https://github.com/RustStudy/design_patterns/blob/master/singleton/src/simple.rs

use std::{mem::MaybeUninit, sync::{Mutex,Once}};

#[derive(Debug)]
struct Config {
    config_str: String
}


// 单例函数
// 用Maybeunit创建未被初始化的内存,
fn single_config() -> &'static Mutex<Config> { // 返回‘static
    static mut CONFIG: MaybeUninit<Mutex<Config>> = MaybeUninit::uninit();
    static ONCE:Once = Once::new();
    ONCE.call_once(||unsafe {
        CONFIG.as_mut_ptr().write(Mutex::new(Config{
            config_str: "test config".to_string(),
        }));
    });

    unsafe{&*CONFIG.as_ptr()}
}


mod test {

    use super::*;

    #[test]
    fn single_1() {
        let config1 = single_config();
        let config2 = single_config();
        println!("{:?}", config1);

        // 改变config1
        {
            let mut conf = config1.lock().unwrap();
            conf.config_str = "config2".to_string();
        }
        // 改变config2是否变化
        println!("{:?}",config1);
        println!("{:?}",config2);

        // 通过子线程修改config2
        let handle = std::thread::spawn(move || {
            let mut config2 = config2.lock().unwrap();
            config2.config_str = "config change".to_string();
        });

        handle.join().unwrap();
        // 查看config1是否变化
        println!("{:?}",config1);
        println!("{:?}",config2);

    }

}
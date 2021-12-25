

mod test {
    #[test]
    fn test_mutex() {
        use std::sync::Mutex;

        let my_mutex = Mutex::new(5);  // 不需要mut
        let mut mutex_changer = my_mutex.lock().unwrap();
    
        println!("{:?}",my_mutex);  // This prints "Mutex { data: <locked> }"
                                    // So we can't access the data with my_mutex now,
                                    // only with mutex_changer
        println!("{:?}", mutex_changer);  // This prints 5. Let's change it to 6.

        *mutex_changer = 6; // mutex_changer is a MutexGuard<i32> so we use * to change the i32
    
        println!("{:?}", mutex_changer); // Now it says 6


        let my_mutex = Mutex::new(5);
        {
            let mut mutex_changer = my_mutex.lock().unwrap();  
            *mutex_changer = 6;  // 会调用drop变量
        }  // mutex_changer goes out of scope - now it is gone. It is not locked anymore
    
        println!("{:?}", my_mutex); // Now it says: Mutex { data: 6 }

        let my_mutex = Mutex::new(5);

        for _ in 0..100 {
            *my_mutex.lock().unwrap() += 1; // locks and unlocks 100 times
        }

        println!("{:?}", my_mutex);
    }

    // #[test]
    // fn test_lock_forever() {
    //     use std::sync::Mutex;

    //     let my_mutex = Mutex::new(5);
    //     let mut mutex_changer = my_mutex.lock().unwrap(); // mutex_changer has the lock
    //     let mut other_mutex_changer = my_mutex.lock().unwrap(); // other_mutex_changer wants the lock
    //                                                             // the program is waiting
    //                                                             // and waiting
    //                                                             // and will wait forever.
    //     println!("This will never print...");
    // }

    // RwLock的意思是 "读写锁"。它像Mutex，但也像RefCell。你用.write().unwrap()代替.lock().unwrap()来改变它。但你也可以用.read().unwrap()来获得读权限。它和RefCell一样，遵循这些规则:
        // 很多.read()变量可以
        // 一个.write()变量可以
        // 但多个.write()或.read()与.write()一起是不行的
    // #[test]
    // fn test_relock() {
    //     use std::sync::RwLock;
    //     use std::mem::drop;

    //     let my_rwlock = RwLock::new(5);

    //     let read1 = my_rwlock.read().unwrap();  
    //     let read2 = my_rwlock.read().unwrap(); // ok
    
    //     println!("{:?}, {:?}", read1, read2);
    
    //     drop(read1);
    //     drop(read2); // we dropped both, so we can use .write() now
    
    //     let mut write1 = my_rwlock.write().unwrap();
    //     *write1 = 6;
    //     drop(write1);
    //     println!("{:?}", my_rwlock);
    // }

    // Cow是一个非常方便的枚举
    #[test]
    fn test_cow() {
        use std::borrow::Cow;

        fn modulo_3(input: u8) -> Cow<'static, str> {
            match input % 3 {
                0 => "Remainder is 0".into(),
                1 => "Remainder is 1".into(),
                remainder => format!("Remainder is {}", remainder).into(),
            }
        }

        for number in 1..=6 {
            match modulo_3(number) {
                Cow::Borrowed(message) => println!("{} went in. The Cow is borrowed with this message: {}", number, message),
                Cow::Owned(message) => println!("{} went in. The Cow is owned with this message: {}", number, message),
            }
        }
    }

}
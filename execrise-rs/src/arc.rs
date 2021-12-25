
/*
Arc的意思是 "atomic reference counter"(原子引用计数器)。原子的意思是它使用计算机的处理器，所以每次只写一次数据。
这一点很重要，因为如果两个线程同时写入数据，你会得到错误的结果。

如果线程1和线程2一起启动，也许就会出现这种情况。
    线程1看到10，写下11，然后线程2看到11，写下12 然后线程2看到11，写入12。到目前为止没有问题。
    线程1看到12。同时，线程2看到12。线程一看到13，写下13 线程2也写了13 现在我们有13个，但应该是14个 Now we have 13, but it should be 14. 这是个大问题。

Arc使用处理器来确保这种情况不会发生，所以当你有线程时必须使用这种方法。不过不建议单线程上用Arc，因为Rc更快一些。
不过你不能只用一个Arc来改变数据。所以你用一个Mutex把数据包起来，然后用一个Arc把Mutex包起来。
*/

mod test {

    #[test]
    fn test_arc() {
        use std::sync::{Arc, Mutex};

        let my_number = Arc::new(Mutex::new(0));

        let my_number1 = Arc::clone(&my_number);
        let my_number2 = Arc::clone(&my_number);

        let thread1 = std::thread::spawn(move || { // Only the clone goes into Thread 1
            for _ in 0..10 {
                *my_number1.lock().unwrap() +=1; // Lock the Mutex, change the value
            }
        });

        let thread2 = std::thread::spawn(move || { // Only the clone goes into Thread 2
            for _ in 0..10 {
                *my_number2.lock().unwrap() += 1;
            }
        });

        thread1.join().unwrap();
        thread2.join().unwrap();
        println!("Value is: {:?}", my_number);
        println!("Exiting the program");


        // 合并两个线程
        let my_number = Arc::new(Mutex::new(0));
        let mut handle_vec = vec![]; // JoinHandles will go in here
    
        for _ in 0..2 { // do this twice
            let my_number_clone = Arc::clone(&my_number); // Make the clone before starting the thread
            let handle = std::thread::spawn(move || { // Put the clone in
                for _ in 0..10 {
                    *my_number_clone.lock().unwrap() += 1;
                }
            });
            handle_vec.push(handle); // save the handle so we can call join on it outside of the loop
                                     // If we don't push it in the vec, it will just die here
        }
    
        handle_vec.into_iter().for_each(|handle| handle.join().unwrap()); // call join on all handles
        println!("{:?}", my_number);
    }
}
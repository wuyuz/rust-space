
/*
A channel is an easy way to use many threads that send to one place.它们相当流行，因为它们很容易组合在一起。你可以在Rust中用std::sync::mpsc创建
一个channel。mpsc的意思是 "多个生产者，单个消费者"，所以 "many threads sending to one place"。要启动一个通道，你可以使用 channel()。
这将创建一个 Sender 和一个 Receiver，它们被绑在一起

 */

 mod  test {
    use std::sync::mpsc::channel;

     #[test]
     fn test_channel() {
        let (sender, receiver) = channel();

        sender.send(5).unwrap();  //  一旦你开始使用Sender和Receiver，Rust就能猜到类型。
        println!("{}", receiver.recv().unwrap());  // 5
     }

     // channel就像Arc一样，因为你可以克隆它，并将克隆的内容发送到其他线程中。让我们创建两个线程，并将值发送到receiver。这段代码可以工作，但它并不完全是我们想要的。
     #[test]
     fn test_channel_thread() {
        let (sender, receiver) = channel();
        let sender_clone = sender.clone();
    
        std::thread::spawn(move|| { // move sender in
            sender.send("Send a &str this time").unwrap();
        });
    
        std::thread::spawn(move|| { // move sender_clone in
            sender_clone.send("And here is another &str").unwrap();
        });
    
        println!("{}", receiver.recv().unwrap());
        // 两个线程开始发送，然后我们println!。它可能会打印 Send a &str this time 或 And here is another &str，这取决于哪个线程先完成。
     }
     

     #[test]  
     fn test_channel_thread_2() {
        let (sender, receiver) = channel();
        let sender_clone = sender.clone();
        let mut handle_vec = vec![]; // Put our handles in here
    
        handle_vec.push(std::thread::spawn(move|| {  // push this into the vec
            sender.send("Send a &str this time").unwrap();
        }));
    
        handle_vec.push(std::thread::spawn(move|| {  // and push this into the vec
            sender_clone.send("And here is another &str").unwrap();
        }));
    
        for _ in handle_vec { // now handle_vec has 2 items. Let's print them
            println!("{:?}", receiver.recv().unwrap());
        }
     }

     #[test]
     fn test_add_1() {
        use std::thread::spawn;

        let (sender, receiver) = channel();

        let hugevec = vec![0; 100_0000];
        let mut newvec=vec![];
        let mut handle_vec = vec![];

        for i in 0..10 {
            let sender_clone = sender.clone();
            let mut work:Vec<u8> = Vec::with_capacity(hugevec.len()/10);

            work.extend(&hugevec[i*100_000..(i+1)*100_000]);
            let handle = spawn(move||{
                for number in work.iter_mut() {
                    *number += 1
                }
                sender_clone.send(work).unwrap();
            });
            handle_vec.push(handle)
        }

        for handle in handle_vec { // stop until the threads are done
            handle.join().unwrap();
        }
    
        while let Ok(results) = receiver.try_recv() {
            newvec.push(results); // push the results from receiver.recv() into the vec
        }
    
        // Now we have a Vec<Vec<u8>>. To put it together we can use .flatten()
        let newvec = newvec.into_iter().flatten().collect::<Vec<u8>>(); // Now it's one vec of 1_000_000 u8 numbers
    
        println!("{:?}, {:?}, total length: {}", // Let's print out some numbers to make sure they are all 1
            &newvec[0..10], &newvec[newvec.len()-10..newvec.len()], newvec.len() // And show that the length is 1_000_000 items
        );
    
        for number in newvec { // And let's tell Rust that it can panic if even one number is not 1
            if number != 1 {
                panic!();
            }
        }
     }
 }
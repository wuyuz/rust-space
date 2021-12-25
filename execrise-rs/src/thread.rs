
// pub fn spawn<F, T>(f: F) -> JoinHandle<T>
// where
//     F: FnOnce() -> T,
//     F: Send + 'static,
//     T: Send + 'static,

/*
现在我们就来了解一下三种类型的闭包。这三种类型是
    FnOnce: 取整个值
    FnMut: 取一个可变引用
    Fn: 取一个普通引用
如果可以的话，闭包会尽量使用Fn。但如果它需要改变值，它将使用 FnMut，而如果它需要取整个值，它将使用 FnOnce。FnOnce是个好名字，因为它解释了它的作用:它取一次值，然后就不能再取了。
 */

mod test {
    #[test]
    fn test_thread() {
        for _ in 0..10 {
            let handle = std::thread::spawn(|| {
                println!("I am printing something");
            });
    
            handle.join(); // Wait for the threads to finish
        }

        let my_string = String::from("I will go into the closure");
        let my_closure = || println!("{}", my_string);
        my_closure();
        my_closure();

        // String没有实现Copy，所以my_closure()是Fn: 它拿到一个引用
        // 如果我们改变my_string，它变成FnMut。
        let mut my_string = String::from("I will go into the closure");
        let mut my_closure = || {
            my_string.push_str(" now");
            println!("{}", my_string);
        };
        my_closure();
        my_closure();

        // 而如果按值获取，则是FnOnce
        let my_vec: Vec<i32> = vec![8, 9, 10];
        let my_closure = || {
            my_vec
                .into_iter() // into_iter takes ownership
                .map(|x| x as u8) // turn it into u8
                .map(|x| x * 2) // multiply by 2
                .collect::<Vec<u8>>() // collect into a Vec
        };
        let new_vec = my_closure();  // 我们是按值取的,所以不能多跑my_closure()次
        println!("{:?}", new_vec);
    }

    // 如果你在线程中需要一个来自线程外的值，你需要使用move
    #[test]
    fn test_move() {
        let mut my_string = String::from("Can I go inside the thread?");

        let handle = std::thread::spawn(move|| {
            println!("{}", my_string);  // now my_string is being used as a reference
        });
        
        // 需要move将变量交给线程，否则后面删除了都不知道
        //std::mem::drop(my_string);  // ⚠️ we can't drop, because handle has it. So this won't work
        handle.join();
    }


    #[test]
    fn test_fnmut() {
        #[derive(Debug)]
        struct City {
            name: String,
            years: Vec<u32>,
            populations: Vec<u32>,
        }

        impl City {
            fn new(name: &str, years: Vec<u32>, populations: Vec<u32>) -> Self {

                Self {
                    name: name.to_string(),
                    years,
                    populations,
                }
            }

            fn city_data<F>(&mut self, mut f: F) // We bring in self, but only f is generic F. f is the closure

            where
                F: FnMut(&mut Vec<u32>, &mut Vec<u32>), // The closure takes mutable vectors of u32
                                                        // which are the year and population data
            {
                f(&mut self.years, &mut self.populations) // Finally this is the actual function. It says
                                                        // "use a closure on self.years and self.populations"
                                                        // We can do whatever we want with the closure
            }
        }


        let years = vec![
            1372, 1834, 1851, 1881, 1897, 1925, 1959, 1989, 2000, 2005, 2010, 2020,
        ];
        let populations = vec![
            3_250, 15_300, 24_000, 45_900, 58_800, 119_800, 283_071, 478_974, 400_378, 401_694,
            406_703, 437_619,
        ];
        // Now we can create our city
        let mut tallinn = City::new("Tallinn", years, populations);

        // Now we have a .city_data() method that has a closure. We can do anything we want.

        // First let's put the data for 5 years together and print it.
        tallinn.city_data(|city_years, city_populations| { // We can call the input anything we want
            let new_vec = city_years
                .into_iter()
                .zip(city_populations.into_iter()) // Zip the two together
                .take(5)                           // but only take the first 5
                .collect::<Vec<(_, _)>>(); // Tell Rust to decide the type inside the tuple
            println!("{:?}", new_vec);
        });

        // Now let's add some data for the year 2030
        tallinn.city_data(|x, y| { // This time we just call the input x and y
            x.push(2030);
            y.push(500_000);
        });

        // We don't want the 1834 data anymore
        tallinn.city_data(|x, y| {
            let position_option = x.iter().position(|x| *x == 1834);
            if let Some(position) = position_option {
                println!(
                    "Going to delete {} at position {:?} now.",
                    x[position], position
                ); // Confirm that we delete the right item
                x.remove(position);
                y.remove(position);
            }
        });

        println!(
            "Years left are {:?}\nPopulations left are {:?}",
            tallinn.years, tallinn.populations
        );

    }
}
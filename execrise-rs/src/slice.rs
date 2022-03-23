#[warn(unused_variables)]
fn file_ext(name: &str) -> Option<&str> {
    // this does not create a new string - it returns
    // a slice of the argument.
    name.split(".").last()
}

mod test {
    use super::*;

    // #[test]
    // fn t_slice() {
    //     // Rust里有切片(slice),切片是对多个连续元素的引用。 你可以借用一个vector的切片
    //     let v = vec![1, 2, 3, 4, 5];
    //     let v2 = &v[2..4];  // 借用slice
    //     println!("v2 = {:?}", v2);  // v2 = [3, 4]
    //
    //     // .. 魔术语法：索引操作符(foo[index])被Index和IndexMut两个traits重载。..语法只是range字面量。Ranges是定义在标准库的一些结构体。
    //     // 它们可以是开放-闭合的，如果想要包含最右边的元素，可以使用=来处理
    //     // 0 or greater
    //     println!("{:?}", (0..).contains(&100)); // true
    //     // strictly less than 20
    //     println!("{:?}", (..20).contains(&20)); // false
    //     // 20 or less than 20
    //     println!("{:?}", (..=20).contains(&20)); // true
    //     // only 3, 4, 5
    //     println!("{:?}", (3..6).contains(&4)); // true
    //
    //     // 借用规则也适用于切片(slice)， 前提是类型实现了Copy
    //     fn tail(s: &[u8]) -> &[u8] {
    //         &s[1..]
    //     }
    //
    //     // 等价于
    //     fn tail_1<'a>(s: &'a [u8]) -> &'a [u8] {
    //         &s[1..]
    //     }
    //
    //     // 等价于
    //     let y = {
    //         let x = &[1, 2, 3, 4, 5];
    //         tail(x)
    //     };
    //     println!("y = {:?}", y);
    //
    //     /*
    //     let y = {
    //         let v = vec![1, 2, 3, 4, 5];
    //         tail(&v)  // 报错，因为vector分配在堆上，它的生命周期是不是'static'。&str类型的值是真正的切片
    //         // error: `v` does not live long enough
    //     };
    //     println!("y = {:?}", y);
    //     */
    //
    //     let name = "Read me. Or don't.txt";  // &str 适用于切片
    //     if let Some(ext) = file_ext(name) {  // file extension: txt
    //         println!("file extension: {}", ext);
    //     } else {
    //         println!("no file extension");
    //     }
    // }

    // #[test]
    // fn t_result() {
    //     // 调用失败，函数典型情况下会返回一个Result
    //     let s = std::str::from_utf8(&[240, 159, 141, 137]);
    //     println!("{:?}", s);  // Ok("🍉")
    //
    //     let s = std::str::from_utf8(&[94,94,92]);
    //     println!("{:?}", s);  // 不是一个字符所以报错
    //     // prints: Err(Utf8Error { valid_up_to: 0, error_len: Some(1) })
    //
    //     //如果你想在失败的时候直接panic,你可以使用unwrap(),或者,如果想要输出自定义信息的话可以使用.expect()
    //     let s = std::str::from_utf8(&[195, 40]).unwrap();
    //     let s = std::str::from_utf8(&[195, 40]).expect("error msg");
    // }

    #[test]
    fn deal_with_err() {
        match std::str::from_utf8(&[240, 159, 141, 137]) {
            Ok(s) => println!("{}", s),
            // Err(e) => panic!(e),
            Err(e) => println!("{}",e),
        };

        // if let
        if let Ok(s) = std::str::from_utf8(&[240, 159, 141, 137]) {
            println!("{}", s);
        }
    }

    #[test]
    fn dref() {
        // *操作符可以用于解引用,但是在访问内部字段或者调用方法的时候可以不进行解引用操作
        struct Point {
            x: f64,
            y: f64,
        }
        let p = Point { x: 1.0, y: 3.0 };
        let p_ref = &p;
        println!("({}, {})", p_ref.x, p_ref.y); // 不需要解引用

        // 并且解引用操作只能在实现了Copy trait的类型上使用
        struct Point_1 {
            x: f64,
            y: f64,
        }
        
        fn negate(p: Point_1) -> Point_1 {
            Point_1 {
                x: -p.x,
                y: -p.y,
            }
        }
        let p = Point { x: 1.0, y: 3.0 };
        let p_ref = &p;
        // negate(*p_ref);
        // error: cannot move out of `*p_ref` which is behind a shared reference

        #[derive(Clone, Copy)]  // 实现了Copy才可以解引用
        struct Point_2 {
            x: f64,
            y: f64,
        }

        fn negate_2(p: Point_2) -> Point_2 {
            Point_2 {
                x: -p.x,
                y: -p.y,
            }
        } 
        let p = Point_2 { x: 1.0, y: 3.0 };
        let p_ref = &p;
        negate_2(*p_ref); // ...and now this works 
        
    }
}
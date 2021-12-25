
mod test {

    #[test]
    fn test_box() {
        #![allow(dead_code)] // Tell the compiler to be quiet
        use std::mem::size_of; // This gives the size of a type

        trait JustATrait {} // We will implement this on everything

        enum EnumOfNumbers {
            I8(i8),
            AnotherI8(i8),
            OneMoreI8(i8),
        }
        impl JustATrait for EnumOfNumbers {}

        struct StructOfNumbers {
            an_i8: i8,
            another_i8: i8,
            one_more_i8: i8,
        }
        impl JustATrait for StructOfNumbers {}

        enum EnumOfOtherTypes {
            I8(i8),
            AnotherI8(i8),
            Collection(Vec<String>),
        }
        impl JustATrait for EnumOfOtherTypes {}

        struct StructOfOtherTypes {
            an_i8: i8,
            another_i8: i8,
            a_collection: Vec<String>,
        }
        impl JustATrait for StructOfOtherTypes {}

        struct ArrayAndI8 {
            array: [i8; 1000], // This one will be very large
            an_i8: i8,
            in_u8: u8,
        }
        impl JustATrait for ArrayAndI8 {}

        println!(
            "{}, {}, {}, {}, {}",  // 2, 3, 32, 32, 1002
            size_of::<EnumOfNumbers>(),
            size_of::<StructOfNumbers>(),
            size_of::<EnumOfOtherTypes>(),
            size_of::<StructOfOtherTypes>(),
            size_of::<ArrayAndI8>(),  
        );

        // 如下函数，返回的JustATrait的size可以是2, 3, 32, 32, 1002，的任意一种，所以在Rust中不允许，所以需要box包装trait
        // fn returns_just_a_trait() -> JustATrait {
        //     let some_enum = EnumOfNumbers::I8(8);
        //     some_enum
        // }

        fn returns_just_a_trait() -> Box<dyn JustATrait> {  // 因为在栈上只是一个Box，我们知道Box的大小， Box<dyn Error>这种形式，因为有时你可能会有多个可能的错误
            let some_enum = EnumOfNumbers::I8(8);
            Box::new(some_enum)
        }
    }

    #[test]
    fn test_error_dyn() {
        use std::error::Error;
        use std::fmt;

        #[derive(Debug)]
        struct ErrorOne;

        impl Error for ErrorOne {} // Now it is an error type with Debug. Time for Display:

        impl fmt::Display for ErrorOne {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "You got the first error!") // All it does is write this message
            }
        }


        #[derive(Debug)] // Do the same thing with ErrorTwo
        struct ErrorTwo;

        impl Error for ErrorTwo {}

        impl fmt::Display for ErrorTwo {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "You got the second error!")
            }
        }

        // Make a function that just returns a String or an error
        fn returns_errors(input: u8) -> Result<String, Box<dyn Error>> { // With Box<dyn Error> you can return anything that has the Error trait
            match input {
                0 => Err(Box::new(ErrorOne)), // Don't forget to put it in a box
                1 => Err(Box::new(ErrorTwo)),
                _ => Ok("Looks fine to me".to_string()), // This is the success type
            }

        }

        let vec_of_u8s = vec![0_u8, 1, 80]; // Three numbers to try out

        for number in vec_of_u8s {
            match returns_errors(number) {
                Ok(input) => println!("{}", input),
                Err(message) => println!("{}", message),
            }
        }

    }
}
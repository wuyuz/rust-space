
#[macro_use]
mod macros {
    macro_rules! say_hello {
        () => {
            println!("Hello");
            say_world!()
        }
    }

    macro_rules! say_world {
        () => {
            println!("world")
        };
    }
}
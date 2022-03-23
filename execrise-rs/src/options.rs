#[warn(unused_variables)]
// Option 不是一个结构体，它是一个带有两个变量的 enum
// Result也是一个枚举(enum),它可以包含某物或者是一个错误:
enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    fn unwrap(self) -> T {
        // enums variants can be used in patterns:
        match self {
            Self::Some(t) => t,
            Self::None => panic!(".unwrap() called on a None option"),
        }
    }
}

// mod test {
//     #[test]
//     fn t_option() {
//         let o1: Option<i32> = Some(128);
//         o1.unwrap(); // this is fine
//
//         let o2: Option<i32> = None;  // 如果为None的话，调用unwrap就会报错
//         o2.unwrap(); // this panics!
//     }
//
//     #[test]
//     fn  t_my_option() {
//         use super::Option::{self,None,Some};
//
//         let o1: Option<i32> = Some(128);
//         o1.unwrap(); // this is fine
//
//         let o2: Option<i32> = None;
//         o2.unwrap(); // this panics! 自定义报错
//     }
//
// }
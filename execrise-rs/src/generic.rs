#[warn(unused_variables)]
struct Pair<T> {
    a: T,
    b: T,
}

// 泛型函数可以被认为是命名空间，包含了无数的具体类型的函数。可以使用::符号来引用里面针对具体类型的函数
fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());  // 这种方式被亲切地称为turbofish语法(turbofish syntax),因为::<>看起来像一条鱼。
}

mod test {
    use super::*;

    #[test]
    fn t_generic() {
        let p1 = Pair { a: 3, b: 9 };
        let p2 = Pair { a: true, b: false };
        print_type_name(&p1); // prints "Pair<i32>"
        print_type_name(&p2); // prints "Pair<bool>"
    }
}
#[warn(unused_variables)]
struct Number {
    value : i32,
    odd: bool
}

impl std::ops::Neg for Number {
    type Output = Self;
    
    fn neg(self) -> Self {
        Self {
            value : -self.value,
            odd: self.odd,
        }
    }
}

impl std::clone::Clone for Number {
    fn clone(&self) -> Self {
        Self{..*self}
    }
}

fn invert(n: &mut Number) {
    n.value = -n.value;
}

fn print_number(n: &Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}


mod test {
    
    #[test]
    fn t_main() {
        use super::*;
        // 实现负数
        let n = Number { odd: true, value: 987 };
        let m = -n; // 因为我们实现了`Neg` trait,这里才是行得通的。
        println!("{}", m.value); // 打印输出: "-987"


        let a: i32 = 15;  // 因为i32实现了Copy，所以可以copy赋值，而不被move
        let b = a; // `a` 被拷贝
        let c = a; // `a` 再次被拷贝

        let n = Number { odd: true, value: 51 };
        let m = n; // `n` 被移动到 `m`
        // let o = n; // 错误: 使用以及被移动过的值: `n` 

        let n = Number { odd: true, value: 51 };  // 通过引用可以进行多次借用
        print_number(&n); // `n` is borrowed for the time of the call
        print_number(&n); // `n` is borrowed again

        let mut n = Number { odd: true, value: 51 };
        print_number(&n); // odd number 51
        invert(&mut n); // `n`被可变借用 - 一切都是显式的
        print_number(&n);  // odd number -51

        // 使用traits实现copy
        let n = Number { odd: true, value: 51 };
        let mut m = n.clone();  // 需要实现了std::clone::Clone
        m.value += 100;
        
        print_number(&n); // odd number 51
        print_number(&m); // odd number 151
    }
}
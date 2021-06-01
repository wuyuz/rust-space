#[warn(unused_variables)]
struct Number {
    value: i32,
}

fn number_value<'a>(num: &'a Number) -> &'a i32 {
    &num.value
}

struct NumRef<'a> {
    x: &'a i32,
}

fn as_num_ref<'a>(x: &'a i32) -> NumRef<'a> {
    NumRef { x: &x }
}

fn as_num_refs(x: &i32) -> NumRef<'_> { // 省略了生命周期
    NumRef { x: &x }
}

impl<'a> NumRef<'a> {  // impl实现范型， 可以省略'a
    fn as_i32_ref(&'a self) -> &'a i32 {  // 可以省略'a，因为是一样的生命周期入参和出参
        self.x
    }
}

// static： 有一个特殊的生命周期叫'static',它的意思是在整个程序的运行期间都是有效的。字符串字面量(String literals)就是'static'
struct Person {
    name: &'static str,
}

mod test {
    use super::*;

    #[test]
    fn life_one(){
        // `x` doesn't exist yet
        {
            let x = 42; // `x` starts existing
            println!("x = {}", x);
            // `x` stops existing

            let y = 42; // `y` starts existing
            let y_ref = &y; // `y_ref` starts existing - it borrows `y`
            println!("y_ref = {}", y_ref);
            // `y_ref` stops existing
            // `y` stops existing
        }
        // `y` no longer exists
    }

    /*
    #[test]
    fn life_ref_out() {
        let x_ref = {
            let x = 42;
            &x  // error: no long enough
        };
        println!("x_ref = {}", x_ref);
        // error: `x` does not live long enough
    }
    */

    #[test]
    fn bound_multi() {
        // 1.一个变量绑定可以进行多次不可变借用:
        let x = 42;
        let x_ref1 = &x;
        let x_ref2 = &x;
        let x_ref3 = &x;
        println!("{} {} {}", x_ref1, x_ref2, x_ref3);

        // 2.当变量绑定被借用的时候，是不能被修改的:
        // let mut x = 42;
        // let x_ref = &x;  // 被借用
        // x = 13;   // 修改报错
        // println!("x_ref = {}", x_ref);

        // 3.当一个变量(绑定)被不可变借用(引用)的时候，就不能再被可变借用(引用):
        // let mut x = 42;
        // let x_ref1 = &x;
        // let x_ref2 = &mut x;
        // // error: cannot borrow `x` as mutable because it is also borrowed as immutable
        // println!("x_ref1 = {}", x_ref1);

        // 4.带有引用参数的函数可以被拥有不同生命周期的借用(借用)来调用,所以:
        //      4.1 所有带有引用参数的函数都是泛型的
        //      4.2 生命周期是泛型参数
        //      4.3 这就允许函数返回引用,返回的引用的生命周期依赖于参数的生命周期
        let n = Number { value: 47 };
        let v = number_value(&n);
        println!("{}",v);
    }

    #[test]
    fn diff_def() {
        // 当只有单个输入生命周期的时候，可以不进行命名(译者注:就就是可以省略不写)，函数里所有的一切都有相同的生命周期，所以下面的两个函数是等价的
        fn number_value<'a>(num: &'a Number) -> &'a i32 {
            &num.value
        }
        
        fn number_values(num: &Number) -> &i32 {
            &num.value
        }
    }

    #[test]
    fn struct_generic() {
        let x: i32 = 99;
        let x_ref = as_num_ref(&x); // 传入
        let x_ref_init = NumRef { x: &x };
        // `x_ref` cannot outlive `x`, etc.
        let x_num_ref = as_num_refs(&x);
        let x_i32_ref = x_num_ref.as_i32_ref();

    }

    #[test]
    fn t_static() {
        let p = Person {
            name: "fasterthanlime",
        };

        let name = format!("fasterthan{}", "lime");
        //let p = Person { name: &name };  // error: requires that `name` is borrowed for `'static`
        /*
            局部的name不是一个'static str',它是一个String。它是被动态分配然后将会被释放。它的生命周期要比整个程序的生命周期短(尽管它在main函数里)。
        为了在Person里存储一个非-'static'的字符串,可以采取下面两种方式:
            1. 写成关于生命周期的泛型： name: &'a str,
            2. 或者写成：name: String,
        */
    }
}
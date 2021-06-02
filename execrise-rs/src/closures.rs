
// 闭包(Closures)只是能够捕获上下文(context)的Fn，FnMut或者FnOnce类型的函数。
// 它们的参数是在一对竖线(|)里用逗号隔开的名称列表。它们不需要大括号(curly braces)，除非你想写多个声明(statement)。
fn for_each_planet<F>(f: F)
   where F: Fn(&'static str)
{
   f("Earth");
   f("Mars");
   f("Jupiter");
}

// FnMut 和 Fn的区别在于，一个FnMut被调用的时候需要是可变借用,所以它在使用的时候只能被调用一次
fn foobar<F>(f: F)  // 
   where F: Fn(i32) -> i32
{
   println!("{}", f(f(2)));   // f 函数可以被重复调用
}

fn foobar_mut<F>(mut f: F)
   where F: FnMut(i32) -> i32
{
   // println!("{}", f(f(2)));   // 不能多次调用，相当于之前传入的f函数变成了f(2)
   // error: cannot borrow `f` as mutable more than once at a time

   let tmp = f(2);  // 这种是合法的
   println!("{}", f(tmp)); 
}

// FnMut会存在是因为一些闭包会可变借用局部变量(local variabl)
fn foobar_mut_var<F>(mut f: F)
   where F: FnMut(i32) -> i32  // f函数中包含了可变变量
{
   let tmp = f(2);
   println!("{}", f(tmp)); 
}

mod test {
    use super::*;
    
    #[test]
    fn clo_1() {
        for_each_planet(|p| println!("Hello, {}",p)); // 将|p| println!("Hello, {}",p)当成函数传入for_each_plant中，p为参数
    }

    #[test]
    fn mut_fn() {
        foobar(|x| x*2 );
        foobar_mut (|x| x*2);

        // 可变变量
        let mut acc = 2;
        foobar_mut_var(|x| {
            acc += 1; // f有可变变量
            x * acc
        });
    }
}



// 闭包(Closures)只是能够捕获上下文(context)的Fn，FnMut或者FnOnce类型的函数。
// 它们的参数是在一对竖线(|)里用逗号隔开的名称列表。它们不需要大括号(curly braces)，除非你想写多个声明(statement)。
fn for_each_planet<F>(f: F)
   where F: Fn(&'static str)
{
   f("Earth");
   f("Mars");
   f("Jupiter");
}


mod test {
    use super::*;
    
    #[test]
    fn clo_1() {
        for_each_planet(|p| println!("Hello, {}",p));
    }
}

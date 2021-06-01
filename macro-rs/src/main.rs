/*
Rust语法包含数种“语法扩展”的形式：
    1. #[derive(Clone)]
    2. #![allow(dead_code)]

    3. println!("Hi!")
    4. macro_rules! dummy { () => {}; }
    前面两种属于属性，用于扩展语法(比如用于要求兼容C的ABI的#[repr(C)])以及语法扩展(比如 #[derive(Clone)]), 后面我们主要讲解后面两种
    */
// 自定义宏
macro_rules! vec_str {
    (            
        $($element:expr),  // $是字面标记, ( ... ) 代表了将要被重复匹配的模式，由小括号包围。
        *    // rep是重复控制标记。当前有两种选择，分别是* (代表接受0或多次重复)以及+ (代表1或多次重复)
    ) => {
       {
            // 为了能包含多条语句，
            // 我们将扩展部分包裹在花括号中...
           let mut v = Vec::new();

           // 重复开始：
           $(
               // 每次重复将包含如下元素，其中
                // “$element”将被替换成其相应的展开...
               v.push(format!("{}", $element));
           )*

           // 最后返回值
           v
       } 
    };
}

// 宏的展开
/*
    换句话讲，宏调用所在的位置，决定了该宏展开之后的结果被解读的方式。编译器将把AST中表示宏调用的节点用其宏展开的输出节点完全替换。
这一替换是结构性(structural)的，而非织构性(textural)的。
 */
macro_rules! capture_then_match_tokens {  // 本质上来说是AST语法书的输出
    ($e:expr) => {match_tokens!($e)};
}
macro_rules! match_tokens {
    ($a:tt + $b:tt) => {"got an addition"};
    (($i:ident)) => {"got an identifier"};
    ($($other:tt)*) => {"got something else"};
}

// 宏规则
/*
macro_rules! $name {  // 宏语法
    $rule0 ;    // 每条规则都形如：($pattern) => {$expansion}
    $rule1 ; 
    // …
    $ruleN ;
}
*/

// 匹配：当一个宏被调用时，对应的macro_rules解释器将一一依序检查规则。对每条规则，它都将尝试将输入标记树的内容与该规则的pattern进行匹配。
// 某个模式必须与输入完全匹配才能被选中为匹配项。
macro_rules! three {
    () => {
        println!("I been found!")
    };
}

// 捕获：宏模式中还可以包含捕获。这允许输入匹配在某种通用语法基础上进行，并使得结果被捕获进某个变量中。此变量可在输出中被替换使用。
/*
    捕获由$符号紧跟一个标识符(identifier)紧跟一个冒号(:)紧跟捕获种类组成。捕获种类须是如下之一：
        item: 条目，比如函数、结构体、模组等。
        block: 区块(即由花括号包起的一些语句加上/或是一项表达式)。
        stmt: 语句
        pat: 模式
        expr: 表达式
        ty: 类型
        ident: 标识符
        path: 路径 (例如 foo, ::std::mem::replace, transmute::<_, int>, …)
        meta: 元条目，即被包含在 #[...]及#![...]属性内的东西。
        tt: 标记树
 */
 macro_rules! multiply_add {
     ($a:expr,$b:expr, $c:expr) => {
         $a * ($b + $c)
     };
 }

 // 重复
 /*
    模式中可以包含重复。这使得匹配标记序列成为可能。重复的一般形式为 $(...) sep rep 可用于匹配和结果中
 */
macro_rules! rep_show {
    (
        $($e: expr)
        ,
        *
    ) => {
        $(
            println!("{}",$e);  
        )*   // 计算过程中没有sep分割
    };
}

// 卫生性
// 当宏涉及到生命周期、范型等情况时可能就不是卫生性的了，之所以能做到“卫生”，在于每个标识符都被赋予了一个看不见的“句法上下文”
/*
    macro_rules! using_a {
        ($e:expr) => {
            {
                let a = 42;  // 这里无法解析出a变量的值，因为没有上下文
                $e
            }
        }
    }
    let four = using_a!(a / 10);  // a无法解析
 */
 macro_rules! using_a {
     ($a:ident, $e:expr) => {
         {
            let $a = 42;
            $e
         }
     }
 }

// self的特殊性，同时时标识符、关键词
macro_rules! double_method {
    ($self_:ident, $body:expr) => {
        fn double(mut $self_) -> Dummy {
            $body
        }
    };
}
struct Dummy(i32);
impl Dummy {
    double_method! {self, {
        self.0 *= 2;
        self
    }}
}

#[macro_use]
mod lib;

fn main() {
    // 自定义
    let c = vec_str![1,"e","eerwre"];
    println!("{:?}",c);

    // 展开
    println!("{}\n{}\n{}\n",
        match_tokens!((caravan)),
        match_tokens!(3 + 6),
        match_tokens!(5));
    println!("{}\n{}\n{}",
        capture_then_match_tokens!((caravan)),  // 和上述的输出本上不一致，语法tree输出
        capture_then_match_tokens!(3 + 6),
        capture_then_match_tokens!(5));

    // 匹配
    three!();

    // 捕获
    println!("{}",multiply_add!(3,2,1)); // 9

    // 重复
    rep_show!(1,3,"xxx");

    // 卫生性
    println!("{}",using_a!(a,a/10));  // 4

    // self
    println!("{:?}", Dummy(4).double().0); //8


    // 导入/导出
    say_hello!();  // 导入\导出的时候使用macro_use

}


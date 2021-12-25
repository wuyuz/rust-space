
use proc_macro_define_crate::my_test_proc_macro;

#[my_test_proc_macro(xx(::ss::ee))]
pub fn foo(a:i32) {
    println!("hello alex")
}


fn main() {
    println!("Hello, world!");
}

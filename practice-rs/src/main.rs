use tokio::io::{self, AsyncWriteExt as _};

// use proc_macro_define_crate::my_test_proc_macro;
use proc_macro_define_crate::Httper;

// #[my_test_proc_macro(xx(::ss::ee))]
// fn foo(a:i32){
//     println!("hello, blog.ideawand.com, hello, 极客幼稚园");
// }

#[derive(Httper)]
struct HttpHandler {
    url: String,
}

impl HttpHandler {
    fn new(u: String) -> HttpHandler {
        HttpHandler{
            url: u,
        }
    }
}

#[tokio::main]
async fn main() {
    // foo(3);
    let r= HttpHandler::new("http://httpbin.org/ip".to_string());
    r.get().await;
    println!("Hello, world!");
}



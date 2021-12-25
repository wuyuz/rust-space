fn countdown<F>(count: usize, tick: F)
   where F: Fn(usize)
{
   for i in (1..=count).rev() {
       tick(i);
   }
}

mod test {
    use super::*;
    
    #[test]
    fn iterable_t() {
        countdown(3, |i| println!("tick {}...", i));

        for i in vec![52, 49, 21] {
            println!("I like the number {}", i);
        }
    }

    #[test]
    fn str_te() {
        let truth = "Rust 学习";
        let ptr = truth.as_ptr();
        let len = truth.len();
        let s = unsafe {
            let slice = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8(slice)
        };
        assert_eq!(s,Ok(truth))
    }
}

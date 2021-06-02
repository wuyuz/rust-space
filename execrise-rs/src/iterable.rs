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
}
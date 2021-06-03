

mod test {
    use std::time::Instant;

    trait OddTrait {
        fn is_odd(&self, n: u64) -> bool;
    }

    struct OddOne;

    impl OddTrait for OddOne {
        fn is_odd(&self, n:u64) -> bool {
            n % 2 == 1
        }
    }

    fn find_nth_odd(finder: &dyn OddTrait, n: u64) ->u64 {
        let mut i = 0;
        let mut count = 0;

        while count != n {
            i += 1;
            if finder.is_odd(i) {
                count += 1
            }
        }
        count
    } 

    fn find_nth_odd_box(finder: Box<dyn OddTrait>, n:u64) -> u64 {
        let mut i = 0;
        let mut count = 0;
        while count != n {
            i += 1;
            if finder.is_odd(i) {
                count += 1
            }
        }
        count
    }

    
    #[test]
    fn t_find_odd() {
        let start = Instant::now();
        let finder = OddOne;
        println!("count: {}, cost:{}",find_nth_odd(&finder, 1_000_000_000), start.elapsed().as_millis());

        let start = Instant::now();
        let finder_box :Box<OddOne> = Box::new(OddOne);
        println!("box count: {}, cost:{}",find_nth_odd_box(finder_box, 1_000_000_000), start.elapsed().as_millis());

    }

}



mod test {

    #[test]
    fn test_rc() {
        use std::rc::Rc;

        #[derive(Debug)]
        struct City {
            name : String,
            population: u32,
            city_history: Rc<String>
        }

        #[derive(Debug)]
        struct CityData {
            names: Vec<String>,
            histories: Vec<Rc<String>>, // A Vec of Strings inside Rcs
        }

        let calgary = City {
            name: "Calgary".to_string(),
            population: 1_200_000,
               // Pretend that this string is very very long
            city_history: Rc::new("Calgary began as a fort called Fort Calgary that...".to_string()), // Rc::new() to make the Rc
        };
    
        let canada_cities = CityData {
            names: vec![calgary.name],
            histories: vec![calgary.city_history.clone()], // .clone() to increase the count
        };
    
        println!("Calgary's history is: {}", calgary.city_history);
        println!("{}", Rc::strong_count(&calgary.city_history));
        let new_owner = calgary.city_history.clone();
        println!("{}", Rc::strong_count(&calgary.city_history)); // 3
    }

    /*
     * 那么，如果有强指针，是否有弱指针呢？是的，有。弱指针是有用的，因为如果两个Rc互相指向对方，它们就不会死。这就是所谓的 "引用循环"。
     * 如果第1项对第2项有一个Rc，而第2项对第1项有一个Rc， 它们不能到0，在这种情况下，要使用弱引用。那么Rc就会对引用进行计数，但如果只
     * 有弱引用，那么它就会死掉。你使用Rc::downgrade(&item),而不是Rc::clone(&item)来创建弱引用。另外，需要用Rc::weak_count(&item)来查看弱引用数。
     */
}
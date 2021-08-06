
mod test {
    #[test]
    fn mut_fn() {
        /*
         * 先来看 mut a : T和mut a : &T的区别，这个应该比较简明，即前者中a是T类型变量的可变绑定，后者中a是T类型不可变引用的可变绑定。
         */

        struct FullName{
            first_name:String,
            last_name:String,
        }
        
        // mut a:&T
        let mut a = &FullName {
            first_name: String::from("Jobs"),
            last_name: String::from("Steve"),
        };

        //a重新绑定到一个新的FullName的引用
        a = &FullName {
            first_name: String::from("Gates"),
            last_name: String::from("Bill"),
        };   //不允许对a指向的内容作出修改
       
        //a.first_name = String::from("Error");
        // println!("{}:{}",a.last_name, a.first_name);

        // 2. a:&mut T
        let a = &mut FullName {
            first_name: String::from("Jobs"),
            last_name: String::from("Steve"),
        };
        //a不允许重新绑定到一个新的FullName的引用
        // a = &FullName {
        //     first_name: String::from("Gates"),
        //     last_name: String::from("Bill"),
        // };
        //允许对a指向的内容作出修改
        a.first_name = String::from("Gates");
        println!("{}:{}",a.last_name, a.first_name);
    }


    #[test]
    fn mut_var() {
        let mut c = 2;
        let mut closure = || {
            c += 1;
            println!("in the closure, c= {}",c);
        };
        closure();
        println!("out of the closure, c={}",c);
    }
}
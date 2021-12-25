

// Rust有很多方法可以让一些不可变的东西里面有一些安全的可变性，最简单的方法叫做Cell。
// 首先我们使用use std::cell::Cell，这样我们就可以每次只写Cell而不是std::cell::Cell。

mod test{

    #[test]
    fn cell_test_1() {
        #[derive(Debug)]
        struct PhoneModel {
            company_name: String,
            model_name: String,
            screen_size: f32,
            memory: usize,
            date_issued: u32,
            on_sale: bool,
        }


        let mut super_phone_3000 = PhoneModel {  // 手动设置为可变变量，但是我们只针对某一个字段进行改变
            company_name: "YY Electronics".to_string(),
            model_name: "Super Phone 3000".to_string(),
            screen_size: 7.5,
            memory: 4_000_000,
            date_issued: 2020,
            on_sale: true,
        };

        super_phone_3000.on_sale = false;
        println!("{:?}", super_phone_3000);
    }

    #[test]
    fn cell_test_2() {
        use std::cell::Cell;
        struct PhoneModel {
            company_name: String,
            on_sale: Cell<bool>  // 修改可变的变量
        }

        let super_phone_3 = PhoneModel{
            company_name:"YY Electronics".to_string(),
            on_sale:Cell::new(true)
        };
        super_phone_3.on_sale.set(false) // Cell 适用于所有类型，但对简单的 Copy 类型效果最好，因为它给出的是值，而不是引用。Cell有一个叫做get()的方法，它只对Copy类型有效。
        // 另一个可以使用的类型是 RefCell。
    }


    // RefCell是另一种无需声明mut而改变值的方法。它的意思是 "引用单元格"，就像 Cell，但使用引用而不是副本。
    #[test]
    fn recell_test(){
        use std::cell::RefCell;
        use std::cell::RefMut;

        #[derive(Debug)]
        struct User {
            id: u32,
            username: String,
            active: RefCell<bool>
        }

        let user1 = User{
            id:1,
            username:"User 1".into(),
            active: RefCell::new(true)
        };
        println!("{:?}", user1);  // User { id: 1, username: "User 1", active: RefCell { value: true } }
        // RefCell的方法有很多。其中两种是.borrow()和.borrow_mut()。使用这些方法，你可以做与&和&mut相同的事情。规则都是一样的:
            // 多个不可变借用可以
            // 一个可变的借用可以
            // 但可变和不可变借用在一起是不行的

        user1.active.replace(false);  // User { id: 1, username: "User 1", active: RefCell { value: false } }
        println!("{:?}", user1); 
        {  
            let mut borrow_one = user1.active.borrow_mut();
            *borrow_one = true;
            println!("{:?},{:?}", user1.active, borrow_one);  // 同一个作用域会显示borrowed，RefCell { value: <borrowed> },true
        }


        println!("{:?}", user1.active);  //RefCell { value: true }
    }

}

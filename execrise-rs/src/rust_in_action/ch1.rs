

#[test]
fn greet_world() {
    println!("Hello world!");
    let southern_germany = "Gott";
    let japan="xxx";
    let regions = [southern_germany, japan];

    for &region in regions.iter() {
        println!("{}", region);
    }
}

#[test]
fn count_world() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data";

    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i==0 ||record.trim().len() ==0 {
            continue;
        }

        let fields: Vec<_>  = record
        .split(',')
        .map(|f|f.trim())
        .collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length)  = fields[1].parse::<f32>() {
            println!("{}, {}cm", name,length);
        }
    }
}

#[test]
fn multiple() {
    use std::rc::Rc;
    use std::sync::{Arc,Mutex};

    let a = 10;
    let b= Box::new(20);  // integer on heap, also known as a box integer
    let c = Rc::new(Box::new(30));  // Boxed integer wrapped within a reference counter
    let d = Arc::new(Mutex::new(40)); //Integer wrapped in an atomic reference counter protected by a mutual exlusion lock
}

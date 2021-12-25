use std::{io::BufRead, ptr::read};

use regex::Regex;



#[test]
fn for_iter() {
    let mut str_list = ["sxx","2313","wo"];
    for item in str_list.iter_mut() {
        *item = "xxx";
    }
    println!("{:?}",str_list);

    let a =(2,4,5);
    println!("{}",a.0);


    for (x,y) in (0..).zip(0..) {  // 打散
        if x+y > 100 {
            break;
        }
        println!("{},{}",x,y)
    }
}

fn process_lines<T: BufRead + Sized>(reader:T, re:Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_)=> println!("{}", line),
            None => (),
        }
    }
}

#[test]
fn stdin_() {
    use std::fs::File;
    use std::{io,io::BufReader,io::prelude::*};
    use regex::Regex;
    use clap::{App,Arg};

    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(true))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader =stdin.lock();
        process_lines(reader, re);

    }else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader,re);
    }
}


#[test]
fn access_data() {
    use std::rc::Rc;
    use std::cell::RefCell;

    
}
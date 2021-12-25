
#[warn(unused_variables)]
mod clone;
mod  generic;
mod options;
mod lifetime;
mod slice;
mod closures;
mod iterable;
mod dyn_trait;
mod build;
mod mut_a;
mod test_rt;
mod cell;
mod mutex;
mod Rc;
mod thread;
mod arc;
mod channel;
mod boxs;

mod patterns;
mod rust_in_action;
mod algorithm;

// use std::{io::BufRead, ptr::read};

// use regex::Regex;
// fn process_lines<T: BufRead + Sized>(reader:T, re:Regex) {
//     for line_ in reader.lines() {
//         let line = line_.unwrap();
//         match re.find(&line) {
//             Some(_)=> println!("{}", line),
//             None => (),
//         }
//     }
// }

// fn main() {
//     use std::fs::File;
//     use std::{io,io::BufReader,io::prelude::*};
//     use regex::Regex;
//     use clap::{App,Arg};

//     let args = App::new("grep-lite")
//         .version("0.1")
//         .about("searches for patterns")
//         .arg(Arg::with_name("pattern")
//             .help("The pattern to search for")
//             .takes_value(true)
//             .required(true))
//         .arg(Arg::with_name("input")
//             .help("File to search")
//             .takes_value(true)
//             .required(true))
//         .get_matches();

//     let pattern = args.value_of("pattern").unwrap();
//     let re = Regex::new(pattern).unwrap();

//     let input = args.value_of("input").unwrap_or("-");

//     if input == "-" {
//         let stdin = io::stdin();
//         let reader =stdin.lock();
//         process_lines(reader, re);

//     }else {
//         let f = File::open(input).unwrap();
//         let reader = BufReader::new(f);
//         process_lines(reader,re);
//     }
// }

use std::collections::HashMap;

fn main()  {
    let v = vec!["flower".to_owned(),"fliss".to_owned(),"flo".to_owned()];
    println!("{:?}",t(v));
}

fn t(strs: Vec<String>) -> String {
    let mut res = Vec::new();

    let tup:Vec<_> =  strs.iter().map(|item| item.as_bytes()).collect();
    'o: for i in 0..tup[0].len(){
        for j in 0..tup.len() {
            if i >= tup[j].len() || tup[0][i] != tup[j][i] {
                res = tup[0][0..i].to_vec();
                break 'o
            }
        }
        res = tup[0].to_vec();
    }
    String::from_utf8(res).unwrap()
}   


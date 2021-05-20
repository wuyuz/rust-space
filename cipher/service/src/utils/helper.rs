use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::iter;


//  随机字符
pub fn rand_6_int() -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(9)
        .collect()
}
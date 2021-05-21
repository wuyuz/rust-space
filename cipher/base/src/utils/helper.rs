use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

pub fn rand_6_int(n:u8) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(n as usize)
            .collect()
}


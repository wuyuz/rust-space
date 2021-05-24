use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rand_core::{RngCore, OsRng};

pub fn rand_6_int<'a>(n:u8) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(n as usize)
            .collect()
}

pub fn salt_send() -> [u8;10] {
    let mut salt = [0u8; 10];
    OsRng.fill_bytes(&mut salt);
    salt
}
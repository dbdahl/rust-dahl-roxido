#![allow(dead_code)]

extern crate rand;

use rand::Rng;
use rand::SeedableRng;
use rand_isaac::IsaacRng;
use std::slice;

pub unsafe fn mk_rng<T: Rng, S>(seed_ptr: *const i32, f: S) -> T
where T: Rng,
      S: Fn([u8;32]) -> T
{
    let seed_slice = slice::from_raw_parts(seed_ptr, 32);
    let mut seed = [0u8; 32];
    for i in 0..seed.len() {
        seed[i] = seed_slice[i] as u8;
    }
    f(seed)
}

pub unsafe fn mk_rng_isaac(seed_ptr: *const i32) -> IsaacRng {
    mk_rng(seed_ptr, IsaacRng::from_seed)
}

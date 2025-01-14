use std::time::Instant;
use std::str::FromStr;
use bigfloppa::BigDecimal;

extern crate bigfloppa;

fn main() {
    //   let mut x = BigDecimal::from(1.1);
    let mut x = BigDecimal::from_str("1.1").unwrap();
    // for iter in 0..1_000_000 {
    for iter in 0..26 {
        let start = Instant::now();
        x = x.clone() * x;
        // x = x.take_and_square();
        let end = Instant::now();
        let usage = end - start;
        println!(
            "iter {} takes {} secs",
            iter,
            usage.as_secs() as f32 + usage.subsec_nanos() as f32 / 1.0e9
        );
    }
}

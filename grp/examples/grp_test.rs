use grp::*;
use std::env;
use num_bigint::BigUint;

fn main() {

    let demands = env::args().skip(1).map(|arg| arg.parse::<BigUint>().unwrap()).collect::<Vec<_>>();

    if !demands.is_empty() {
        let ltr_cnt = demands[0].clone();
        let sbl_cnt = demands[1].clone();
        let num_cnt = demands[2].clone();

        println!("\n{}\n", RandomPassword::new(ltr_cnt, sbl_cnt, num_cnt).show());
    } else { println!("\n{}\n", RandomPassword::new(10, 2, 3).show()); }

}

use grp::*;
use std::env;
use num_bigint::BigUint;




fn main() {

    let demands = env::args().skip(1).map(|arg| arg.parse::<BigUint>().unwrap()).collect::<Vec<_>>();

    let mut r_p;

    if demands.is_empty() {
        r_p = RandomPassword::new(10, 2, 3);
        r_p.mix();
        println!("\n{}\n", r_p.show());
    }
    else {
        let ltr_cnt = demands[0].clone();
        let sbl_cnt = demands[1].clone();
        let num_cnt = demands[2].clone();

        r_p = RandomPassword::new(ltr_cnt, sbl_cnt, num_cnt);
        r_p.mix();
        println!("\n{}\n", r_p.show());
    }

}

use std::env;
use grp::RandomPassword;



fn main() {

    let requirement = env::args()
        .skip(1)
        .map(|arg| arg.parse::<usize>().expect("Should be positive"))
        .collect::<Vec<usize>>(); // get the user's input and parse them into numbers which should be positive

    if !requirement.is_empty() {
        let (length, sbl_cnt, num_cnt) = (requirement[0], requirement[1], requirement[2]);
        if length >= sbl_cnt + num_cnt { // check if the numbers is legal or not in logic
            let mut r_p = RandomPassword::new(length, sbl_cnt, num_cnt);
            println!("{}", r_p.show());
        } else { println!("Invalid input!!"); }
    } else { println!("{}", RandomPassword::new(12, 1, 3).show()); } // Default
}
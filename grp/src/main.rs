use std::env;
use grp_api::GEN_PWD;

fn main() {
    let length; // length of the password
    let symbols_cnt; // amount of the symbols
    let numbers_cnt; // amount of the numbers

    let requirement =
        env::args()
            .skip(1)
            .map(|arg| arg.parse::<usize>().expect("Should be positive"))
            .collect::<Vec<usize>>(); // get the user's input and parse them into numbers which should be positive

    if !requirement.is_empty() {
        if requirement[0] >= requirement[1] + requirement[2] { // check if the numbers is legal or not in logic
            length = requirement[0];
            symbols_cnt = requirement[1];
            numbers_cnt = requirement[2];
            println!("{}", GEN_PWD(length-symbols_cnt-numbers_cnt, symbols_cnt, numbers_cnt));
        } else { println!("Invalid input!!"); }
    } else { println!("{}", GEN_PWD(12, 1, 3)); } // Default
}
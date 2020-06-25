use std::env;
use api_g_r_d::GEN_PWD;


fn main() {
    let length; // length of the password
    let symbol; // amount of the symbols
    let number; // amount of the numbers

    let demands = env::args()
        .skip(1)
        .map(|arg| arg.parse::<usize>().expect("Should be positive"))
        .collect::<Vec<usize>>(); // get the user's input and parse them into numbers which should be positive

    if !demands.is_empty() {
        if demands[0] >= demands[1] + demands[2] { // check if the numbers is legal or not in logic
            length = demands[0];
            symbol = demands[1];
            number = demands[2];
            let password = GEN_PWD(length-symbol-number, symbol, number);
            println!("{}", password);
        } else { println!("Invalid input!!"); }
    } else { println!("Should receive three arguments!"); }
}
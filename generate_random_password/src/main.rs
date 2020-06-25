use std::env;
use api_g_r_d::GEN_PWD;


fn main() {
    let length; // length of the password
    let symbol; // amount of the symbols
    let number; // amount of the numbers

    let requirement =
        env::args()
            .skip(1)
            .map(|arg| arg.parse::<usize>().expect("Should be positive"))
            .collect::<Vec<usize>>(); // get the user's input and parse them into numbers which should be positive

    if !requirement.is_empty() {
        if requirement[0] >= requirement[1] + requirement[2] { // check if the numbers is legal or not in logic
            length = requirement[0];
            symbol = requirement[1];
            number = requirement[2];
            let password = GEN_PWD(length-symbol-number, symbol, number);
            println!("{}", password);
        } else { println!("Invalid input!!"); }
    } else { println!("Should receive three arguments!"); }
}
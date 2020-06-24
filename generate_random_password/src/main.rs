use std::env;
use rand::Rng;
use rand::prelude::*;

const GEN: fn(u8, u8) -> Vec<String> = |start, end| {
        (start..=end).collect::<Vec<u8>>()
                     .iter()
                     .map(|asc_num| (*asc_num as char).to_string())
                     .collect::<Vec<String>>()
}; // return the ASCII characters in Vec we need

const RAND_IDX: fn(usize, usize) -> Vec<usize> = |n, cnt| { // Given the amount and upper bound, generate the random index
    let mut rng = rand::thread_rng();
    let mut idx;
    let mut idxs = vec![];
    for _ in 0..n {
        idx = rng.gen_range(0, cnt);
        idxs.push(idx);
    }
    idxs
};

fn main() {

    let length; // length of the password
    let symbol; // amount of the symbols
    let number; // amount of the numbers

    let symbols =
        vec![GEN(33, 47), GEN(58, 64), GEN(91, 96), GEN(123, 126)]
            .iter_mut()
            .fold(vec![], |mut acc, x| {acc.append(x); acc});

    let letters =
        vec![GEN(65, 90), GEN(97, 122)]
            .iter_mut()
            .fold(vec![], |mut acc, x| {acc.append(x); acc});

    let numbers = GEN(48, 57);    // include all single numbers

    let demands = env::args()
                          .skip(1)
                          .map(|arg| arg.parse::<usize>().expect("Should be positive"))
                          .collect::<Vec<usize>>(); // get the user's input and parse them into numbers which should be positive

    if !demands.is_empty() {
        if demands[0] >= demands[1] + demands[2] { // check if the numbers is legal in logic
            length = demands[0];
            symbol = demands[1];
            number = demands[2];
            let gen_pwd = |l, s, n| {
                // l: amount of letters [A-Z,a-z], say length - symbol - number
                // s: amount of symbols, say symbol
                // n: amount of numbers [0-9], say number

                let mut rng = rand::thread_rng();
                let mut password =
                    vec![
                        (l, letters.len(), letters),
                        (s, symbols.len(), symbols),
                        (n, numbers.len(), numbers),
                        ]
                        .iter()
                        .map(|args| {
                            RAND_IDX(args.0, args.1) // generate the random index on corresponding Vec depend on its amount
                                .iter()
                                .map(|idx| args.2[*idx].clone())// index their values in to Vec<String>
                                .collect()
                            })
                        .fold(vec![], |mut acc, mut x| {acc.append(&mut x);acc});
                        // unfold these Vec<Vec<String>> in to Vec<String>
                password.shuffle(&mut rng);
                println!("{:?}", password.join(""));
            };
            gen_pwd(length-symbol-number, symbol, number);
        } else {
            println!("Invalid input!!");
        }
    } else {
        println!("Should receive three arguments!");
    }
}
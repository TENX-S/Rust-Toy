#![allow(unused)]

use std::env;
use rand::Rng;
use rand::prelude::*;

const GEN: fn(u8, u8) -> Vec<String> = |start, end| {
        (start..=end).collect::<Vec<u8>>()
                     .iter()
                     .map(|asc_num| (*asc_num as char).to_string())
                     .collect::<Vec<String>>()
};

const CHECK: fn(&mut Vec<usize>) -> bool = |v| {

    let (length, symbol, number) = (v[0], v[1], v[2]);

    return
        symbol + number > length
            &&
        vec![symbol, number, length]
            .iter()
            .filter(|x| **x < 0)
            .collect::<Vec<&usize>>()
            .is_empty()
};

const RAND_IDX: fn(usize, usize) -> Vec<usize> = |n, cnt| {
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

    let mut length: usize = 16;
    let mut symbol: usize = 4;
    let mut number: usize = 4;
    
    let mut symbols_buffer = GEN(33, 47);
    symbols_buffer.append(&mut GEN(58, 64));
    symbols_buffer.append(&mut GEN(91, 96));
    symbols_buffer.append(&mut GEN(123, 126));

    let mut letters_buffer = GEN(65, 90);
    letters_buffer.append(&mut GEN(97, 122));

    let letters = letters_buffer;
    let symbols = symbols_buffer;
    let numbers = GEN(48, 57);

    let mut demands = env::args()
                          .skip(1)
                          .map(|arg| arg.parse::<usize>().unwrap())
                          .collect::<Vec<usize>>();
    if !demands.is_empty() {
        if CHECK(&mut demands) {
            length = demands[0];
            symbol = demands[1];
            number = demands[2];
        } else {
            println!("Invalid input!!");
        }
    }

    let gen_pwd = |l: usize, s: usize, n: usize| {

        let mut rng = rand::thread_rng();

        let mut password =
            vec![
                (l, letters.len(), letters),
                (s, symbols.len(), symbols),
                (n, numbers.len(), numbers),
                ]
                .iter()
                .map(|args| {
                    RAND_IDX(args.0, args.1)
                            .iter()
                            .map(|idx| args.2[*idx].clone())
                            .collect::<Vec<String>>()
                    })
                .fold(vec![], |mut acc, mut x| {acc.append(&mut x);acc});
        password.shuffle(&mut rng);
        println!("{:?}", password.join(""));
    };
    gen_pwd(length-symbol-number, symbol, number);
}
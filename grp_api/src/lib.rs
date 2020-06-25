#![allow(non_snake_case)]
use rand::prelude::*;

const _GEN: fn(Vec<(u8, u8)>) -> Vec<String> = |range_list| {
    let mut all = vec![];
    for (start, end) in range_list {
        let mut v = (start..=end)
            .collect::<Vec<u8>>()
            .iter()
            .map(|asc_num| (*asc_num as char).to_string())
            .collect();
        all.append(&mut v);
    }
    all
};

const _RAND_IDX: fn(usize, usize) -> Vec<usize> = |n, cnt| { // Given the amount and upper bound, generate the random index
    let mut rng = rand::thread_rng();
    let mut idx;
    let mut idxs = vec![];
    for _ in 0..n {
        idx = rng.gen_range(0, cnt);
        idxs.push(idx);
    }
    idxs
};

const _DATA: fn() -> (Vec<String>, Vec<String>, Vec<String>) = || {
    let letters: Vec<String> = _GEN(vec![(65, 90), (97, 122)]);
    let symbols: Vec<String> = _GEN(vec![(33, 47), (58, 64), (91, 96), (123, 126)]);
    let numbers: Vec<String> = _GEN(vec![(48, 57)]);

    (letters, symbols, numbers)
};

pub const GEN_PWD: fn(usize, usize, usize) -> String = |l, s, n| {
        // l: amount of letters [A-Z,a-z], say length - symbol - number
        // s: amount of symbols, say symbol
        // n: amount of numbers [0-9], say number

        let mut rng = rand::thread_rng();
        let data = _DATA();
        let mut PWD =
        vec![
            (l, data.0),
            (s, data.1),
            (n, data.2),
            ]
            .iter()
            .map(|args| {
                _RAND_IDX(args.0, args.1.len()) // generate the random index on corresponding Vec depend on its amount
                    .iter()
                    .map(|idx| args.1[*idx].clone())// index their values in to Vec<String>
                    .collect()
            })
            .fold(vec![], |mut acc, mut x| { acc.append(&mut x); acc });
            // unfold these Vec<Vec<String>> in to Vec<String>
            PWD.shuffle(&mut rng);
            PWD.join("")
};
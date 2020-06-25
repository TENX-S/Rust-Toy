use rand::prelude::*;

const _GEN: fn(u8, u8) -> Vec<String> = |start, end| {
    (start..=end).collect::<Vec<u8>>()
        .iter()
        .map(|asc_num| (*asc_num as char).to_string())
        .collect::<Vec<String>>()
}; // return the ASCII characters in Vec we need

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
    let symbols: Vec<String> =
        vec![_GEN(33, 47), _GEN(58, 64), _GEN(91, 96), _GEN(123, 126)]
            .iter_mut()
            .fold(vec![], |mut acc, x| {acc.append(x); acc});

    let letters: Vec<String> =
        vec![_GEN(65, 90), _GEN(97, 122)]
            .iter_mut()
            .fold(vec![], |mut acc, x| {acc.append(x); acc});

    let numbers: Vec<String> = _GEN(48, 57);

    (symbols, letters, numbers)
};

pub const GEN_PWD: fn(usize, usize, usize) -> String = |l, s, n| {
        // l: amount of letters [A-Z,a-z], say length - symbol - number
        // s: amount of symbols, say symbol
        // n: amount of numbers [0-9], say number

        let mut rng = rand::thread_rng();
        let data = _DATA();
        let mut password =
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
            .fold(vec![], |mut acc, mut x| {acc.append(&mut x);acc});
            // unfold these Vec<Vec<String>> in to Vec<String>
            password.shuffle(&mut rng);
            password.join("")
};
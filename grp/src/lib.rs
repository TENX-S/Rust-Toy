#![allow(non_snake_case)]
use rand::prelude::*;

pub struct RandomPassword {
    length: usize,
    sbl_cnt: usize,
    num_cnt: usize,
    content: Option<String>,
}

impl RandomPassword {

    pub fn new(length: usize, sbl_cnt: usize, num_cnt: usize) -> Self {
        RandomPassword { length, sbl_cnt, num_cnt, content: None }
    }

    pub fn show(&mut self) -> &str {
        let mut rng = rand::thread_rng();
        let data = _DATA();
        let mut PWD = vec![(self.length-self.sbl_cnt-self.num_cnt, data.0), (self.sbl_cnt, data.1), (self.num_cnt, data.2),]
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
        self.content = Some(PWD.join(""));
        self.content.as_ref().expect("Should have a password here")
    }
}

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _GEN_works() {
        assert_eq!(_GEN(vec![(48, 57)]), vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);
        assert_eq!(_GEN(vec![(33, 47), (58, 64), (91, 96), (123, 126)]), vec!["!", "\"", "#", "$", "%", "&", "\'", "(", ")", "*", "+", ",", "-", ".", "/", ":", ";", "<", "=", ">", "?", "@", "[", "\\", "]", "^", "_", "`", "{", "|", "}", "~"]);
        assert_eq!(_GEN(vec![(65, 90), (97, 122)]), vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"]);
    }

    #[test]
    fn _RAND_IDX_works() {
        let ret = _RAND_IDX(10_000, 100_0000)
            .iter()
            .filter(|x| **x > 100_0000)
            .collect::<Vec<&usize>>()
            .is_empty();
        assert!(ret);
    }
}
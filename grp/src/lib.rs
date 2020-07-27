#![allow(non_snake_case)]

use rand::prelude::*;
pub use num_bigint::{ BigUint, ToBigUint };

pub struct RandomPassword
{

    length: BigUint,
    sbl_cnt: BigUint,
    num_cnt: BigUint,
    content: String,

}


impl RandomPassword
{

    /// Return an instance of RandomPassword
    pub fn new<T: ToBigUint>(length: T, sbl_cnt: T, num_cnt: T) -> Self

    {

        RandomPassword {

            length: length.to_biguint().unwrap(),
            sbl_cnt: sbl_cnt.to_biguint().unwrap(),
            num_cnt: num_cnt.to_biguint().unwrap(),
            content: String::new(),

        }

    }


    /// Return the string of random password
    ///
    /// # Example
    ///
    /// ```
    /// use crate::grp::*;
    /// let mut rp = RandomPassword::new(10, 2, 3);
    /// println!("{}", rp.show());
    /// ```
    ///
    pub fn show(&mut self) -> String
    {

        let mut rng = rand::thread_rng();
        let data = Self::_DATA();
        let mut PWD =
            vec!
            [
                (self.length.clone()-self.sbl_cnt.clone()-self.num_cnt.clone(), data.0),
                (self.sbl_cnt.clone(), data.1),
                (self.num_cnt.clone(), data.2)
            ]
            .iter()
            .map(|args|
                {   // generate the random index on corresponding Vec depend on its amount
                    Self::_RAND_IDX(args.0.clone(), args.1.len())
                        .iter()
                        .map(|idx| args.1[*idx].clone())// index their values in to Vec<String>
                        .collect()
                })
            .collect::<Vec<Vec<_>>>()
            .concat();
            // or
            //.fold(vec![], |mut acc, mut x| { acc.append(&mut x); acc });

        PWD.shuffle(&mut rng);
        self.content = PWD.join("");

        self.content.clone()

    }


    /// Generate n random numbers up to cnt
    /// # Example
    ///
    /// ```
    /// use crate::grp::*;
    /// let random_indexs = _RAND_IDX(5, 10);
    /// println!("{:?}", random_indexs);
    /// ```
    ///
    #[inline]
    fn _RAND_IDX(mut n: BigUint, cnt: usize) -> Vec<usize>
    {

        let mut rng = rand::thread_rng();
        let mut idx;
        let mut idxs = vec![];

        while n != 0.to_biguint().unwrap()
        {
            idx = rng.gen_range(0, cnt);
            idxs.push(idx);
            n -= 1.to_biguint().unwrap();
        }

        idxs

    }

    /// The character set needed to generate a random password
    #[inline]
    fn _GEN(range_list: Vec<(u8, u8)>) -> Vec<String>
    {

        let mut all = vec![];

        for (start, end) in range_list
        {
            let mut v = (start..=end).collect::<Vec<_>>()
                                                .iter()
                                                .map(|asc_num| (*asc_num as char).to_string())
                                                .collect();
            all.append(&mut v);
        }

        all

    }

    fn _DATA() -> (Vec<String>, Vec<String>, Vec<String>)
    {

        let letters: Vec<String> = Self::_GEN(vec![(65, 90), (97, 122)]);
        let symbols: Vec<String> = Self::_GEN(vec![(33, 47), (58, 64), (91, 96), (123, 126)]);
        let numbers: Vec<String> = Self::_GEN(vec![(48, 57)]);

        (letters, symbols, numbers)

    }
}

// const _GEN: fn(Vec<(u8, u8)>) -> Vec<String> = |range_list| {
//     let mut all = vec![];
//     for (start, end) in range_list {
//         let mut v = (start..=end)
//             .collect::<Vec<_>>()
//             .iter()
//             .map(|asc_num| (*asc_num as char).to_string())
//             .collect();
//         all.append(&mut v);
//     }
//     all
// };


// const _DATA: fn() -> (Vec<String>, Vec<String>, Vec<String>) = || {
//     let letters: Vec<String> = _GEN(vec![(65, 90), (97, 122)]);
//     let symbols: Vec<String> = _GEN(vec![(33, 47), (58, 64), (91, 96), (123, 126)]);
//     let numbers: Vec<String> = _GEN(vec![(48, 57)]);

//     (letters, symbols, numbers)
// };


// const _RAND_IDX: fn(usize, usize) -> Vec<usize> = |n, cnt| { // Given the amount and upper bound, generate the random index
//     let mut rng = rand::thread_rng();
//     let mut idx;
//     let mut idxs = vec![];
//     for _ in 0..n {
//         idx = rng.gen_range(0, cnt);
//         idxs.push(idx);
//     }
//     idxs
// };


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _GEN_works()
    {

        assert_eq!(RandomPassword::_GEN(vec![(48, 57)]), vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);
        assert_eq!(RandomPassword::_GEN(vec![(33, 47), (58, 64), (91, 96), (123, 126)]), vec!["!", "\"", "#", "$", "%", "&", "\'", "(", ")", "*", "+", ",", "-", ".", "/", ":", ";", "<", "=", ">", "?", "@", "[", "\\", "]", "^", "_", "`", "{", "|", "}", "~"]);
        assert_eq!(RandomPassword::_GEN(vec![(65, 90), (97, 122)]), vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"]);

    }


    #[test]
    fn _RAND_IDX_works()
    {

        let ret = RandomPassword::_RAND_IDX(10_000.to_biguint().unwrap(), 100_0000)
                                      .iter()
                                      .filter(|x| **x > 100_0000)
                                      .collect::<Vec<&usize>>()
                                      .is_empty();

        assert!(ret);

    }

}

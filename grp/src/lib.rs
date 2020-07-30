#![allow(non_snake_case)]

use std::ops::Add;
use rand::prelude::*;
use num_traits::{ Zero, One };
pub use num_bigint::{ BigUint, ToBigUint };


#[derive(Clone, Debug)]
pub struct RandomPassword
{
    length: BigUint,
    sbl_cnt: BigUint,
    num_cnt: BigUint,
    content: String,
}


impl RandomPassword
{

    /// Return an instance of `Result<RandomPassword>`
    /// # Example
    /// ```
    /// use grp::{RandomPassword, BigUint};
    /// let rp_1 = RandomPassword::new(11, 4, 2); // ok
    ///
    /// let rp_1_1 = RandomPassword::new(11.to_biguint().unwrap(), 4.to_biguint().unwrap(), 2.to_biguint().unwrap());
    /// // It works too, but not recommended
    ///
    /// // If you want push a large number in it
    /// // parse the `&str` into `Biguint`
    /// let length = format!("{}000", usize::MAX).parse::<BigUint>().unwrap();
    /// let sbl_cnt = format!("{}00", usize::MAX).parse::<BigUint>().unwrap();
    /// let num_cnt = format!("{}0", usize::MAX).parse::<BigUint>().unwrap();
    /// let rp_1_2 = RandomPassword::new(length, sbl_cnt, num_cnt);
    ///
    /// let rp_2 = RandomPassword::new(-1, 0, 0);
    /// assert_eq!(rp_2.err(), Err("`length`, `sbl_cnt` and `num_cnt` should all be positive"));
    ///
    /// let rp_3 = RandomPassword::new(3, 3, 3);
    /// assert_eq!(rp_3.err(), Err("`length` should be greater than or equal to `sbl_cnt` plus `num_cnt`"));
    /// ```
    ///
    #[inline]
    pub fn new<T>(length: T, sbl_cnt: T, num_cnt: T) -> Result<Self, &'static str>
        where T: ToBigUint + Add<Output=T> + PartialOrd + Clone {
        let l = length.to_biguint();
        let s = sbl_cnt.to_biguint();
        let n = num_cnt.to_biguint();

        if !l.is_none() && !s.is_none() && !n.is_none() {
            let l = l.unwrap();
            let s = s.unwrap();
            let n = n.unwrap();

            if l.clone() >= s.clone() + n.clone() {
                Ok(RandomPassword {
                        length: l,
                        sbl_cnt: s,
                        num_cnt: n,
                        content: String::new(),
                })
            } else {
                Err("`length` should be greater than or equal to `sbl_cnt` plus `num_cnt`")
            }
        } else {
            Err("`length`, `sbl_cnt` and `num_cnt` should all be positive")
        }
    }


    /// Return the string of random password
    ///
    /// # Example
    ///
    /// ```
    /// let mut rp = RandomPassword::new(10, 2, 3);
    /// println!("{}", rp.unwrap().show());
    /// // Output: +*yz952SwG
    /// ```
    ///
    pub fn show(&mut self) -> String {

        let mut rng = rand::thread_rng();
        let data = Self::_DATA();
        let mut PWD =
            vec![
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
    /// let random_indexs = _RAND_IDX(5.to_biguint().unwrap(), 10);
    /// println!("{:?}", random_indexs);
    /// // Output: [9, 0, 5, 8, 6]
    /// ```
    ///
    #[inline]
    fn _RAND_IDX(mut n: BigUint, cnt: usize) -> Vec<usize> {

        let mut rng = rand::thread_rng();
        let mut idx;
        let mut idxs = vec![];


        while n != BigUint::zero() {
            idx = rng.gen_range(0, cnt);
            idxs.push(idx);
            n -= BigUint::one();
        }

        idxs

    }

    /// The character set needed to generate a random password
    #[inline]
    fn _GEN(range_list: Vec<(u8, u8)>) -> Vec<String> {

        let mut all = vec![];

        for (start, end) in range_list {
            let mut v = (start..=end)
                        .collect::<Vec<_>>()
                        .iter()
                        .map(|asc_num| (*asc_num as char).to_string())
                        .collect();

            all.append(&mut v);
        }

        all

    }

    #[inline]
    fn _DATA() -> (Vec<String>, Vec<String>, Vec<String>) {

        let letters: Vec<String> = Self::_GEN(vec![(65, 90), (97, 122)]);
        let symbols: Vec<String> = Self::_GEN(vec![(33, 47), (58, 64), (91, 96), (123, 126)]);
        let numbers: Vec<String> = Self::_GEN(vec![(48, 57)]);

        (letters, symbols, numbers)

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _GEN_works() {

        assert_eq!(RandomPassword::_GEN(vec![(48, 57)]), vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);
        assert_eq!(RandomPassword::_GEN(vec![(33, 47), (58, 64), (91, 96), (123, 126)]), vec!["!", "\"", "#", "$", "%", "&", "\'", "(", ")", "*", "+", ",", "-", ".", "/", ":", ";", "<", "=", ">", "?", "@", "[", "\\", "]", "^", "_", "`", "{", "|", "}", "~"]);
        assert_eq!(RandomPassword::_GEN(vec![(65, 90), (97, 122)]), vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"]);

    }


    #[test]
    fn _RAND_IDX_works() {

        let ret = RandomPassword::_RAND_IDX(10_000.to_biguint().unwrap(), 100_0000)
                                      .iter()
                                      .filter(|x| **x > 100_0000)
                                      .collect::<Vec<&usize>>()
                                      .is_empty();

        assert!(ret);

    }

    #[test]
    fn constructor_works() {
        let rp1 = RandomPassword::new(12, 1, 1);
        assert!(rp1.is_ok());

        let rp2 = RandomPassword::new(-1, 1, 1);
        assert!(rp2.is_err());

        let rp3 = RandomPassword::new(2, 2, 2);
        assert!(rp3.is_err());
    }

}

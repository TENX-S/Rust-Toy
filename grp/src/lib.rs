#![allow(non_snake_case)]


use rand::prelude::*;
use rayon::prelude::*;
use num_traits::{ Zero, One, ToPrimitive };
use std::{ fmt::Display, ops::{ Add, SubAssign } };

pub use num_bigint::{ BigUint, ToBigUint };

/// struct `RandomPassword`
#[derive(Clone, Debug)]
pub struct RandomPassword {
    length: BigUint,
    sbl_cnt: BigUint,
    num_cnt: BigUint,
    content: String,
}


impl RandomPassword {

    /// Return an instance of `Result<RandomPassword, &'static str>`
    /// # Example
    /// ```
    /// use grp::{RandomPassword, BigUint};
    /// let rp_1 = RandomPassword::new(11, 4, 2)?; // ok
    ///
    /// let rp_1_1 = RandomPassword::new(11.to_biguint().unwrap(), 4.to_biguint().unwrap(), 2.to_biguint().unwrap());
    /// // It works too, but not recommended
    ///
    /// // If you want push a large number in it
    /// // parse the `&str` into `Biguint`
    /// let length = format!("{}000", usize::MAX).parse::<BigUint>()?;
    /// // or
    /// // use std::str::FromStr;
    /// // let length = BigUint::from_str(format!("{}000", usize::MAX))?;
    /// let sbl_cnt = format!("{}00", usize::MAX).parse::<BigUint>()?;
    /// let num_cnt = format!("{}0", usize::MAX).parse::<BigUint>()?;
    /// let rp_1_2 = RandomPassword::new(length, sbl_cnt, num_cnt)?;
    ///
    /// let rp_2 = RandomPassword::new(-1, 0, 0)?;
    /// assert_eq!(rp_2, Err("`length`, `sbl_cnt` and `num_cnt` should all be positive"));
    ///
    /// let rp_3 = RandomPassword::new(3, 3, 3)?;
    /// assert_eq!(rp_3, Err("`length` should be greater than or equal to `sbl_cnt` plus `num_cnt`"));
    /// ```
    ///
    #[inline]
    pub fn new<T>(length: T, sbl_cnt: T, num_cnt: T) -> Result<Self, &'static str>
        where T: ToBigUint + Add<Output=T> + PartialOrd + Clone
    {

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
    /// let mut rp = RandomPassword::new(10, 2, 3)?;
    /// println!("{}", rp.show());
    /// // Output: +*yz952SwG
    /// ```
    ///
    #[inline]
    pub fn show(&mut self) -> String {

        let data = Self::_DATA();

        let mut PWD: String =
                vec![
                    (self.length.clone()-self.sbl_cnt.clone()-self.num_cnt.clone(), data.0),
                    (self.sbl_cnt.clone(), data.1),
                    (self.num_cnt.clone(), data.2)
                ]
                .iter()
                .map(|(bignum, data)| {
                    Self::_DIV_UNIT((*bignum).clone())
                        .par_iter()
                        .map(|cnt| {
                            Self::_RAND_IDX(*cnt, data.len())
                                .par_iter()
                                .map(|idx| data[*idx].clone())
                                .collect::<String>()
                        })
                            .collect()
                })
                    .collect::<Vec<Vec<_>>>()
                    .concat()
                    .join("");

        let mut rng = thread_rng();

        let bytes;
        unsafe {
            bytes = PWD.as_bytes_mut();
        }

        bytes.shuffle(&mut rng);
        let ret = bytes.par_iter().map(|s| *s as char).collect::<String>();

        self.content = ret;

        self.content.clone()

    }


    /// Decompose large numbers into smaller numbers to use more CPU
    #[inline]
    fn _DIV_UNIT<T>(n: T) -> Vec<usize>
        where T: ToBigUint + Add<Output=T> + SubAssign + PartialOrd + Clone + Display
    {

        let mut n = n.to_biguint().unwrap();

        // The value of UNIT is inversely proportional to memory overhead
        // In order to increase CPU time and reduce the memory overhead, raise the value of `UNIT`
        let UNIT = i8::MAX.to_biguint().unwrap();

        let mut ret = Vec::new();

        loop {

            if n < UNIT.clone() {
                ret.push(n.to_usize().unwrap());
                break;
            } else {
                n -= UNIT.clone();
                ret.push(i8::MAX as usize);
            }
        }

        ret

    }


    /// Generate n random numbers up to cnt
    /// # Example
    ///
    /// ```
    /// let random_indexs = _RAND_IDX(5, 10);
    /// println!("{:?}", random_indexs);
    /// // Output: [9, 0, 5, 8, 6]
    /// ```
    ///
    #[inline]
    fn _RAND_IDX(n: impl ToBigUint, cnt: usize) -> Vec<usize> {

        let mut n = n.to_biguint().unwrap();
        let mut rng = thread_rng();
        let mut idx;
        let mut idxs = Vec::new();


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

        let mut all = Vec::new();

        for (start, end) in range_list {
            let mut v = (start..=end).collect::<Vec<_>>()
                                     .iter()
                                     .map(|asc_num| (*asc_num as char).to_string())
                                     .collect();

            all.append(&mut v);
        }

        all

    }

    /// Range of character set
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
    use num_traits::ToPrimitive;

    #[test]
    fn _GEN_works() {

        assert_eq!(RandomPassword::_GEN(vec![(48, 57)]), vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);
        assert_eq!(RandomPassword::_GEN(vec![(33, 47), (58, 64), (91, 96), (123, 126)]), vec!["!", "\"", "#", "$", "%", "&", "\'", "(", ")", "*", "+", ",", "-", ".", "/", ":", ";", "<", "=", ">", "?", "@", "[", "\\", "]", "^", "_", "`", "{", "|", "}", "~"]);
        assert_eq!(RandomPassword::_GEN(vec![(65, 90), (97, 122)]), vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"]);

        let a = RandomPassword::_GEN(vec![(33, 47), (58, 64), (91, 96), (123, 126)]);
        let mut sum = 0;
        for sbl in &a {
            sum += sbl.len();
        }
        println!("{}", sum);
        println!("{}", a.len());

    }


    #[test]
    fn _RAND_IDX_works() {

        let ret = RandomPassword::_RAND_IDX(10_000.to_biguint().unwrap(), 100_0000)
                                 .into_iter()
                                 .filter(|x| *x > 100_0000)
                                 .collect::<Vec<_>>()
                                 .is_empty();

        assert!(ret);

    }

    #[test]
    fn constructor_works() {

        let rp0 = RandomPassword::new(0, 0, 0);
        assert!(rp0.is_ok());

        let rp1 = RandomPassword::new(12, 1, 1);
        assert!(rp1.is_ok());

        let rp2 = RandomPassword::new(-1, 1, 1);
        assert!(rp2.is_err());

        let rp3 = RandomPassword::new(2, 2, 2);
        assert!(rp3.is_err());

    }

    #[test]
    fn _DIV_UNIT_works() {

        let mut bignum = 2000.to_biguint().unwrap().to_usize().unwrap();

        println!("{} divide into {:?}", bignum, RandomPassword::_DIV_UNIT(bignum));

        let smallnum = 0;

        println!("{} divide into {:?}", smallnum, RandomPassword::_DIV_UNIT(smallnum));

    }

}

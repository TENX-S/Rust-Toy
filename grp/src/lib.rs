#![allow(non_snake_case)]

use rand::prelude::*;
use rayon::prelude::*;
use std::ops::SubAssign;
use num_bigint::{ BigUint, ToBigUint };
use num_traits::{ Zero, One, ToPrimitive };


/// struct `RandomPassword`
#[derive(Clone, Debug)]
pub struct RandomPassword {
    ltr_cnt: BigUint,
    sbl_cnt: BigUint,
    num_cnt: BigUint,
    content: String,
    _UNIT: usize,
}


impl RandomPassword {

    /// Return an empty instance of `Result<RandomPassword, &'static str>`
    /// # Example
    /// ```
    /// use grp::{ RandomPassword, BigUint };
    /// let r_p = RandomPassword::new(11, 4, 2);
    /// // If you want push a large number in it
    /// // parse the `&str` into `Biguint`
    /// use std::str::FromStr;
    /// let ltr_cnt = BigUint::from_str(format!("{}000", usize::MAX));
    /// let sbl_cnt = BigUint::from_str(format!("{}000", usize::MAX));
    /// let num_cnt = BigUint::from_str(format!("{}000", usize::MAX));
    /// r_p = RandomPassword::new(ltr_cnt, sbl_cnt, num_cnt);
    /// ```
    #[inline]
    pub fn new<T: ToBigUint>(ltr_cnt: T, sbl_cnt: T, num_cnt: T) -> Self {

        RandomPassword {
            ltr_cnt: ltr_cnt.to_biguint().unwrap(),
            sbl_cnt: sbl_cnt.to_biguint().unwrap(),
            num_cnt: num_cnt.to_biguint().unwrap(),
            content: String::new(),
            _UNIT: 1
        }

    }


    /// Return the string of random password
    /// # Example
    ///
    /// ```
    /// let mut rp = RandomPassword::new(10, 2, 3);
    /// println!("{}", rp.show());
    /// // Output: +*yz952SwG
    /// ```
    #[inline]
    pub fn show(&mut self) -> String {

        let data = Self::_DATA();
        let mut PWD: String = self._PWD((self.ltr_cnt.clone(), data.0),
                                        (self.sbl_cnt.clone(), data.1),
                                        (self.num_cnt.clone(), data.2),);
        let bytes = unsafe { PWD.as_bytes_mut() };
        bytes.shuffle(&mut thread_rng());
        self.content = bytes.par_iter().map(|s| *s as char).collect::<String>();

        self.content.clone()

    }

    /// Returns the length of this `RandomPassword`, in both bytes and [char]s.
    #[inline]
    pub fn len(&self) -> usize { self.content.len() }

    /// Returns true if this `RandomPassword` has a length of zero, and false otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool { self.content.is_empty() }

    /// The value of UNIT is inversely proportional to memory overhead
    /// In order to increase CPU time and reduce the memory overhead, raise the value of `UNIT`
    #[inline]
    pub fn set_unit(&mut self, val: usize) { self._UNIT = val; }

    /// Generate random password
    #[inline]
    fn _PWD<T>(&self, letters: (T, Vec<String>), symbols: (T, Vec<String>), numbers: (T, Vec<String>)) -> String
        where T: ToBigUint + Clone + SubAssign + PartialOrd
    {

        vec![(letters.0, letters.1),
             (symbols.0, symbols.1),
             (numbers.0, numbers.1),]
            .iter()
            .map(|(bignum, data)| {
                self._DIV_UNIT((*bignum).clone())
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
            .join("")

    }

    /// Decompose large numbers into smaller numbers
    #[inline]
    fn _DIV_UNIT<T>(&self, n: T) -> Vec<usize>
        where T: ToBigUint + SubAssign + PartialOrd + Clone
    {

        let mut n = n.to_biguint().unwrap();

        let UNIT = self._UNIT.to_biguint().unwrap();
        let mut ret = Vec::with_capacity((n.clone() / UNIT.clone() + BigUint::one()).to_usize().unwrap());

        loop {
            if n < UNIT.clone() {
                ret.push(n.to_usize().unwrap());
                break;
            } else {
                n -= UNIT.clone();
                ret.push(self._UNIT);
            }
        }

        ret

    }


    /// Generate n random numbers up to cnt
    /// # Example
    /// ```
    /// let random_indexs = _RAND_IDX(5, 10);
    /// println!("{:?}", random_indexs);
    /// // Output: [9, 0, 5, 8, 6]
    /// ```
    #[inline]
    fn _RAND_IDX(n: impl ToBigUint, cnt: usize) -> Vec<usize> {

        let mut idx;
        let mut n = n.to_biguint().unwrap();
        let mut idx_s = Vec::with_capacity(n.to_usize().unwrap());

        while n != BigUint::zero() {
            idx = thread_rng().gen_range(0, cnt);
            idx_s.push(idx);
            n -= BigUint::one();
        }

        idx_s

    }

    /// The character set needed to generate a random password
    #[inline]
    fn _GEN(range_list: Vec<(u8, u8)>) -> Vec<String> {

        range_list
            .into_iter()
            .map(|(start, end)|
                (start..=end)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|asc_num|
                        (asc_num as char).to_string()
                    )
                    .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>()
            .concat()

    }

    /// Range of character set
    /// return (letters, symbols, numbers)
    #[inline]
    fn _DATA() -> (Vec<String>, Vec<String>, Vec<String>) {

        (
            Self::_GEN(vec![(65, 90), (97, 122)]),
            Self::_GEN(vec![(33, 47), (58, 64), (91, 96), (123, 126)]),
            Self::_GEN(vec![(48, 57)])
        )

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
        assert!(RandomPassword::_RAND_IDX(10_000.to_biguint().unwrap(), 100_0000)
                               .into_iter()
                               .filter(|x| *x > 100_0000)
                               .collect::<Vec<_>>()
                               .is_empty());
    }

}

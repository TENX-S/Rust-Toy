#![allow(non_snake_case)]
#![feature(trait_alias)]


#[macro_use]
extern crate lazy_static;

mod prelude;
use prelude::*;


// TODO: implement a specifc version for smaller number
/// struct `RandPwd`
#[derive(Clone, Debug)]
pub struct RandPwd {
    ltr_cnt: BigUint,
    sbl_cnt: BigUint,
    num_cnt: BigUint,
    content: String, // TODO: - use the heapless String
    _UNIT: usize,    // TODO: - implement a smart _UNIT initialization
}


impl RandPwd {

    /// Return an empty instance of `Result<RandPwd, &'static str>`
    /// # Example
    /// ```
    /// use grp::RandPwd;
    /// use num_bigint::BigUint;
    /// let mut r_p = RandPwd::new(11, 4, 2);
    ///
    /// // If you want push a large number in it
    /// // parse the `&str` into `BigUint`
    /// use std::str::FromStr;
    ///
    /// let ltr_cnt = BigUint::from_str(&format!("{}000", usize::MAX)).unwrap();
    /// let sbl_cnt = BigUint::from_str(&format!("{}000", usize::MAX)).unwrap();
    /// let num_cnt = BigUint::from_str(&format!("{}000", usize::MAX)).unwrap();
    ///
    /// r_p = RandPwd::new(ltr_cnt, sbl_cnt, num_cnt);
    ///
    /// // You can also mix the `BigUint` with primitive type
    /// ```
    #[inline]
    pub fn new<L, S, N>(ltr_cnt: L, sbl_cnt: S, num_cnt: N) -> Self
    where L: ToBigUint,
          S: ToBigUint,
          N: ToBigUint,
    {

        RandPwd {
            ltr_cnt: ltr_cnt.to_biguint().unwrap(),
            sbl_cnt: sbl_cnt.to_biguint().unwrap(),
            num_cnt: num_cnt.to_biguint().unwrap(),
            content: String::new(),
            _UNIT: 1
        }

    }


    /// Generate the password
    #[inline]
    pub fn join(&mut self) {
        let data = &DATA;
        let mut PWD: String = self._PWD((&self.ltr_cnt, &data[0]),
                                        (&self.sbl_cnt, &data[1]),
                                        (&self.num_cnt, &data[2]),);
        // This is absolutely safe, because they are all ASCII characters except control ones.
        let bytes = unsafe { PWD.as_bytes_mut() };
        bytes.shuffle(&mut thread_rng());
        self.content = bytes.par_iter().map(|s| *s as char).collect::<String>();
    }


    /// Return the content of random password in `&str`
    /// # Example
    ///
    /// ```
    /// use grp::RandPwd;
    /// let mut rp = RandPwd::new(10, 2, 3);
    /// rp.join();
    /// println!("{}", rp.show());
    /// // Output: 0fajn-ulS8S}7sn
    /// ```
    #[inline]
    pub fn show(&self) -> &str {
        &self.content
    }


    /// Change the content of `RandPwd`
    #[inline]
    pub fn set_content(&mut self, val: &str) {
        self.content = val.to_string();
    }


    /// Returns the length of this `RandPwd`, in both bytes and [char]s.
    #[inline]
    pub fn len(&self) -> usize {
        self.content.len()
    }


    /// Returns true if this `RandPwd` has a length of zero, and false otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }


    /// The value of UNIT is inversely proportional to memory overhead
    /// In order to reduce the memory overhead, raise the value of `UNIT`
    #[inline]
    pub fn set_unit(&mut self, val: usize) {
        self._UNIT = val;
    }


    /// Change the count of letters, symbols or numbers of `RandPwd`
    /// ```
    /// use grp::*;
    /// let mut r_p = RandPwd::new(10, 2, 3);
    ///
    /// // Set the letter's count
    /// r_p.set_cnt("ltr", 0);
    /// r_p.join();
    /// println!("{}", r_p.show());
    /// // Output: *029(
    ///
    /// // Set the symbol's count
    /// r_p.set_cnt("sbl", 0);
    /// r_p.join();
    /// println!("{}", r_p.show());
    /// // Output: nz1MriAl0j5on
    ///
    /// // Set the number's count
    /// r_p.set_cnt("num", 0);
    /// r_p.join();
    /// println!("{}", r_p.show());
    /// // Output: +iQiQGSXl(nv
    /// ```
    #[inline]
    pub fn set_cnt<T: ToBigUint>(&mut self, kind: &str, val: T) -> Option<()> {
        match kind {

            "ltr" => self.ltr_cnt = val.to_biguint()?,
            "sbl" => self.sbl_cnt = val.to_biguint()?,
            "num" => self.num_cnt = val.to_biguint()?,

            _     => (),
        }
        Some(())
    }


    /// Get count of `RandPwd`
    /// ```
    /// use grp::RandPwd;
    /// use num_traits::ToPrimitive;
    /// let r_p = RandPwd::new(10, 2, 3);
    /// assert_eq!(r_p.get_cnt("ltr").unwrap().to_usize().unwrap(), 10);
    /// assert_eq!(r_p.get_cnt("sbl").unwrap().to_usize().unwrap(), 2);
    /// assert_eq!(r_p.get_cnt("num").unwrap().to_usize().unwrap(), 3);
    /// ```
    #[inline]
    pub fn get_cnt(&self, kind: &str) -> Option<&BigUint> {
        match kind {
            "ltr" => Some(&self.ltr_cnt),
            "sbl" => Some(&self.sbl_cnt),
            "num" => Some(&self.num_cnt),

              _   => None,
        }
    }


    /// Generate random password
    #[inline]
    pub(crate) fn _PWD<'a, T: P>(&self, ltr: I<'a, T>, sbl: I<'a, T>, num: I<'a, T>) -> String {
        // TODO: - Improve readability
        vec![(ltr.0, ltr.1),
             (sbl.0, sbl.1),
             (num.0, num.1),]
            .iter()
            .map(|(bignum, data)| {
                self._DIV_UNIT(*bignum)
                    .par_iter()
                    .map(|cnt| {
                        _RAND_IDX(*cnt, data.len())
                            .par_iter()
                            // TODO: - Remove this `clone` which can cause huge overhead of both memory and CPU
                            .map(|idx| data[*idx].clone())
                            .collect::<String>()
                    })
                    .collect()
            })
            .collect::<Vec<Vec<_>>>()
            .concat()
            .join("")

    }


    /// Resolve large numbers into smaller numbers
    #[inline]
    pub(crate) fn _DIV_UNIT<T: P>(&self, n: &T) -> Vec<usize> {

        let mut n = n.to_biguint().unwrap();

        let UNIT = BigUint::from(self._UNIT);
        let mut ret = Vec::with_capacity((&n / &UNIT + BigUint::one()).to_usize().unwrap());

        loop {
            if n < UNIT {
                ret.push(n.to_usize().unwrap());
                break;
            } else {
                n -= UNIT.clone();
                ret.push(self._UNIT);
            }
        }

        ret

    }

}


impl Default for RandPwd {
    fn default() -> Self {
        RandPwd::new(0, 0, 0)
    }
}


impl Display for RandPwd {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\n{}", self.content)
    }
}


impl Add for RandPwd {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        RandPwd {
            ltr_cnt: self.ltr_cnt + rhs.ltr_cnt,
            sbl_cnt: self.sbl_cnt + rhs.sbl_cnt,
            num_cnt: self.num_cnt + rhs.num_cnt,
            content: self.content + &rhs.content,
            _UNIT: 1,
        }
    }
}


impl AddAssign for RandPwd {

    fn add_assign(&mut self, rhs: Self) {

        self.ltr_cnt += rhs.ltr_cnt;
        self.sbl_cnt += rhs.sbl_cnt;
        self.num_cnt += rhs.num_cnt;
        self.content += &rhs.content;

    }
}


impl AsRef<str> for RandPwd {

    fn as_ref(&self) -> &str {
        &self.content
    }

}


impl From<&str> for RandPwd {

    fn from(s: &str) -> Self {
        let (ltr_cnt, sbl_cnt, num_cnt) = _CNT(s);
        let mut r_p = RandPwd::new(ltr_cnt, sbl_cnt, num_cnt);
        r_p.set_content(s);
        r_p.set_unit(1);

        r_p
    }

}


pub trait ToRandPwd {
    fn to_randpwd(&self) -> Option<RandPwd>;
}

impl<T: AsRef<str>> ToRandPwd for T {

    #[inline]
    fn to_randpwd(&self) -> Option<RandPwd> {
        Some(RandPwd::from(self.as_ref()))
    }

}

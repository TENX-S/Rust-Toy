
pub use heapless;
pub use rand::prelude::*;
pub use rayon::prelude::*;
pub use typenum::{ U3, U52, };
pub use num_bigint::{ BigUint, ToBigUint };
pub use num_traits::{ Zero, One, ToPrimitive };
pub use std::{
    convert::From,
    ops::{ Add, SubAssign, AddAssign, },
    fmt::{ Display, Formatter, Result, },
};

/// Type alias for the parameter of method `_PWD`,
/// `T` represents the count of characters should be used,
/// `&[String]` represent the corresponding characters set
pub type I<'a, T> = (&'a T, &'a [String]);

pub type StrVec = heapless::Vec<String, U52>;
pub type CharVec = heapless::Vec<StrVec, U3>;

pub trait P = Clone + ToBigUint + SubAssign + PartialOrd;


lazy_static! {
    /// Cached the characters set
    pub static ref DATA: CharVec = _DATA();
}


/// Characters set
/// return letters, symbols, numbers in `CharVec`
pub(crate) fn _DATA() -> CharVec {

    let mut letters = StrVec::new();
    let mut symbols = StrVec::new();
    let mut numbers = StrVec::new();

    let mut charset = CharVec::new();

    let _ = (33..127)
            .into_iter()
            .map(|x| {
                let ch = x as u8 as char;
                if ch.is_ascii_alphabetic()  { letters.push(ch.to_string()).unwrap(); }
                if ch.is_ascii_punctuation() { numbers.push(ch.to_string()).unwrap(); }
                if ch.is_ascii_digit()       { symbols.push(ch.to_string()).unwrap(); }
            })
            .collect::<()>();

    charset.push(letters).unwrap();
    charset.push(symbols).unwrap();
    charset.push(numbers).unwrap();

    charset

}


/// Count the number of a string
#[inline]
pub(crate) fn _CNT<T: AsRef<str>>(content: T) -> (usize, usize, usize) {

    let mut l = 0;
    let mut s = 0;
    let mut n = 0;

    content.as_ref().chars().for_each(
        |x| {
            if x.is_ascii() {
                if x.is_ascii_alphabetic()  { l += 1; }
                if x.is_ascii_punctuation() { s += 1; }
                if x.is_ascii_digit()       { n += 1; }
            } else {
                panic!("Has non-ASCII character(s)!, the first one is: {:?}", x)
            }
        }
    );

    (l, s, n)

}

/// Generate n random numbers, each one is up to cnt
#[inline]
pub(crate) fn _RAND_IDX(n: impl ToBigUint, cnt: usize) -> Vec<usize> {

    let mut n = n.to_biguint().unwrap();
    let mut idxs = Vec::with_capacity(n.to_usize().unwrap());

    while !n.is_zero() {
        idxs.push(thread_rng().gen_range(0, cnt));
        n -= BigUint::one();
    }

    idxs

}

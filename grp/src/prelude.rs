

pub use heapless;
pub use rand::prelude::*;
pub use rayon::prelude::*;
pub use typenum::{ U3, U26, U52, };
pub use num_bigint::{ BigUint, ToBigUint };
pub use num_traits::{ Zero, One, ToPrimitive };
pub use std::{ ops::SubAssign, convert::From, fmt::{ Display, Formatter, Result }, };

/// Type alias for the parameter of method `_PWD`,
/// `T` represents the count of characters should be used,
/// `&[String]` represent the corresponding characters set
pub type I<'a, T> = (&'a T, &'a [String]);

pub type NumVec = heapless::Vec<u8, U26>;
pub type StrVec = heapless::Vec<String, U52>;
pub type CharVec = heapless::Vec<StrVec, U3>;

pub trait P = ToBigUint + Clone + SubAssign + PartialOrd;


lazy_static! {
    /// Cached the characters set
    pub static ref DATA: CharVec = _DATA();
}

/// Characters set
/// return letters, symbols, numbers in `CharVec`
pub(crate) fn _DATA() -> CharVec {
    let GEN = |range_list: &[(u8, u8)]|
        range_list
            .into_iter()
            .map(|(start, end)|
                (*start..=*end)
                    .collect::<NumVec>()
                    .into_iter()
                    .map(|asc_num|
                        (asc_num as char).to_string()
                    )
                    .collect::<StrVec>()
            )
            .fold(StrVec::new(), |mut acc, x| { acc.extend_from_slice(&x).unwrap(); acc });

    [&[(65, 90), (97, 122)][..],
        &[(33, 47), (58, 64), (91, 96), (123, 126)][..],
        &[(48, 57)][..],]
        .iter()
        .map(|x| GEN(x))
        .collect::<CharVec>()

}


/// Count the number of a string
pub(crate) fn cnt<T: AsRef<str>>(content: T) -> (usize, usize, usize) {

    let mut l = 0;
    let mut s = 0;
    let mut n = 0;


    // TODO: - Multi-threads
    content.as_ref().chars().for_each(
        |x| {
            if x.is_alphabetic() { l += 1; }
            else if x.is_numeric() { n += 1; }
            else { s += 1; }
        }
    );

    (l, s, n)

}

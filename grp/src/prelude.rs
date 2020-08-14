

pub use heapless;
pub use rand::prelude::*;
pub use rayon::prelude::*;
pub use typenum::{ U3, U26, U52, };
pub use num_bigint::{ BigUint, ToBigUint };
pub use num_traits::{ Zero, One, ToPrimitive };
pub use std::{ fmt::{ Display, Formatter, Result }, ops::SubAssign };

/// Type alias for the parameter of method `_PWD`,
/// `T` represents the count of characters should be used,
/// `&[String]` represent the corresponding characters set
pub type I<'a, T> = (&'a T, &'a [String]);

pub type NumSet = heapless::Vec<u8, U26>;
pub type StrSet = heapless::Vec<String, U52>;
pub type CharSet = heapless::Vec<StrSet, U3>;

pub trait P = ToBigUint + Clone + SubAssign + PartialOrd;
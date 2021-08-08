#![doc = include_str!("../README.md")]

/// Contains all thirteen strings.
pub mod thirteen_strings;

use fnv::FnvHashSet as HashSet;
use num_traits::FromPrimitive;
use once_cell::sync::OnceCell;
use std::fmt::Debug;
use std::ops::Rem;
use thirteen_strings::THIRTEEN_STRINGS;

/// A type that can be compared to thirteen. This trait is implemented for all primitive types and
/// `&str`.
pub trait IsThirteen {
    /// Returns `true` if self is thirteen.
    fn thirteen(&self) -> bool;
}

macro_rules! impl_for_integer {
    ($type:ty) => {
        impl IsThirteen for $type {
            /// Returns `true` if `self == 13`.
            fn thirteen(&self) -> bool {
                *self == 13
            }
        }
    };
}

impl_for_integer!(i8);
impl_for_integer!(i16);
impl_for_integer!(i32);
impl_for_integer!(i64);
impl_for_integer!(i128);
impl_for_integer!(isize);
impl_for_integer!(u8);
impl_for_integer!(u16);
impl_for_integer!(u32);
impl_for_integer!(u64);
impl_for_integer!(u128);
impl_for_integer!(usize);

macro_rules! impl_for_float {
    ($type:ty) => {
        impl IsThirteen for $type {
            /// Returns `true` if `self` is approximately `13`.
            fn thirteen(&self) -> bool {
                (self - 13.0).abs() < <$type>::EPSILON
            }
        }
    };
}

impl_for_float!(f64);
impl_for_float!(f32);

impl IsThirteen for &str {
    /// Returns `true` if:
    /// - `self` equals `"13"` or `"B"`
    /// - `self` is 13 characters long and all characters are equal to each other
    /// - The lowercase version of `self` is included in [`thirteen_strings::THIRTEEN_STRINGS`]
    fn thirteen(&self) -> bool {
        matches!(*self, "13" | "B")
            || (self.len() == 13 && self.bytes().all(|b| matches!(b, b'I' | b'l' | b'1')))
            || is_thirteen_equal_chars(self)
            // The next line could be non-allocating if there is an ascii-only IsThirteen
            || THIRTEEN_STRINGS.contains(self.to_lowercase().as_str())
    }
}

fn is_thirteen_equal_chars(s: &str) -> bool {
    if let Some(first_char) = s.chars().next() {
        if s.chars().count() == 13 {
            s.chars().all(|c| c == first_char)
        } else {
            false
        }
    } else {
        false
    }
}

impl IsThirteen for String {
    fn thirteen(&self) -> bool {
        self.as_str().thirteen()
    }
}

impl IsThirteen for char {
    /// Returns `true` if self matches a thirteen character.
    fn thirteen(&self) -> bool {
        matches!(*self, 'B' | 'ß' | 'β' | '阝')
    }
}

macro_rules! impl_always_false {
    ($type:ty) => {
        impl IsThirteen for $type {
            /// Returns `false`.
            fn thirteen(&self) -> bool {
                false
            }
        }
    };
}

impl_always_false!(bool);
impl_always_false!(());

/// `Roughly` is thirteen if it is in [12.5, 13.5).
#[derive(Debug, Copy, Clone)]
pub struct Roughly(pub f64);

impl IsThirteen for Roughly {
    fn thirteen(&self) -> bool {
        (12.5..13.5).contains(&self.0)
    }
}

/// `Returns` calls its closure and compares the returned value to thirteen.
#[derive(Debug, Clone)]
pub struct Returns<T>(pub T);

impl<F, R> IsThirteen for Returns<F>
where
    F: Fn() -> R,
    R: IsThirteen,
{
    fn thirteen(&self) -> bool {
        self.0().thirteen()
    }
}

/// `DivisibleBy` is thirteen if it is a divisor of 13.
#[derive(Debug, Copy, Clone)]
pub struct DivisibleBy<T>(pub T);

impl<T, RemOutput> IsThirteen for DivisibleBy<T>
where
    T: Rem<Output = RemOutput> + FromPrimitive + Copy,
    RemOutput: PartialEq + FromPrimitive,
{
    fn thirteen(&self) -> bool {
        self.0 % FromPrimitive::from_u64(13).unwrap() == FromPrimitive::from_u64(0).unwrap()
    }
}

/// `GreaterThan` returns `true` if it is greater than 13.
#[derive(Debug, Copy, Clone)]
pub struct GreaterThan<T>(pub T);

impl<T> IsThirteen for GreaterThan<T>
where
    T: PartialOrd + FromPrimitive,
{
    fn thirteen(&self) -> bool {
        self.0 > FromPrimitive::from_u64(13).unwrap()
    }
}

/// `LessThan` returns `true` if it is greater than 13.
#[derive(Debug, Copy, Clone)]
pub struct LessThan<T>(pub T);

impl<T> IsThirteen for LessThan<T>
where
    T: PartialOrd + FromPrimitive,
{
    fn thirteen(&self) -> bool {
        self.0 < FromPrimitive::from_u64(13).unwrap()
    }
}

/// `Within` has a custom tolerance for equalling thirteen.
#[derive(Debug, Copy, Clone)]
pub struct Within {
    value: f64,
    radius: f64,
}

impl Within {
    /// `radius` is how far `value` can be from 13 to equal 13. That makes sense, right?
    pub fn new(value: f64, radius: f64) -> Self {
        Self { value, radius }
    }
}

impl IsThirteen for Within {
    fn thirteen(&self) -> bool {
        (self.value - 13.0).abs() <= self.radius
    }
}

/// `CanSpell` is thirteen if its set of characters is a superset of those in "thirteen."
#[derive(Debug, Clone)]
pub struct CanSpell {
    letters: HashSet<u8>,
}

impl CanSpell {
    pub fn new(s: &str) -> Self {
        Self {
            letters: s.bytes().map(|b| b.to_ascii_lowercase()).collect(),
        }
    }
}

impl IsThirteen for CanSpell {
    fn thirteen(&self) -> bool {
        [b't', b'h', b'i', b'r', b't', b'e', b'e', b'n']
            .iter()
            .all(|b| self.letters.contains(b))
    }
}

/// `AnagramOf` is thirteen if it is an [anagram](https://en.wikipedia.org/wiki/Anagram) of
/// "thirteen."
#[derive(Debug, Clone)]
pub struct AnagramOf {
    bytes: HashSet<u8>,
}

impl AnagramOf {
    pub fn new(s: &str) -> Self {
        Self {
            bytes: s.bytes().map(|b| b.to_ascii_lowercase()).collect(),
        }
    }
}

const THIRTEEN_STR: &str = "thirteen";
static THIRTEEN_LETTERS: OnceCell<HashSet<u8>> = OnceCell::new();

impl IsThirteen for AnagramOf {
    fn thirteen(&self) -> bool {
        self.bytes == *THIRTEEN_LETTERS.get_or_init(|| THIRTEEN_STR.bytes().collect())
    }
}

/// `Backwards` is thirteen if its lowercase version equals `"neetriht"` (reverse spelling of
/// "thirteen"). This is different from the original JS version as the original is case-sensitive.
#[derive(Debug, Clone)]
pub struct Backwards<'s>(pub &'s str);

impl IsThirteen for Backwards<'_> {
    fn thirteen(&self) -> bool {
        self.0.eq_ignore_ascii_case("neetriht")
    }
}

/// `AtomicNumber` is thirteen if the string equals `"aluminum"`.
#[derive(Debug, Clone)]
pub struct AtomicNumber<'s>(pub &'s str);

impl IsThirteen for AtomicNumber<'_> {
    fn thirteen(&self) -> bool {
        self.0.eq_ignore_ascii_case("aluminum")
    }
}

#[cfg(test)]
mod lib_test;

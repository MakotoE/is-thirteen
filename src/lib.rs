mod thirteen_strings;

use fnv::FnvHashSet as HashSet;
use std::fmt::{Debug, Formatter};
use std::num::ParseFloatError;
use std::str::FromStr;
use thirteen_strings::THIRTEEN_STRINGS;

const THIRTEEN_STR: &str = "thirteen";

pub trait IsThirteen {
    fn is_thirteen(&self) -> bool;
}

macro_rules! impl_for_integer {
    ($type:ty) => {
        impl IsThirteen for $type {
            fn is_thirteen(&self) -> bool {
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
            fn is_thirteen(&self) -> bool {
                (self - 13.0).abs() < <$type>::EPSILON
            }
        }
    };
}

impl_for_float!(f64);
impl_for_float!(f32);

impl IsThirteen for &str {
    fn is_thirteen(&self) -> bool {
        *self == "13"
            || (self.len() == 13 && self.bytes().all(|b| matches!(b, b'I' | b'l' | b'1')))
            || is_thirteen_equal_chars(self)
            || THIRTEEN_STRINGS.contains(self)
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
    fn is_thirteen(&self) -> bool {
        self.as_str().is_thirteen()
    }
}

macro_rules! impl_always_false {
    ($type:ty) => {
        impl IsThirteen for $type {
            fn is_thirteen(&self) -> bool {
                false
            }
        }
    };
}

impl_always_false!(bool);
impl_always_false!(char);
impl_always_false!(());

#[derive(Debug, Copy, Clone)]
pub struct Roughly(f64);

impl<T> From<T> for Roughly
where
    T: Into<f64>,
{
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl FromStr for Roughly {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl IsThirteen for Roughly {
    fn is_thirteen(&self) -> bool {
        (12.5..13.5).contains(&self.0)
    }
}

pub struct ReturnedValue<T>(pub T);

macro_rules! impl_debug {
    ($type:ty) => {
        impl<T> Debug for $type
        where
            T: Debug,
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

impl_debug!(ReturnedValue<T>);

impl<F, R> IsThirteen for ReturnedValue<F>
where
    F: Fn() -> R,
    R: IsThirteen,
{
    /// Calls closure to get value
    fn is_thirteen(&self) -> bool {
        self.0().is_thirteen()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Within {
    value: f64,
    radius: f64,
}

impl Within {
    pub fn new<T>(value: T, radius: f64) -> Self
    where
        T: Into<f64>,
    {
        Self {
            value: value.into(),
            radius,
        }
    }

    pub fn from_str(s: &str, radius: f64) -> Result<Self, ParseFloatError> {
        Ok(Self {
            value: s.parse()?,
            radius,
        })
    }
}

impl IsThirteen for Within {
    fn is_thirteen(&self) -> bool {
        (self.value - 13.0).abs() <= self.radius
    }
}

#[derive(Debug, Clone)]
pub struct ContainsLetters {
    letters: HashSet<u8>,
}

impl ContainsLetters {
    pub fn new(s: &str) -> Self {
        Self {
            letters: s.to_lowercase().bytes().collect(),
        }
    }
}

impl IsThirteen for ContainsLetters {
    fn is_thirteen(&self) -> bool {
        [b't', b'h', b'i', b'r', b't', b'e', b'e', b'n']
            .iter()
            .all(|b| self.letters.contains(b))
    }
}

#[derive(Debug, Clone)]
pub struct Anagram {
    // It could be stored as a sorted Vec for smaller size but hash set has better time complexity
    // (O(1) vs O(log(n)).
    letters: HashSet<u8>,
}

impl Anagram {
    pub fn new(s: &str) -> Self {
        Self {
            letters: s.to_lowercase().bytes().collect(),
        }
    }
}

impl IsThirteen for Anagram {
    fn is_thirteen(&self) -> bool {
        let thirteen_letters: HashSet<u8> = THIRTEEN_STR.bytes().collect();
        self.letters == thirteen_letters
    }
}

#[derive(Debug, Clone)]
pub struct Backwards<'s>(pub &'s str);

impl IsThirteen for Backwards<'_> {
    fn is_thirteen(&self) -> bool {
        self.0.to_lowercase().bytes().rev().eq(THIRTEEN_STR.bytes())
    }
}

#[derive(Debug, Clone)]
pub struct AtomicNumber<'s>(pub &'s str);

impl IsThirteen for AtomicNumber<'_> {
    fn is_thirteen(&self) -> bool {
        self.0.to_lowercase() == "aluminum"
    }
}

#[cfg(test)]
mod lib_test;

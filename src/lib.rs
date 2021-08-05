pub mod thirteen_strings;

use fnv::FnvHashSet as HashSet;
use once_cell::sync::OnceCell;
use std::fmt::Debug;
use thirteen_strings::THIRTEEN_STRINGS;

/// A type that can be compared to thirteen. This trait is implemented for all primitive types and
/// `&str`.
pub trait IsThirteen {
    /// Returns `true` if self is thirteen.
    fn is_thirteen(&self) -> bool;
}

macro_rules! impl_for_integer {
    ($type:ty) => {
        impl IsThirteen for $type {
            /// Returns `true` if `self == 13`.
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
            /// Returns `true` if `self` is approximately `13`.
            fn is_thirteen(&self) -> bool {
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
    fn is_thirteen(&self) -> bool {
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
    fn is_thirteen(&self) -> bool {
        self.as_str().is_thirteen()
    }
}

impl IsThirteen for char {
    /// Returns `true` if self matches a thirteen character.
    fn is_thirteen(&self) -> bool {
        matches!(*self, 'B' | 'ß' | 'β' | '阝')
    }
}

macro_rules! impl_always_false {
    ($type:ty) => {
        impl IsThirteen for $type {
            /// Returns `false`.
            fn is_thirteen(&self) -> bool {
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
    fn is_thirteen(&self) -> bool {
        (12.5..13.5).contains(&self.0)
    }
}

/// `ReturnedValue` calls closure to get the value to compare to thirteen.
#[derive(Debug, Clone)]
pub struct ReturnedValue<T>(pub T);

impl<F, R> IsThirteen for ReturnedValue<F>
where
    F: Fn() -> R,
    R: IsThirteen,
{
    fn is_thirteen(&self) -> bool {
        self.0().is_thirteen()
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
    fn is_thirteen(&self) -> bool {
        (self.value - 13.0).abs() <= self.radius
    }
}

/// `ContainsLetters` is thirteen if its set of characters is a superset of those in "thirteen."
#[derive(Debug, Clone)]
pub struct ContainsLetters {
    letters: HashSet<u8>,
}

impl ContainsLetters {
    pub fn new(s: &str) -> Self {
        Self {
            letters: s.bytes().map(|b| b.to_ascii_lowercase()).collect(),
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

/// `Anagram` is thirteen if it is an [anagram](https://en.wikipedia.org/wiki/Anagram) of
/// "thirteen."
#[derive(Debug, Clone)]
pub struct Anagram {
    bytes: HashSet<u8>,
}

impl Anagram {
    pub fn new(s: &str) -> Self {
        Self {
            bytes: s.bytes().map(|b| b.to_ascii_lowercase()).collect(),
        }
    }
}

const THIRTEEN_STR: &str = "thirteen";
static THIRTEEN_LETTERS: OnceCell<HashSet<u8>> = OnceCell::new();

impl IsThirteen for Anagram {
    fn is_thirteen(&self) -> bool {
        self.bytes == *THIRTEEN_LETTERS.get_or_init(|| THIRTEEN_STR.bytes().collect())
    }
}

/// `Backwards` is thirteen if it is the reverse spelling of "thirteen."
#[derive(Debug, Clone)]
pub struct Backwards<'s>(pub &'s str);

impl IsThirteen for Backwards<'_> {
    fn is_thirteen(&self) -> bool {
        self.0
            .bytes()
            .map(|b| b.to_ascii_lowercase())
            .rev()
            .eq(THIRTEEN_STR.bytes())
    }
}

/// `AtomicNumberOf` is thirteen if it contains the string `"aluminum"`.
#[derive(Debug, Clone)]
pub struct AtomicNumberOf<'s>(pub &'s str);

impl IsThirteen for AtomicNumberOf<'_> {
    fn is_thirteen(&self) -> bool {
        self.0.eq_ignore_ascii_case("aluminum")
    }
}

#[cfg(test)]
mod lib_test;

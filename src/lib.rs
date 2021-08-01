mod thirteen_strings;
use thirteen_strings::THIRTEEN_STRINGS;

pub trait IsThirteen {
    fn is_thirteen(&self) -> bool;
}

macro_rules! impl_for_number {
    ($type:ty) => {
        impl IsThirteen for $type {
            fn is_thirteen(&self) -> bool {
                *self == 13
            }
        }
    };
}

impl_for_number!(i8);
impl_for_number!(i16);
impl_for_number!(i32);
impl_for_number!(i64);
impl_for_number!(i128);
impl_for_number!(isize);
impl_for_number!(u8);
impl_for_number!(u16);
impl_for_number!(u32);
impl_for_number!(u64);
impl_for_number!(u128);
impl_for_number!(usize);

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
            || THIRTEEN_STRINGS.contains(self)
            || THIRTEEN_STRINGS.contains(&self.to_lowercase().as_str())
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // Tests from the is-thirteen suite
    // https://github.com/jezen/is-thirteen/blob/3e1cc843db584f7c8a9a13d8bc74a5e4bd1fa82f/test.js
    #[case(13, true)] // 1
    #[case("13", true)] // 2
    #[case("۱۳", true)] // 3
    #[case("XIII", true)] // 4
    #[case("xiii", true)] // 5
    #[case("IIIIIIIIIIIII", true)] // 6
    #[case("IlIlIlIlIlIlI", true)] // 7
    // Added test cases
    #[case(0, false)]
    #[case(13.0, true)]
    #[case("", false)]
    #[case("13".to_string(), true)]
    #[case(true, false)]
    #[case('1', false)]
    #[case((), false)]
    #[case("1111111111111", true)] // 6
    fn is_thirteen<T>(#[case] input: T, #[case] expected: bool)
    where
        T: IsThirteen,
    {
        assert_eq!(input.is_thirteen(), expected);
    }
}

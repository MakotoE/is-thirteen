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
    #[case("https://en.wikipedia.org/wiki/This_Is_Thirteen", true)] // 8
    #[case("https://scontent.cdninstagram.com/hphotos-xtf1/t51.2885-15/s320x320/e35/12237511_444845689040315_1101385461_n.jpg", true)] // 9
    #[case("http://www.metal-archives.com/images/1/5/3/7/153772.jpg", false)] // 10
    #[case("https://www.youtube.com/watch?v=pte3Jg-2Ax4", true)] // 11
    #[case("https://www.youtube.com/watch?v=33Kv5D2zwyc", true)] // 12
    #[case("thirteen", true)] // 13
    #[case("Thirteen", true)] // 14
    #[case("Remy Hadley", true)] // 15
    #[case("Olivia Wilde", true)] // 16
    #[case("weedle", true)] // 17
    #[case("baker's dozen", true)] // 18
    #[case("Dr. Remy Beauregard Hadley", true)] // 19
    #[case("Patty Tsai", true)] // 20
    #[case("PT", true)] // 21
    #[case("Washington Luís", true)] // 22
    #[case("Millard Fillmore", true)] // 23
    #[case("https://en.wikipedia.org/wiki/XIII_(video_game)", true)] // 24
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

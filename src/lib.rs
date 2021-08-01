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
    }
}

impl IsThirteen for String {
    fn is_thirteen(&self) -> bool {
        self.as_str().is_thirteen()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, false)]
    #[case(13, true)]
    #[case(13.0, true)]
    #[case("", false)]
    #[case("13", true)]
    #[case("13".to_string(), true)]
    fn is_thirteen<T>(#[case] input: T, #[case] expected: bool)
    where
        T: IsThirteen,
    {
        assert_eq!(input.is_thirteen(), expected);
    }
}

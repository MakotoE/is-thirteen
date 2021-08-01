mod thirteen_strings;
use std::fmt::{Debug, Formatter};
use std::num::ParseFloatError;
use std::str::FromStr;
use thirteen_strings::THIRTEEN_STRINGS;

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

pub struct Roughly<T>(pub T);

impl FromStr for Roughly<f64> {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Roughly::<f64>(s.parse()?))
    }
}

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

impl_debug!(Roughly<T>);

impl<T> IsThirteen for Roughly<T>
where
    T: Into<f64> + Clone,
{
    fn is_thirteen(&self) -> bool {
        let f: f64 = self.0.clone().into();
        (12.5..13.5).contains(&f)
    }
}

pub struct ReturnedValue<T>(pub T);

impl_debug!(ReturnedValue<T>);

impl<F, R> IsThirteen for ReturnedValue<F>
where
    F: Fn() -> R,
    R: IsThirteen,
{
    fn is_thirteen(&self) -> bool {
        self.0().is_thirteen()
    }
}

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
    #[case(
        "https://scontent.cdninstagram.com/hphotos-xtf1/t51.2885-15/s320x320/e35/12237511_444845689040315_1101385461_n.jpg",
        true,
    )] // 9
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
    #[case("http://www.imdb.com/title/tt0798817/", true)] // 25
    #[case("https://www.imdb.com/title/tt2991516/", true)] // 26
    // #[case(2003, false)] // Can be done with chrono::Utc::today().year()
    #[case("13+0i", true)] // 27
    #[case("13i", true)] //  28
    #[case("13 + 13i", true)] //  29
    #[case("12i", false)] //  30
    #[case("Ei", true)] //  31
    #[case("EI", true)] //  32
    #[case("E1", true)] //  33
    #[case("El", true)] //  34
    #[case("E|", true)] //  35
    #[case("ƖƐ", true)] //  36
    #[case("ƐƖ", true)] //  37
    #[case("th1rt33n", true)] //  38
    #[case("th1rte3n", true)] //  39
    #[case("th1rteen", true)] //  40
    #[case("thirt3en", true)] //  41
    #[case("thirt33n", true)] //  42
    #[case("thirte3n", true)] //  43
    #[case("dertien", true)] //  44
    #[case("ثلاثة عشر", true)] //  45
    #[case("تلطاشر", true)] //  46
    #[case("تلتاشر", true)] //  47
    #[case("طلتاشر", true)] //  48
    #[case("طلطاشر", true)] //  49
    #[case("dertiendertien", true)] //  50
    #[case("seri-un-teng", true)] //  51
    #[case("seriunteng", true)] //  52
    #[case("serí-un-teng", true)] //  53
    #[case("seríunteng", true)] //  54
    #[case("тринадесет", true)] //  55
    #[case("тринайсет", true)] //  56
    #[case("tretze", true)] //  57
    #[case("napulo ug tulo", true)] //  58
    #[case("třináct", true)] //  59
    #[case("十三", true)] //  60
    #[case("拾參", true)] //  61
    #[case("拾叁", true)] //  62
    #[case("拾叄", true)] //  63
    #[case("拾参", true)] //  64
    #[case("trinaest", true)] //  65
    #[case("tretten", true)] //  66
    #[case("senthi", true)] //  67
    #[case("kolmteist", true)] //  68
    #[case("thirteen", true)] //  69
    #[case("labintatlo", true)] //  70
    #[case("kolmetoista", true)] //  71
    #[case("treize", true)] //  72
    #[case("treizième", true)] //  73
    #[case("dreizehn", true)] //  74
    #[case("ცამეტი", true)] //  75
    #[case("‘umikūmākolu", true)] //  76
    #[case("שלוש עשרה", true)] //  77
    #[case("שלושעשרה", true)] //  78
    #[case("ֹשְלֹש- עֶשְֹרֵה", true)] //  79
    #[case("שלושה עשר", true)] //  80
    #[case("שלושהעשר", true)] //  81
    #[case("ֹשְלֹשָה- עָשָֹר", true)] //  82
    #[case("יג", true)] //  83
    #[case("י״ג", true)] //  84
    #[case("तेरह", true)] //  85
    #[case("tizenhárom", true)] //  86
    #[case("trí déag", true)] //  87
    #[case("tredici", true)] //  88
    #[case("on üç", true)] //  89
    #[case("ಹದಿಮೂರು", true)] //  90
    #[case("పదమూడు", true)] //  91
    #[case("೧೩", true)] //  92
    #[case("열셋", true)] //  93
    #[case("십삼", true)] //  94
    #[case("sêzdeh", true)] //  95
    #[case("tredecim", true)] //  96
    #[case("trīspadsmit", true)] //  97
    #[case("trylika", true)] //  98
    #[case("dräizéng", true)] //  99
    #[case("тринаесет", true)] // 100
    #[case("tiga belas", true)] // 101
    #[case("арван", true)] // 102
    #[case(".---- ...--", true)] // 103
    #[case("matlactlihuan yei", true)] // 104
    #[case("mahtlactli omei", true)] // 105
    #[case("mahtlactli ihuan yei", true)] // 106
    #[case("irteenthay", true)] // 107
    #[case("trzynaście", true)] // 108
    #[case("trzynasty", true)] // 109
    #[case("trzynasta", true)] // 110
    #[case("trzynaste", true)] // 111
    #[case("trzynaści", true)] // 112
    #[case("trzynastego", true)] // 113
    #[case("trzynastej", true)] // 114
    #[case("trzynastych", true)] // 115
    #[case("trzynastemu", true)] // 116
    #[case("trzynastym", true)] // 117
    #[case("trzynastą", true)] // 118
    #[case("trzynastymi", true)] // 119
    #[case("trzynastu", true)] // 120
    #[case("trzynastek", true)] // 121
    #[case("trzynastoma", true)] // 122
    #[case("trzynaścioro", true)] // 123
    #[case("trzynastka", true)] // 124
    #[case("trzynastki", true)] // 125
    #[case("trzynastką", true)] // 126
    #[case("trzynastce", true)] // 127
    #[case("trzynastko", true)] // 128
    #[case("trzynaściorgiem", true)] // 129
    #[case("trzynaściorgu", true)] // 130
    #[case("trzynaściorga", true)] // 131
    #[case("trzynastokrotny", true)] // 132
    #[case("trzynastokrotnie", true)] // 133
    #[case("trzynastokrotną", true)] // 134
    #[case("trzynastokrotnemu", true)] // 135
    #[case("trzynastokrotnej", true)] // 136
    #[case("trzynastokrotnych", true)] // 137
    #[case("trzynastokrotność", true)] // 138
    #[case("trzynastokrotności", true)] // 139
    #[case("trzynastokrotnością", true)] // 140
    #[case("treze", true)] // 141
    #[case("ਤੇਰਾਂ", true)] // 142
    #[case("੧੩", true)] // 143
    #[case("treisprezece", true)] // 144
    #[case("тринадцать", true)] // 145
    #[case("тринаест", true)] // 146
    #[case("trinásť", true)] // 147
    #[case("wa’maH wej", true)] // 148
    #[case("trinajst", true)] // 149
    #[case("trece", true)] // 150
    #[case("dektri", true)] // 151
    #[case("trese", true)] // 152
    #[case("tretton", true)] // 153
    #[case("பதின்மூன்று", true)] // 154
    #[case("สิบสาม", true)] // 155
    #[case("тринадцять", true)] // 156
    #[case("تیرہ", true)] // 157
    #[case("tayra", true)] // 158
    #[case("tri ar ddeg", true)] // 159
    #[case("דרייַצן", true)] // 160
    #[case("דרייצן", true)] // 161
    #[case("kumi na tatu", true)] // 162
    #[case("പതിമൂന്ന്", true)] // 163
    #[case("१३", true)] // 164
    #[case("तेह्र", true)] // 165
    #[case("quainel", true)] // 166
    #[case("mînuiug", true)] // 167
    #[case("7h1r733n", true)] // 168
    #[case("B", true)] // 169
    #[case("b", false)] // 170
    #[case("ß", true)] // 171
    #[case("ẞ", true)] // 172
    #[case("Β", true)] // 173
    #[case("β", true)] // 174
    #[case("阝", true)] // 175
    #[case("i3", true)] // 176
    #[case("I3", true)] // 177
    #[case("l3", true)] // 178
    #[case("L3", true)] // 179
    #[case("|3", true)] // 180
    #[case("!3", true)] // 181
    #[case("Dilma", true)] // 182
    #[case(25 - 12, true)] // 183
    #[case(1 + 12, true)] // 184
    #[case((2 * 8 + 11 - 1) / 2, true)] // 185
    #[case((10 - 1 + 32) / 4 * 3, false)] // 186
    #[case(Roughly(((5.3 + 0.5) * 5.0 - 4.0) / 2.0), true)] // 187
    #[case(13, true)] // 188
    #[case(14, false)] // 189
    #[case(u8::from_str_radix("1101", 2).unwrap(), true)] // 190
    #[case(u8::from_str_radix("1111", 2).unwrap(), false)] // 191
    #[case(u8::from_str_radix("15", 8).unwrap(), true)] // 192
    #[case(u8::from_str_radix("13", 8).unwrap(), false)] // 193
    #[case(u8::from_str_radix("d", 16).unwrap(), true)] // 194
    #[case(u8::from_str_radix("D", 16).unwrap(), true)] // 195
    #[case(u8::from_str_radix("A", 16).unwrap(), false)] // 196
    #[case(ReturnedValue(|| 13), true)] // 197
    #[case("|||||||||||||", true)] // 198
    #[case("/////////////", true)] // 199
    #[case("🐱🐱🐱🐱🐱🐱🐱🐱🐱🐱🐱🐱🐱", true)] // 200
    #[case("oooooooooooooo", false)] // 201
    #[case("bbbbbbbbbbb", false)] // 202
    #[case("||h||||||||||", false)] // 203
    #[case("///i/////////", false)] // 204
    // Additional test cases
    #[case(0, false)]
    #[case(13.0, true)]
    #[case("", false)]
    #[case("13".to_string(), true)]
    #[case(true, false)]
    #[case('1', false)]
    #[case((), false)]
    #[case("1111111111111", true)]
    #[case(Roughly(0), false)]
    #[case(Roughly(12.5), true)]
    #[case(Roughly(13), true)]
    #[case(Roughly(13.4), true)]
    #[case(Roughly(13.5), false)]
    #[case(Roughly::from_str("12.5").unwrap(), true)]
    fn is_thirteen<T>(#[case] input: T, #[case] expected: bool)
    where
        T: IsThirteen,
    {
        assert_eq!(input.is_thirteen(), expected);
    }

    #[rstest]
    #[case("", false)]
    #[case("aaaaaaaaaaaaa", true)]
    fn test_is_thirteen_equal_chars(#[case] s: &str, #[case] expected: bool) {
        assert_eq!(is_thirteen_equal_chars(s), expected);
    }
}

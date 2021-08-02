use super::*;
use chrono::Datelike;
use chrono::TimeZone;
use chrono::Utc;
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
#[case(Utc::today().year() - Utc.ymd(2003, 1, 1).year(), false)] // 27
#[case("13+0i", true)] // 28
#[case("13i", true)] // 29
#[case("13 + 13i", true)] // 30
#[case("12i", false)] // 31
#[case("Ei", true)] // 32
#[case("EI", true)] // 33
#[case("E1", true)] // 34
#[case("El", true)] // 35
#[case("E|", true)] // 36
#[case("ƖƐ", true)] // 37
#[case("ƐƖ", true)] // 38
#[case("th1rt33n", true)] // 39
#[case("th1rte3n", true)] // 40
#[case("th1rteen", true)] // 41
#[case("thirt3en", true)] // 42
#[case("thirt33n", true)] // 43
#[case("thirte3n", true)] // 44
#[case("dertien", true)] // 45
#[case("ثلاثة عشر", true)] // 46
#[case("تلطاشر", true)] // 47
#[case("تلتاشر", true)] // 48
#[case("طلتاشر", true)] // 49
#[case("طلطاشر", true)] // 50
#[case("dertiendertien", true)] // 51
#[case("seri-un-teng", true)] // 52
#[case("seriunteng", true)] // 53
#[case("serí-un-teng", true)] // 54
#[case("seríunteng", true)] // 55
#[case("тринадесет", true)] // 56
#[case("тринайсет", true)] // 57
#[case("tretze", true)] // 58
#[case("napulo ug tulo", true)] // 59
#[case("třináct", true)] // 60
#[case("十三", true)] // 61
#[case("拾參", true)] // 62
#[case("拾叁", true)] // 63
#[case("拾叄", true)] // 64
#[case("拾参", true)] // 65
#[case("trinaest", true)] // 66
#[case("tretten", true)] // 67
#[case("senthi", true)] // 68
#[case("kolmteist", true)] // 69
#[case("thirteen", true)] // 70
#[case("labintatlo", true)] // 71
#[case("kolmetoista", true)] // 72
#[case("treize", true)] // 73
#[case("treizième", true)] // 74
#[case("dreizehn", true)] // 75
#[case("ცამეტი", true)] // 76
#[case("‘umikūmākolu", true)] // 77
#[case("שלוש עשרה", true)] // 78
#[case("שלושעשרה", true)] // 79
#[case("ֹשְלֹש- עֶשְֹרֵה", true)] // 80
#[case("שלושה עשר", true)] // 81
#[case("שלושהעשר", true)] // 82
#[case("ֹשְלֹשָה- עָשָֹר", true)] // 83
#[case("יג", true)] // 84
#[case("י״ג", true)] // 85
#[case("तेरह", true)] // 86
#[case("tizenhárom", true)] // 87
#[case("trí déag", true)] // 88
#[case("tredici", true)] // 89
#[case("on üç", true)] // 90
#[case("ಹದಿಮೂರು", true)] // 91
#[case("పదమూడు", true)] // 92
#[case("೧೩", true)] // 93
#[case("열셋", true)] // 94
#[case("십삼", true)] // 95
#[case("sêzdeh", true)] // 96
#[case("tredecim", true)] // 97
#[case("trīspadsmit", true)] // 98
#[case("trylika", true)] // 99
#[case("dräizéng", true)] // 100
#[case("тринаесет", true)] // 101
#[case("tiga belas", true)] // 102
#[case("арван", true)] // 103
#[case(".---- ...--", true)] // 104
#[case("matlactlihuan yei", true)] // 105
#[case("mahtlactli omei", true)] // 106
#[case("mahtlactli ihuan yei", true)] // 107
#[case("irteenthay", true)] // 108
#[case("trzynaście", true)] // 109
#[case("trzynasty", true)] // 110
#[case("trzynasta", true)] // 111
#[case("trzynaste", true)] // 112
#[case("trzynaści", true)] // 113
#[case("trzynastego", true)] // 114
#[case("trzynastej", true)] // 115
#[case("trzynastych", true)] // 116
#[case("trzynastemu", true)] // 117
#[case("trzynastym", true)] // 118
#[case("trzynastą", true)] // 119
#[case("trzynastymi", true)] // 120
#[case("trzynastu", true)] // 121
#[case("trzynastek", true)] // 122
#[case("trzynastoma", true)] // 123
#[case("trzynaścioro", true)] // 124
#[case("trzynastka", true)] // 125
#[case("trzynastki", true)] // 126
#[case("trzynastką", true)] // 127
#[case("trzynastce", true)] // 128
#[case("trzynastko", true)] // 129
#[case("trzynaściorgiem", true)] // 130
#[case("trzynaściorgu", true)] // 131
#[case("trzynaściorga", true)] // 132
#[case("trzynastokrotny", true)] // 133
#[case("trzynastokrotnie", true)] // 134
#[case("trzynastokrotną", true)] // 135
#[case("trzynastokrotnemu", true)] // 136
#[case("trzynastokrotnej", true)] // 137
#[case("trzynastokrotnych", true)] // 138
#[case("trzynastokrotność", true)] // 139
#[case("trzynastokrotności", true)] // 140
#[case("trzynastokrotnością", true)] // 141
#[case("treze", true)] // 142
#[case("ਤੇਰਾਂ", true)] // 143
#[case("੧੩", true)] // 144
#[case("treisprezece", true)] // 145
#[case("тринадцать", true)] // 146
#[case("тринаест", true)] // 147
#[case("trinásť", true)] // 148
#[case("wa’maH wej", true)] // 149
#[case("trinajst", true)] // 150
#[case("trece", true)] // 151
#[case("dektri", true)] // 152
#[case("trese", true)] // 153
#[case("tretton", true)] // 154
#[case("பதின்மூன்று", true)] // 155
#[case("สิบสาม", true)] // 156
#[case("тринадцять", true)] // 157
#[case("تیرہ", true)] // 158
#[case("tayra", true)] // 159
#[case("tri ar ddeg", true)] // 160
#[case("דרייַצן", true)] // 161
#[case("דרייצן", true)] // 162
#[case("kumi na tatu", true)] // 163
#[case("പതിമൂന്ന്", true)] // 164
#[case("१३", true)] // 165
#[case("तेह्र", true)] // 166
#[case("quainel", true)] // 167
#[case("mînuiug", true)] // 168
#[case("7h1r733n", true)] // 169
#[case("B", true)] // 170
#[case("b", false)] // 171
#[case("ß", true)] // 172
#[case("ẞ", true)] // 173
#[case("Β", true)] // 174
#[case("β", true)] // 175
#[case("阝", true)] // 176
#[case("i3", true)] // 177
#[case("I3", true)] // 178
#[case("l3", true)] // 179
#[case("L3", true)] // 180
#[case("|3", true)] // 181
#[case("!3", true)] // 182
#[case("Dilma", true)] // 183
#[case(25 - 12, true)] // 184
#[case(1 + 12, true)] // 185
#[case((2 * 8 + 11 - 1) / 2, true)] // 186
#[case((10 - 1 + 32) / 4 * 3, false)] // 187
#[case(Roughly::from(((5.3 + 0.5) * 5.0 - 4.0) / 2.0), true)] // 188
#[case(13, true)] // 189
#[case(14, false)] // 190
#[case(u8::from_str_radix("1101", 2).unwrap(), true)] // 191
#[case(u8::from_str_radix("1111", 2).unwrap(), false)] // 192
#[case(u8::from_str_radix("15", 8).unwrap(), true)] // 193
#[case(u8::from_str_radix("13", 8).unwrap(), false)] // 194
#[case(u8::from_str_radix("d", 16).unwrap(), true)] // 195
#[case(u8::from_str_radix("D", 16).unwrap(), true)] // 196
#[case(u8::from_str_radix("A", 16).unwrap(), false)] // 197
#[case(ReturnedValue(|| 13), true)] // 198
#[case("|||||||||||||", true)] // 199
#[case("/////////////", true)] // 200
#[case("🐱🐱🐱🐱🐱🐱🐱🐱🐱🐱🐱🐱🐱", true)] // 201
#[case("oooooooooooooo", false)] // 202
#[case("bbbbbbbbbbb", false)] // 203
#[case("||h||||||||||", false)] // 204
#[case("///i/////////", false)] // 205
// Additional test cases
#[case(0, false)] // 206
#[case(13.0, true)] // 207
#[case("", false)] // 208
#[case("13".to_string(), true)] // 209
#[case(true, false)] // 210
#[case('1', false)] // 211
#[case((), false)] // 212
#[case("1111111111111", true)] // 213
#[case(Roughly::from(0), false)] // 214
#[case(Roughly::from(12.5), true)] // 215
#[case(Roughly::from(13), true)] // 216
#[case(Roughly::from(13.4), true)] // 217
#[case(Roughly::from(13.5), false)] // 218
#[case(Roughly::from_str("12.5").unwrap(), true)] // 219
#[case(Within::new(0, 1.0), false)] // 220
#[case(Within::new(12, 1.0), true)] // 221
#[case(ContainsLetters::new(""), false)] // 222
#[case(ContainsLetters::new("eihbtrtAecdn"), true)] // 223
#[case(Anagram::new(""), false)] // 224
#[case(Anagram::new("nRteehit"), true)] // 225
#[case(Backwards(""), false)] // 226
#[case(Backwards("neetRiht"), true)] // 227
#[case(AtomicNumber(""), false)] // 228
#[case(AtomicNumber("Aluminum"), true)] // 229
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

#[test]
fn thirteen_strings() {
    // All THIRTEEN_STRINGS should be lowercase
    assert!(THIRTEEN_STRINGS.iter().all(|&s| s == s.to_lowercase()));
}

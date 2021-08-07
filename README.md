[![Latest version](https://img.shields.io/crates/v/is-thirteen.svg)](https://crates.io/crates/is-thirteen) [![Documentation](https://docs.rs/is-thirteen/badge.svg)](https://docs.rs/is-thirteen/)

"Sometimes it is too difficult to figure out if a variable equals 13 or not. That's why we need `is-thirteen`."

&emsp;— Literally no one

This very useful crate is a production-ready* solution for all of your 13-comparison needs.

\* I like asterisks.

This is a port of [is-thirteen](https://github.com/jezen/is-thirteen) and is tested with all tests from the original library. A RIIR blog about how our servers improved by x10-100 will be posted to Medium shortly.

# Syntax map

The syntax of this library is quite different from that of the original library. This table shows how your messy JS code can be cleaned up with the Rust version.

| Original version | Rust version |
|-|-|
| `is(x).thirteen()` | `x.thirteen()` |
| `is(x).roughly.thirteen()` | `Roughly(x).thirteen()` |
| `is(() => x).returning.thirteen()` | `Returns(\|\| x).thirteen()` |
| `is(x).not.thirteen()` | `!x.thirteen() // How lazy do you have to be if you need a library to negate a Boolean?` |
| `is(x).divisible.by.thirteen()` | `DivisibleBy(x).thirteen()` |
| `is(x).square.of.thirteen()` | `(x * x).thirteen()` |
| `is(x).greater.than.thirteen()` | `GreaterThan(x).thirteen()` |
| `is(x).less.than.thirteen()` | `LessThan(x).thirteen()` |
| `is(x).within(1).of.thirteen()` | `Within::new(x, 1.0).thirteen()` |
| `is(x).yearOfBirth()` | `(chrono::Utc::today().year() - x).thirteen()` |
| `is(x).plus(2).thirteen()` | `(x + 2).thirteen()` |
| `is(x).minus(2).thirteen()` | `(x - 2).thirteen()` |
| `is(x).times(2).thirteen()` | `(x * 2).thirteen()` |
| `is(x).dividedby(2).thirteen()` | `(x / 2).thirteen()` |
| `is(x).canSpell.thirteen()` | `CanSpell(x).thirteen()` |
| `is(x).anagramOf.thirteen()` | `AnagramOf(x).thirteen()` |
| `is(x).backwards.thirteen()` | `Backwards(x).thirteen()` |
| `is(x).atomicNumber.thirteen()` | `AtomicNumber(x).thirteen()` |
| `is(x).base(16).thirteen()` | `i64::from_str_radix(x, 16).thirteen()` |
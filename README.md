"Sometimes it is difficult to figure out if a variable equals 13 or not. That is why we need `npm install is-thirteen`."

&emsp;— Literally no one

This very useful crate is a production-ready* solution for all of your 13-comparison needs.

\* I like asterisks.

This is a port of [is-thirteen](https://github.com/jezen/is-thirteen) and features all tests from the original library. A RIIR blog about how our servers improved by x10-100 will be posted to Medium shortly.

# Syntax map

The syntax of this library is quite different from that of the original library. This table shows how your messy code from the JS version can be cleaned in the Rust version.

| Original version | Rust version |
|-|-|
| `is(x).thirteen()` | `x.is_thirteen()` |
| `is(x).roughly.thirteen()` | `Roughly(x).is_thirteen()` |
| `is(() => x).returning.thirteen()` | `ReturnedValue(|| x).is_thirteen()` |
| `is(x).not.thirteen()` | `(!x).is_thirteen()` |
| `is(x).divisible.thirteen()` | `` |
| `is(x).square.thirteen()` | `` |
| `is(x).greater.thirteen()` | `` |
| `is(x).less.thirteen()` | `` |
| `is(x).within(1).thirteen()` | `Within::new(x, 1.0).is_thirteen()` |
| `is(x).yearOfBirth().thirteen()` | `` |
| `is(x).plus(2).thirteen()` | `(x + 2).is_thirteen()` |
| `is(x).minus(2).thirteen()` | `(x - 2).is_thirteen()` |
| `is(x).times(2).thirteen()` | `(x * 2).is_thirteen()` |
| `is(x).dividedby(2).thirteen()` | `(x / 2).is_thirteen()` |
| `is(x).canSpell.thirteen()` | `ContainsLetters(x).is_thirteen()` |
| `is(x).anagramOf.thirteen()` | `Anagram(x).is_thirteen()` |
| `is(x).backwards.thirteen()` | `Backwards(x).is_thirteen()` |
| `is(x).atomicNumber.thirteen()` | `AtomicNumber(x).is_thirteen()` |
| `is(x).base.thirteen()` | `` |
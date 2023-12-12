# Egyptian Fraction in Rust

An Egyptian Fraction is the sum of unique unit fractions, such as `1/2`, `1/3`, `1/16`, etc.

Every positive fraction can be represented as a sum of unique Egyptian Fractions. For example, the Egyptian Fraction representation of `2/3` is `1/2 + 1/6`.

## Usage

To use the `egyptian_fraction` function in Rust, include it in your project and call it with the numerator and denominator of the fraction you wish to represent as an Egyptian Fraction.

## Example

```rust
use fraction::Fraction;
use egyptian_fraction::print_egyptian_fraction;

let fraction = Fraction::from("5/121");

// this is an Option<Vec<Fraction>>
let egyptian_representation = egyptian_fraction(fraction);

// You can also pretty print it:
print_egyptian_fraction(fraction);
// this prints:
// 1/25 + 1/757 + 1/763309 + 1/873960180913 + 1/1527612795642093418846225
```

See `main.rs` and `egyptian_fraction.rs` for more

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

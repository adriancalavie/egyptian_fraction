use crate::{egyptian_fraction::print_egyptian_fraction, fraction::Fraction};

pub mod egyptian_fraction;
pub mod fraction;

fn main() {
    print_egyptian_fraction(Fraction::new(0, 1))
}

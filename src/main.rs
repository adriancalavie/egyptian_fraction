use crate::{egyptian_fraction::print_egyptian_fraction, fraction::Fraction};

mod egyptian_fraction;
mod fraction;

fn main() {
    print_egyptian_fraction(Fraction::new(5, 121))
}

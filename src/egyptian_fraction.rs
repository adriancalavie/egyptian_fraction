use crate::fraction::Fraction;

fn compute_egyptian_fraction(fraction: Fraction) -> Option<Vec<Fraction>> {
    let mut fractions = vec![];

    if fraction.denominator == 0 {
        return None;
    }
    if fraction.numerator % fraction.denominator == 0 {
        return Some(vec![Fraction::new(
            fraction.numerator / fraction.denominator,
            1,
        )]);
    }
    if fraction.denominator % fraction.numerator == 0 {
        return Some(vec![Fraction::new(
            1,
            fraction.denominator / fraction.numerator,
        )]);
    }

    let integer_part = (fraction.denominator / fraction.numerator) + 1;
    fractions.push(Fraction::new(1, integer_part));

    match compute_egyptian_fraction(Fraction::new(
        fraction.numerator * integer_part - fraction.denominator,
        fraction.denominator * integer_part,
    )) {
        None => {
            return None;
        }
        Some(f) => {
            fractions.extend(f);
        }
    }

    Some(fractions)
}

fn fold_unitary_fractions(fractions: Vec<Fraction>) -> Vec<Fraction> {
    let (unit_fractions, non_unit_fractions): (Vec<Fraction>, Vec<Fraction>) =
        fractions.into_iter().partition(Fraction::is_unitary);

    let sum_of_units = unit_fractions
        .into_iter()
        .fold(0, |acc, f| acc + f.numerator);

    if sum_of_units == 0 {
        return non_unit_fractions;
    }

    let mut bundled_fractions = vec![Fraction {
        numerator: sum_of_units,
        denominator: 1,
    }];

    bundled_fractions.extend(non_unit_fractions);
    bundled_fractions
}

pub(crate) fn egyptian_fraction(fraction: Fraction) -> Option<Vec<Fraction>> {
    compute_egyptian_fraction(fraction).map(fold_unitary_fractions)
}

pub(crate) fn print_egyptian_fraction(fraction: Fraction) {
    if let Some(fractions) = egyptian_fraction(fraction) {
        for (i, f) in fractions.iter().enumerate() {
            if i < fractions.len() - 1 {
                print!("{} + ", f);
            } else {
                print!("{}", f);
            }
        }
    }
}

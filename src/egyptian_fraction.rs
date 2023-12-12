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
    compute_egyptian_fraction(fraction)
        .map(fold_unitary_fractions)
        .filter(|f| !f.is_empty())
}

pub(crate) fn to_egyptian_fraction_notation(fraction: Fraction) -> String {
    if let Some(fractions) = egyptian_fraction(fraction) {
        fractions
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(" + ")
    } else {
        String::from("Egyptian Fraction couldn't be computed")
    }
}

pub(crate) fn print_egyptian_fraction(fraction: Fraction) {
    println!("{} = {}", fraction, to_egyptian_fraction_notation(fraction));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_unitary_fractions_empty() {
        let fractions = vec![];
        let result = fold_unitary_fractions(fractions);
        assert!(result.is_empty());
    }

    #[test]
    fn test_fold_unitary_fractions_only_unitary() {
        let fractions = vec![
            Fraction {
                numerator: 1,
                denominator: 1,
            },
            Fraction {
                numerator: 1,
                denominator: 1,
            },
        ];
        let expected = vec![Fraction {
            numerator: 2,
            denominator: 1,
        }];
        let result = fold_unitary_fractions(fractions);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fold_unitary_fractions_mixed() {
        let fractions = vec![
            Fraction {
                numerator: 1,
                denominator: 1,
            },
            Fraction {
                numerator: 1,
                denominator: 2,
            },
            Fraction {
                numerator: 1,
                denominator: 1,
            },
            Fraction {
                numerator: 1,
                denominator: 3,
            },
        ];
        let expected = vec![
            Fraction {
                numerator: 2,
                denominator: 1,
            },
            Fraction {
                numerator: 1,
                denominator: 2,
            },
            Fraction {
                numerator: 1,
                denominator: 3,
            },
        ];
        let result = fold_unitary_fractions(fractions);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_egyptian_fraction_valid() {
        let fraction = Fraction {
            numerator: 2,
            denominator: 3,
        };
        let result = egyptian_fraction(fraction);
        assert!(result.is_some());

        // check result is 1/2 and 1/6
        let expected = vec![
            Fraction {
                numerator: 1,
                denominator: 2,
            },
            Fraction {
                numerator: 1,
                denominator: 6,
            },
        ];
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_egyptian_fraction_invalid() {
        let fraction = Fraction {
            numerator: 0,
            denominator: 1,
        };
        let result = egyptian_fraction(fraction);
        println!("{:?}", result);
        assert!(result.is_none());
    }

    #[test]
    fn test_to_egyptian_fraction() {
        let fraction = Fraction {
            numerator: 2,
            denominator: 5,
        };
        assert_eq!(to_egyptian_fraction_notation(fraction), "1/3 + 1/15");
    }
}

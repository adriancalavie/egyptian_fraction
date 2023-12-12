use num::integer::gcd;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy)]
pub(crate) struct Fraction {
    pub(crate) numerator: i128,
    pub(crate) denominator: i128,
}

impl Fraction {
    pub(crate) fn new(nominator: i128, denominator: i128) -> Self {
        assert_ne!(denominator, 0);

        Fraction {
            numerator: nominator,
            denominator,
        }
    }

    pub(crate) fn is_unitary(&self) -> bool {
        self.denominator == 1
    }

    pub(crate) fn display(&self) -> String {
        match self.denominator {
            1 => self.numerator.to_string(),
            _ => format!("{}/{}", self.numerator, self.denominator),
        }
    }

    pub(crate) fn get_ratio(&self) -> (i128, i128) {
        let gcd = gcd(self.numerator, self.denominator);
        (self.numerator / gcd, self.denominator / gcd)
    }
}

impl Debug for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl From<&str> for Fraction {
    fn from(s: &str) -> Self {
        let mut parts = s.split('/');
        Fraction::new(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        )
    }
}

impl From<i128> for Fraction {
    fn from(n: i128) -> Self {
        Fraction::new(n, 1)
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.get_ratio() == other.get_ratio()
    }
}

impl Eq for Fraction {}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_ratio().cmp(&other.get_ratio())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fraction = Fraction::new(3, 4);
        assert_eq!(fraction.numerator, 3);
        assert_eq!(fraction.denominator, 4);
    }

    #[test]
    #[should_panic]
    fn test_new_zero_denominator() {
        Fraction::new(3, 0);
    }

    #[test]
    fn test_is_unitary() {
        let fraction1 = Fraction::new(5, 1);
        assert!(fraction1.is_unitary());

        let fraction2 = Fraction::new(5, 2);
        assert!(!fraction2.is_unitary());
    }

    #[test]
    fn test_display() {
        let fraction1 = Fraction::new(5, 1);
        assert_eq!(fraction1.display(), "5");

        let fraction2 = Fraction::new(3, 4);
        assert_eq!(fraction2.display(), "3/4");
    }

    #[test]
    fn test_from_str() {
        let fraction: Fraction = "3/4".into();
        assert_eq!(fraction.numerator, 3);
        assert_eq!(fraction.denominator, 4);
    }

    #[test]
    fn test_from_i128() {
        let fraction: Fraction = 5.into();
        assert_eq!(fraction.numerator, 5);
        assert_eq!(fraction.denominator, 1);
    }

    #[test]
    fn test_eq() {
        let fraction1 = Fraction::new(3, 4);
        let fraction2 = Fraction::new(6, 8);
        assert_eq!(fraction1, fraction2);
    }

    #[test]
    fn test_cmp() {
        let fraction1 = Fraction::new(3, 4);
        let fraction2 = Fraction::new(5, 6);
        assert!(fraction1 < fraction2);
    }
}

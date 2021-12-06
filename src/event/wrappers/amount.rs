use std::{
    fmt,
    ops::{Add, AddAssign, Neg, SubAssign},
};

use serde::{Deserialize, Serialize};

/// Amount. Several mathematical operations are implemented for this
/// wrapper type for convenience.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Amount(pub f64);

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Amount is displayed with fixed 4 decimal places
        write!(f, "{:.4}", self.0)
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0);
    }
}

impl Neg for Amount {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Add for Amount {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0);
    }
}

impl Amount {
    pub fn is_negative(&self) -> bool {
        self.0 < 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Amount(10.0);
        let b = Amount(5.0);

        assert_eq!(Amount(15.0), a + b);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Amount(10.0);
        let b = Amount(5.0);

        a += b;

        assert_eq!(Amount(15.0), a);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Amount(10.0);
        let b = Amount(5.0);

        a -= b;

        assert_eq!(Amount(5.0), a);
    }

    #[test]
    fn test_neg() {
        let a = Amount(10.0);

        assert_eq!(Amount(-10.0), -a);
    }

    #[test]
    fn test_is_negative() {
        assert_eq!(true, Amount(-1.0).is_negative());
        assert_eq!(false, Amount(0.0).is_negative());
        assert_eq!(false, Amount(1.0).is_negative());
    }

    #[test]
    fn test_display() {
        assert_eq!("0.0000".to_string(), Amount(0.0).to_string());
        assert_eq!("0.9999".to_string(), Amount(0.9999).to_string());
        assert_eq!("1.0000".to_string(), Amount(0.99999).to_string());
        assert_eq!("0.1234".to_string(), Amount(0.1234).to_string());
        assert_eq!("0.1235".to_string(), Amount(0.12345).to_string());
        assert_eq!("1.0000".to_string(), Amount(1.000000001).to_string());
    }
}

use std::{
    fmt,
    ops::{AddAssign, Neg, SubAssign},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Amount(pub f64);

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
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

impl SubAssign for Amount {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0);
    }
}

impl Amount {
    const PRECISION: i32 = 4;

    pub fn round(self) -> Self {
        let base = 10.0_f64.powi(Self::PRECISION);

        Self((self.0 * base).floor() / base)
    }

    pub fn is_negative(&self) -> bool {
        self.0 < 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_amount() {
        assert_eq!(Amount(0.0).round(), Amount(0.0));
        assert_eq!(Amount(0.9).round(), Amount(0.9));
        assert_eq!(Amount(0.99).round(), Amount(0.99));
        assert_eq!(Amount(0.999).round(), Amount(0.999));
        assert_eq!(Amount(0.9999).round(), Amount(0.9999));
        assert_eq!(Amount(0.99999).round(), Amount(0.9999));
    }
}

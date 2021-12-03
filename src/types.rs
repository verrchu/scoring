use std::ops::{AddAssign, SubAssign};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Client(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tx(pub u32);

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Amount(pub f64);

impl AddAssign for Amount {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0)
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0)
    }
}

impl Amount {
    const PRECISION: i32 = 4;

    pub fn round(self) -> Self {
        let base = 10.0f64.powi(Self::PRECISION);

        Self((self.0 * base).floor() / base)
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

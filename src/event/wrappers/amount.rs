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

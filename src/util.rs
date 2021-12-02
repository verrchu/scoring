pub fn round_decimal(x: f64, precision: u8) -> f64 {
    let base = 10.0f64.powi(precision as i32);
    (x * base).floor() / base
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_decimal() {
        assert_eq!(round_decimal(0.99999, 0), 0.0);
        assert_eq!(round_decimal(0.99999, 1), 0.9);
        assert_eq!(round_decimal(0.99999, 2), 0.99);
        assert_eq!(round_decimal(0.99999, 3), 0.999);
        assert_eq!(round_decimal(0.99999, 4), 0.9999);
        assert_eq!(round_decimal(0.99999, 5), 0.99999);
    }
}

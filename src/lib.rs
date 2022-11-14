//! This is a logic gates simulation crate built to demonstrate writing unit tests and integration tests

//unit_tests/src/lib.rs

/// Implements a boolean `and` gate taking as input two bits and returns a bit as output
pub fn and(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 1) => 1,
        _ => 0,
    }
}

/// Implements a boolean `xor` gate taking as input two bits and returns a bit as output
pub fn xor(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 0) | (0, 1) => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::{and, xor};
    #[test]
    fn test_and() {
        assert_eq!(and(0, 1), 0);
        assert_eq!(and(0, 0), 0);
        assert_eq!(and(1, 0), 0);
        assert_eq!(and(1, 1), 1);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(0, 1), 1);
        assert_eq!(xor(0, 0), 0);
        assert_eq!(xor(1, 0), 1);
        assert_eq!(xor(1, 1), 0);
    }
}

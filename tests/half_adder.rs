#![doc(html_logo_url = "https://d30y9cdsu7xlg0.cloudfront.net/png/411962-200.png")]
//unit_tests/src/lib.rs
use unit_test::{and, xor};

pub type Sum = u8;
pub type Carry = u8;

/// This function implements a half adder using primitive gates

pub fn half_adder_input_output() -> Vec<((u8, u8), (Sum, Carry))> {
    vec![
        ((0, 0), (0, 0)),
        ((0, 1), (1, 0)),
        ((1, 0), (1, 0)),
        ((1, 1), (0, 1)),
    ]
}

pub fn half_adder(a: u8, b: u8) -> (Sum, Carry) {
    (xor(a, b), and(a, b))
}

#[test]
pub fn test_half_adder() {
    for (inn, out) in half_adder_input_output() {
        let (a, b) = inn;
        println!("Testing: {}, {} -> {:?}", a, b, out);
        assert_eq!(half_adder(a, b), out)
    }
}

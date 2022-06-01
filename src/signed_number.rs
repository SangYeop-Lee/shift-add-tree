// https://doc.rust-lang.org/std/primitive.u32.html
use std::cmp;
use std::cmp::PartialEq;
use std::ops::{Add, Mul};

/// structure for signed number
/// total_bitcount should be larger than fraction_bitcount and less than 33
#[derive(Debug)]
pub struct SignedNumber {
    mantissa: i32,
    total_bitcount: u32,
    fraction_bitcount: u32,
}

impl SignedNumber {
    pub fn new(mut mantissa: i32, total_bitcount: u32, fraction_bitcount: u32) -> SignedNumber {
        if mantissa & (1 << (total_bitcount - 1)) != 0 {
            for i in total_bitcount..32 {
                mantissa = mantissa | (1 << i);
            }
        }
        SignedNumber {
            mantissa,
            total_bitcount,
            fraction_bitcount,
        }
    }
}

impl PartialEq for SignedNumber {
    fn eq(&self, other: &Self) -> bool {
        self.mantissa == other.mantissa && self.fraction_bitcount == other.fraction_bitcount
    }
}

impl Mul for SignedNumber {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.mantissa * rhs.mantissa,
            self.total_bitcount + rhs.total_bitcount,
            self.fraction_bitcount + rhs.fraction_bitcount,
        )
    }
}

impl Add for SignedNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let integer_bitcount = cmp::max(
            self.total_bitcount - self.fraction_bitcount,
            rhs.total_bitcount - rhs.fraction_bitcount,
        ) + 1;

        let fraction_bitcount = cmp::max(self.fraction_bitcount, rhs.fraction_bitcount);

        Self::new(
            (self.mantissa << (fraction_bitcount - self.fraction_bitcount))
                + (rhs.mantissa << (fraction_bitcount - rhs.fraction_bitcount)),
            integer_bitcount + fraction_bitcount,
            fraction_bitcount,
        )
    }
}

#[cfg(test)]
mod test_signed_number {
    use super::*;

    #[test]
    fn initialize_negative() {
        let negative_number = SignedNumber::new(3, 2, 1);
        assert_eq!(negative_number.mantissa, -1 as i32);
    }

    #[test]
    fn initialize_postivie() {
        let positive_number = SignedNumber::new(3, 3, 1);
        assert_eq!(positive_number.mantissa, 3);
    }

    #[test]
    fn test_mul() {
        // mull 1.1 with 1.11
        // expect 1.11 + 0.111 = 10.101 (7, 3)
        let a = SignedNumber::new(3, 3, 1);
        let b = SignedNumber::new(7, 4, 2);
        let result = a * b;
        assert_eq!(result, SignedNumber::new(21, 7, 3));
    }

    #[test]
    fn test_add() {
        // add 11.1 with 1.11
        // expect 101.01
        let a = SignedNumber::new(7, 4, 1);
        let b = SignedNumber::new(7, 4, 2);
        let result = a + b;
        assert_eq!(result, SignedNumber::new(21, 6, 2));
    }
}

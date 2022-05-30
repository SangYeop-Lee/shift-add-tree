// https://doc.rust-lang.org/std/primitive.u32.html 

pub struct SignedNumber {
    mantissa: u32, // use bitwise operators
    fraction_bitcount: u32,
}

impl SignedNumber {
    pub fn new(mantissa: u32, fraction_bitcount: u32) -> SignedNumber {
        SignedNumber {
            mantissa,
            fraction_bitcount,
        }
    }
}

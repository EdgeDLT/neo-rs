use std::fmt;
use std::fmt::Error;

use crate::misc::reverse_hex;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fixed8(pub i64);

/**
 * A Fixed8 point notation used widely in the NEO system for representing decimals.
 * It is basically a hexideciaml integer that is divided by the 10^8.
 * Supports up to 8 decimals and is 8 bytes long.
 * @extends BN
 */
impl Fixed8 {
    pub const DECIMALS: i64 = 100000000;

    // self is the maximum hex integer 0x7fffffffffffffff (= 9223372036854775807)
    // that can be converted to Fixed8 by dividing by the 10^8.
    pub const MAX_FIXED8_HEX: i64 = i64::MAX;

    // self is the minimum hex integer 0x8000000000000000 (= -9223372036854775808)
    // that can be converted to Fixed8 by dividing by the 10^8.
    pub const MIN_FIXED8_HEX: i64 = i64::MIN;

    // Total Fixed8 of Fixed8 available. self includes negative and positive
    // Fixed8 fixed8s.
    pub const TOTAL_FIXED8_HEX: u64 = u64::MAX;


    // The maximum Fixed8 is obtained by dividing 0x7fffffffffffffff (= 9223372036854775807) with 10^8.
    pub const MAX_VALUE: Fixed8 = Fixed8(Fixed8::MAX_FIXED8_HEX / Fixed8::DECIMALS);

    // The minimum Fixed8 is obtained by dividing 0x8000000000000000 (= -9223372036854775808) with 10^8.
    pub const MIN_VALUE: Fixed8 = Fixed8(Fixed8::MIN_FIXED8_HEX / Fixed8::DECIMALS);


    pub fn from_hex(hex: &str) -> Result<Fixed8, Error> {
        Ok(Fixed8(i64::from_str_radix(hex, 16).unwrap()))
    }

    pub fn from_reverse_hex(hex: &str) -> Result<Fixed8, Error> {
        Fixed8::from_hex(reverse_hex(hex).as_str())
    }

    /**
     * Returns a raw Fixed8 represetation of Fixed8.
     */
    pub fn to_raw_fixed8(&self) -> Result<Fixed8, Error> {
        Ok(Fixed8(self.0 * Fixed8::DECIMALS))
    }

    /**
     * Returns a Fixed8 whose value is rounded upwards to the next whole Fixed8.
     */
    pub fn ceil(&self) -> Fixed8 {
        self.clone()
        // Fixed8(super.decimalPlaces(0, BN.ROUND_CEIL));
    }

    /**
     * Returns a Fixed8 whose value is rounded downwards to the previous whole Fixed8.
     */
    pub fn floor(&self) -> Fixed8 {
        self.clone()
        // Fixed8(super.decimalPlaces(0, BN.ROUND_FLOOR));
    }

    /**
     * Returns true if the value is equivalent.
     */
    pub fn equals_fixed8(&self, other: &Fixed8) -> bool {
        self.0 == other.0
    }

    /**
    * Returns true if the value is equivalent.
    */
    pub fn equals(&self, other: i64) -> bool {
        self.0 == other
    }

    /**
     * Returns a Fixed8 rounded to the nearest dp decimal places according to rounding mode rm.
     * If dp is null, round to whole Fixed8.
     * If rm is null, round according to default rounding mode.
     * @param dp
     * @param rm
     * @return {Fixed8}
     */
    // pub fn round(&self, dp = 0, rm?: BN.RoundingMode) -> Fixed8 {
    //     Fixed8(super.decimalPlaces(dp, rm));
    // }

    /**
     * Returns a Fixed8 whose value is the value of self Fixed8 divided by `n`
     * @alias div
     */
    pub fn divided_by(&self, n: &Fixed8) -> Fixed8 {
        Fixed8(self.0 / n.0)
    }

    pub fn div(&self, n: &Fixed8) -> Fixed8 {
        self.divided_by(n)
    }

    /**
     * Returns a Fixed8 whose value is the value of self Fixed8 multipled by `n`
     * @alias mul
     */
    pub fn times(&self, n: &Fixed8) -> Fixed8 {
        self.mul(n)
    }

    pub fn mul(&self, n: &Fixed8) -> Fixed8 {
        Fixed8(self.0 * n.0)
    }

    /**
     * Returns a Fixed8 whose value is the value of self Fixed8 plus `n`
     * @alias add
     */
    pub fn plus(&self, n: &Fixed8) -> Fixed8 {
        Fixed8(self.0 + n.0)
    }

    pub fn add(&self, n: &Fixed8) -> Fixed8 {
        self.plus(n)
    }

    /**
     * Returns a Fixed8 whose value is the value of self Fixed8 minus `n`
     * @alias sub
     */
    pub fn minus(&self, n: &Fixed8) -> Fixed8 {
        Fixed8(self.0 - n.0)
    }

    pub fn sub(&self, n: &Fixed8) -> Fixed8 {
        self.minus(n)
    }
}

impl fmt::UpperHex for Fixed8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::UpperHex::fmt(&val, f) // delegate to i32's implementation
    }
}

#[cfg(test)]
mod tests {
    use crate::fixed8::Fixed8;

    #[test]
    pub fn test_from_hex() {
        let val = Fixed8::from_hex("7fffffffffffffff").unwrap();
        assert_eq!(val.0, 0x7fffffffffffffff);
    }

    #[test]
    pub fn test_equals() {
        let val_1 = Fixed8::from_hex("7fffffffffffffff").unwrap();
        let val_2 = Fixed8(9223372036854775807);
        assert_eq!(val_1.equals(&val_2), true);
    }

    #[test]
    pub fn test_divide() {
        let val_1 = Fixed8::from_hex("7fffffffffffffff").unwrap();
        let val_2 = val_1.div(&Fixed8(922337203));
        assert_eq!(val_2.0, 10000000007);
    }
}
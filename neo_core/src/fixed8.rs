// import BN from "bigfixed8.js";
// import { reverseHex } from "./misc";

use std::fmt;
use std::fmt::Error;
use crate::misc::reverseHex;

#[derive(Clone,Debug,Eq, PartialEq)]
pub struct fixed8(pub i64);

impl fixed8 {
    pub const DECIMALS: i64 = 100000000;

    // self is the maximum hex integer 0x7fffffffffffffff (= 9223372036854775807)
    // that can be converted to Fixed8 by dividing by the 10^8.
    pub const MAX_FIXED8_HEX: i64 = i64::MAX;

    // self is the minimum hex integer 0x8000000000000000 (= -9223372036854775808)
    // that can be converted to Fixed8 by dividing by the 10^8.
    pub const MIN_FIXED8_HEX: i64 = i64::MIN;

    // Total fixed8 of Fixed8 available. self includes negative and positive
    // Fixed8 fixed8s.
    pub const TOTAL_FIXED8_HEX: u64 = u64::MAX;


    // The maximum Fixed8 is obtained by dividing 0x7fffffffffffffff (= 9223372036854775807) with 10^8.
    pub const MAX_VALUE: fixed8 = fixed8(fixed8::MAX_FIXED8_HEX / fixed8::DECIMALS);

    // The minimum Fixed8 is obtained by dividing 0x8000000000000000 (= -9223372036854775808) with 10^8.
    pub const MIN_VALUE: fixed8 = fixed8(fixed8::MIN_FIXED8_HEX / fixed8::DECIMALS);
}

/**
 * A fixed8 point notation used widely in the NEO system for representing decimals.
 * It is basically a hexideciaml integer that is divided by the 10^8.
 * Supports up to 8 decimals and is 8 bytes long.
 * @extends BN
 */
impl fixed8 {

    pub fn from_hex(&self, hex: &str) -> Result<fixed8, Error> {
        Ok(fixed8(i64::from_str_radix(hex, 16).unwrap()))
    }

    pub fn from_reverse_hex(&self, hex: &str) -> Result<fixed8, Error> {
        self.from_hex(reverseHex(hex).as_str())
    }

    /**
     * Returns a raw fixed8 represetation of Fixed8.
     */
    pub fn to_raw_fixed8(&self) -> Result<fixed8, Error> {
        Ok(fixed8(self.0 * fixed8::DECIMALS))
    }

    /**
     * Returns a Fixed8 whose value is rounded upwards to the next whole fixed8.
     */
    pub fn ceil(&self) -> fixed8 {
            self.clone()
        // fixed8(super.decimalPlaces(0, BN.ROUND_CEIL));
    }

    /**
     * Returns a Fixed8 whose value is rounded downwards to the previous whole fixed8.
     */
    pub fn floor(&self) -> fixed8 {
        self.clone()
        // fixed8(super.decimalPlaces(0, BN.ROUND_FLOOR));
    }

    /**
     * Returns true if the value is equivalent.
     */
    pub fn equals(&self, other: &fixed8) -> bool {
        self.0 == other.0
    }

    /**
     * Returns a Fixed8 rounded to the nearest dp decimal places according to rounding mode rm.
     * If dp is null, round to whole fixed8.
     * If rm is null, round according to default rounding mode.
     * @param dp
     * @param rm
     * @return {Fixed8}
     */
    // pub fn round(&self, dp = 0, rm?: BN.RoundingMode) -> fixed8 {
    //     fixed8(super.decimalPlaces(dp, rm));
    // }

    /**
     * Returns a Fixed8 whose value is the value of self Fixed8 divided by `n`
     * @alias div
     */
    pub fn divided_by(&self, n: &fixed8) -> fixed8 {
        fixed8(self.0/n.0)
    }

    pub fn div(&self, n: &fixed8) -> fixed8 {
        self.divided_by(n)
    }

    /**
     * Returns a Fixed8 whose value is the value of self Fixed8 multipled by `n`
     * @alias mul
     */
    pub fn times(&self, n: &fixed8) -> fixed8 {
        self.mul(n)
    }

    pub fn mul(&self, n: &fixed8) -> fixed8 {
        fixed8(self.0 * n.0)
    }

    /**
     * Returns a Fixed8 whose value is the value of self Fixed8 plus `n`
     * @alias add
     */
    pub fn plus(&self, n: &fixed8) -> fixed8 {
        fixed8(self.0 + n.0)
    }

    pub fn add(&self, n: &fixed8) -> fixed8 {
        self.plus(n)
    }

    /**
     * Returns a Fixed8 whose value is the value of self Fixed8 minus `n`
     * @alias sub
     */
    pub fn minus(&self, n: &fixed8) -> fixed8 {
        fixed8(self.0 - n.0)
    }

    pub fn sub(&self, n: &fixed8) -> fixed8 {
        self.minus(n)
    }
}

impl fmt::UpperHex for fixed8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::UpperHex::fmt(&val, f) // delegate to i32's implementation
    }
}

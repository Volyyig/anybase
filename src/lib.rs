//! High-performance arbitrary base integer conversion library
//!
//! This library provides efficient conversion between arbitrary base numeral systems
//! using a custom character table approach. It supports very large numbers through
//! its custom BigInt implementation that avoids overflow issues.
//!
//! ## Features
//!
//! - Convert between any two bases using custom character tables
//! - Supports arbitrarily large integers
//! - Optimized performance with limb-based arithmetic
//! - No external dependencies
//!
//! ## Examples
//!
//! ```rust
//! use anybase::convert_base;
//!
//! // Convert hexadecimal to octal
//! let result = convert_base("ff", "0123456789abcdef", "01234567").unwrap();
//! assert_eq!(result, "377");
//!
//! // Convert decimal to hexadecimal
//! let result = convert_base("255", "0123456789", "0123456789abcdef").unwrap();
//! assert_eq!(result, "ff");
//! ```
//!
//! ## Performance
//!
//! The library uses a limb-based BigInt implementation with a radix of 1,000,000,000
//! to efficiently handle large numbers while avoiding overflow. Arithmetic operations
//! use u128 intermediates for additional safety.

//! High-performance arbitrary base integer conversion (optimized version)
//! - Uses limb base = 1_000_000_000 (1e9)
//! - Uses u128 as intermediate to avoid overflow

use std::collections::HashMap;

/// Radix for each limb in the BigInt implementation
///
/// Each limb stores values in the range 0..LIMB_RADIX-1
const LIMB_RADIX: u32 = 1_000_000_000;

/// Arbitrary precision integer implementation for base conversion
///
/// This BigInt implementation uses a vector of "limbs" in base LIMB_RADIX
/// to represent arbitrarily large integers. Limbs are stored in little-endian
/// order where limbs[0] is the least significant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BigInt {
    /// Vector of limbs in little-endian order
    ///
    /// Each limb represents a digit in base LIMB_RADIX.
    limbs: Vec<u32>,
}

impl BigInt {
    /// Create a new BigInt with value zero
    ///
    /// # Returns
    ///
    /// A new BigInt instance representing zero
    ///
    /// # Examples
    ///
    /// ```rust
    /// use anybase::BigInt;
    ///
    /// let zero = BigInt::zero();
    /// ```
    pub fn zero() -> Self {
        BigInt { limbs: vec![0] }
    }

    /// Check if this BigInt is zero
    ///
    /// # Returns
    ///
    /// true if the BigInt represents zero, false otherwise
    pub fn is_zero(&self) -> bool {
        self.limbs.len() == 1 && self.limbs[0] == 0
    }

    /// Normalize the BigInt by removing leading zeros
    ///
    /// This internal function removes unnecessary leading zero limbs
    /// to maintain a canonical representation.
    fn normalize(&mut self) {
        while self.limbs.len() > 1 && *self.limbs.last().unwrap() == 0 {
            self.limbs.pop();
        }
    }

    /// Multiply this BigInt by a small value
    ///
    /// # Arguments
    ///
    /// * `small` - The multiplier, must fit in u32
    ///
    /// # Panics
    ///
    /// This function may panic on arithmetic overflow in debug mode,
    /// but handles it gracefully in release mode.
    fn mul_small(&mut self, small: u32) {
        if small == 0 {
            self.limbs.clear();
            self.limbs.push(0);
            return;
        }
        if small == 1 {
            return;
        }
        let mut carry: u128 = 0;
        for limb in &mut self.limbs {
            let prod = (u128::from(*limb)) * u128::from(small) + carry;
            *limb = (prod % LIMB_RADIX as u128) as u32;
            carry = prod / LIMB_RADIX as u128;
        }
        while carry > 0 {
            self.limbs.push((carry % LIMB_RADIX as u128) as u32);
            carry /= LIMB_RADIX as u128;
        }
    }

    /// Add a small value to this BigInt
    ///
    /// # Arguments
    ///
    /// * `small` - The value to add, must fit in u32
    fn add_small(&mut self, small: u32) {
        let mut carry: u128 = u128::from(small);
        for limb in &mut self.limbs {
            if carry == 0 {
                break;
            }
            let sum = u128::from(*limb) + carry;
            *limb = (sum % LIMB_RADIX as u128) as u32;
            carry = sum / LIMB_RADIX as u128;
        }
        while carry > 0 {
            self.limbs.push((carry % LIMB_RADIX as u128) as u32);
            carry /= LIMB_RADIX as u128;
        }
    }

    /// Divide this BigInt by a small value and return the remainder
    ///
    /// Performs in-place division, modifying the BigInt to contain the quotient.
    ///
    /// # Arguments
    ///
    /// * `small` - The divisor, must fit in u32 and be non-zero
    ///
    /// # Returns
    ///
    /// The remainder of the division operation
    ///
    /// # Panics
    ///
    /// Panics if `small` is zero (division by zero)
    fn div_mod_small(&mut self, small: u32) -> u32 {
        // perform division in-place, return remainder
        if small == 0 {
            panic!("division by zero");
        }
        let mut rem: u128 = 0;
        // iterate from high limb to low limb
        for limb in self.limbs.iter_mut().rev() {
            let v = rem * LIMB_RADIX as u128 + u128::from(*limb);
            let q = (v / u128::from(small)) as u32;
            rem = v % u128::from(small);
            *limb = q;
        }
        self.normalize();
        rem as u32
    }
}

/// Parse input string (using src_table) into BigInt
///
/// Converts a string representation in the source base to a BigInt.
///
/// # Arguments
///
/// * `input` - The string to parse
/// * `src_table` - Character table defining the source base
///
/// # Returns
///
/// Result containing the parsed BigInt or an error message
///
/// # Errors
///
/// Returns an error if:
/// - src_table is empty
/// - src_table contains duplicate characters
/// - input contains characters not in src_table
fn parse_to_bigint(input: &str, src_table: &str) -> Result<BigInt, String> {
    if src_table.is_empty() {
        return Err("src_table is empty".to_string());
    }
    let mut map = HashMap::new();
    for (i, ch) in src_table.chars().enumerate() {
        if map.insert(ch, i as u32).is_some() {
            return Err("src_table contains duplicate characters".to_string());
        }
    }

    let mut big = BigInt::zero();
    let src_base = src_table.len() as u32;
    for ch in input.chars() {
        let digit = match map.get(&ch) {
            Some(&d) => d,
            None => return Err(format!("Input character '{}' not found in src_table", ch)),
        };
        // big = big * src_base + digit
        big.mul_small(src_base);
        big.add_small(digit);
    }
    Ok(big)
}

/// Convert BigInt to target character table representation (integers only)
///
/// Converts a BigInt to its string representation in the destination base.
///
/// # Arguments
///
/// * `big` - The BigInt to convert
/// * `dst_table` - Character table defining the destination base
///
/// # Returns
///
/// Result containing the converted string or an error message
///
/// # Errors
///
/// Returns an error if dst_table is empty
fn bigint_to_dst_table(mut big: BigInt, dst_table: &str) -> Result<String, String> {
    if dst_table.is_empty() {
        return Err("dst_table is empty".to_string());
    }
    let dst_chars: Vec<char> = dst_table.chars().collect();
    let dst_base = dst_chars.len() as u32;

    if big.is_zero() {
        return Ok(dst_chars[0].to_string());
    }

    let mut out_chars: Vec<char> = Vec::new();
    while !big.is_zero() {
        let rem = big.div_mod_small(dst_base);
        out_chars.push(dst_chars[rem as usize]);
    }
    out_chars.reverse();
    Ok(out_chars.into_iter().collect())
}

/// Main API: Convert input (src_table) to dst_table
///
/// Converts a number represented as a string in one base to its equivalent
/// in another base, using custom character tables for both bases.
///
/// # Arguments
///
/// * `input` - The input number as a string
/// * `src_table` - Character table for the source base
/// * `dst_table` - Character table for the destination base
///
/// # Returns
///
/// Result containing the converted string or an error message
///
/// # Examples
///
/// ```rust
/// use anybase::convert_base;
///
/// // Convert hexadecimal to binary
/// let result = convert_base("ff", "0123456789abcdef", "01");
/// assert_eq!(result.unwrap(), "11111111");
///
/// // Convert decimal to base-36
/// let result = convert_base("12345", "0123456789", "0123456789abcdefghijklmnopqrstuvwxyz");
/// assert_eq!(result.unwrap(), "9ix");
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - src_table or dst_table is empty
/// - src_table contains duplicate characters
/// - input contains characters not in src_table
pub fn convert_base(input: &str, src_table: &str, dst_table: &str) -> Result<String, String> {
    let b = parse_to_bigint(input, src_table)?;
    bigint_to_dst_table(b, dst_table)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let src = "0123456789abcdef";
        let dst = "01234567";
        assert_eq!(convert_base("ff", src, dst).unwrap(), "377");
        assert_eq!(convert_base("0", src, dst).unwrap(), "0");
    }

    #[test]
    fn test_large() {
        // Large repeated characters, testing performance/correctness
        let src = "0123456789abcdefghijklmnopqrstuvwxyz"; // base36
        let dst = "01"; // to binary
        let input = "z".repeat(200); // very large number
        let out = convert_base(&input, src, dst).unwrap();
        assert!(!out.is_empty());
    }
}
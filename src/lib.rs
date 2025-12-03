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

mod big_int;
mod converter;

pub use converter::*;

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
    let converter = Converter::new(src_table, dst_table);
    converter.convert(input)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    // playground
    fn something() {
    }

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

    #[test]
    fn test_converter() {
        let converter = Converter::new("0123456789", "01");
        let result = converter.convert("10").unwrap();
        assert_eq!(result, "1010");
    }

    #[test]
    fn test_inverse() {
        let converter = Converter::new("0123456789", "01");
        let inv_converter = converter.inverse();
        let result = inv_converter.convert("1010").unwrap();
        assert_eq!(result, "10");
    }

    #[test]
    #[should_panic(expected = "dst_table contains duplicate characters")]
    fn test_duplicate_chars_in_table() {
        convert_base("123", "0123456789", "011").unwrap();
    }

    #[test]
    fn test_same_table() {
        let converter = Converter::new("0123456789", "0123456789");
        let result = converter.convert("12345").unwrap();
        assert_eq!(result, "12345");
    }

}
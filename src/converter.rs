use crate::big_int::BigInt;
use std::collections::HashMap;

/// A converter for transforming numbers between arbitrary bases using custom character tables.
/// 
/// The `Converter` allows conversion of string representations of numbers from one base to another,
/// where each base is defined by a custom character table. Each character in the table represents
/// a digit in that base.
/// 
/// # Examples
/// 
/// ```
/// use anybase::Converter;
/// 
/// let converter = Converter::new("01", "0123456789");
/// let result = converter.convert("1010").unwrap();
/// assert_eq!(result, "10");
/// ```
pub struct Converter<'a> {
    src_table: &'a str,
    dst_table: &'a str,

    src_map: HashMap<char, u32>,
    dst_chars: Vec<char>,
}

impl<'a> Converter<'a> {
    /// Creates a new `Converter` with specified source and destination character tables.
    /// 
    /// # Arguments
    /// 
    /// * `src_table` - A string slice representing the source base character table
    /// * `dst_table` - A string slice representing the destination base character table
    /// 
    /// # Panics
    /// 
    /// Panics if either table is empty or contains duplicate characters.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use anybase::Converter;
    /// let converter = Converter::new("01", "0123456789");
    /// ```                 
    pub fn new(src_table: &'a str, dst_table: &'a str) -> Self {
        Converter {
            src_table,
            dst_table,
            src_map: {
                if src_table.is_empty() {
                    panic!("src_table is empty");
                }
                let mut map = HashMap::new();
                for (i, ch) in src_table.chars().enumerate() {
                    if map.insert(ch, i as u32).is_some() {
                        panic!("src_table contains duplicate characters");
                    }
                }
                map
            },
            dst_chars: {
                if dst_table.is_empty() {
                    panic!("dst_table is empty");
                }
                let chars: Vec<char> = dst_table.chars().collect();
                let unique_count = chars
                    .iter()
                    .collect::<std::collections::HashSet<_>>()
                    .len();
                if unique_count != chars.len() {
                    panic!("dst_table contains duplicate characters");
                }
                chars
            },
        }
    }

    /// Creates an inverse converter with swapped source and destination tables.
    /// 
    /// # Returns
    /// 
    /// A new `Converter` instance with source and destination tables swapped.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use anybase::Converter;
    /// let converter = Converter::new("01", "0123456789");
    /// let inverse_converter = converter.inverse();
    /// assert_eq!(converter.src_table(), inverse_converter.dst_table());
    /// assert_eq!(converter.dst_table(), inverse_converter.src_table());
    /// ```
    pub fn inverse(&self) -> Self {
        Converter::new(self.dst_table, self.src_table)
    }

    /// Converts an input string from source base to destination base.
    /// 
    /// # Arguments
    /// 
    /// * `input` - The string to convert, using characters from the source table
    /// 
    /// # Returns
    /// 
    /// `Ok(String)` containing the converted value in destination base characters,
    /// or `Err(String)` with an error message if conversion fails.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use anybase::Converter;
    /// let converter = Converter::new("01", "0123456789");
    /// let result = converter.convert("1010").unwrap();
    /// assert_eq!(result, "10");
    /// ```
    pub fn convert(&self, input: &str) -> Result<String, String> {
        let b = self.parse_to_bigint(input)?;
        self.bigint_to_dst_table(b)
    }

    /// Returns the source character table.
    /// 
    /// # Returns
    /// 
    /// A string slice representing the source base character table.
    pub fn src_table(&self) -> &str {
        self.src_table
    }

    /// Returns the destination character table.
    /// 
    /// # Returns
    /// 
    /// A string slice representing the destination base character table.
    pub fn dst_table(&self) -> &str {
        self.dst_table
    }

    /// Returns the size of the source base.
    /// 
    /// # Returns
    /// 
    /// The number of characters in the source character table, which represents the source base.
    pub fn src_base(&self) -> usize {
        self.src_table.chars().count()
    }

    /// Returns the size of the destination base.
    /// 
    /// # Returns
    /// 
    /// The number of characters in the destination character table, which represents the destination base.
    pub fn dst_base(&self) -> usize {
        self.dst_table.chars().count()
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
    fn parse_to_bigint(&self, input: &str) -> Result<BigInt, String> {
        let mut big = BigInt::zero();
        let src_base = self.src_table.chars().count() as u32;
        for ch in input.chars() {
            let digit = match self.src_map.get(&ch) {
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
    /// Returns an error if:
    /// - dst_table is empty
    /// - dst_table contains duplicate characters
    fn bigint_to_dst_table(&self, mut big: BigInt) -> Result<String, String> {
        if self.dst_table.is_empty() {
            return Err("dst_table is empty".to_string());
        }

        let dst_base = self.dst_chars.len() as u32;

        if big.is_zero() {
            return Ok(self.dst_chars[0].to_string());
        }

        let mut out_chars: Vec<char> = Vec::new();
        while !big.is_zero() {
            let rem = big.div_mod_small(dst_base);
            out_chars.push(self.dst_chars[rem as usize]);
        }
        out_chars.reverse();
        Ok(out_chars.into_iter().collect())
    }
}

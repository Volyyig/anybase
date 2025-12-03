/// Radix for each limb in the BigInt implementation
///
/// Type alias for the limb data type used in BigInt implementation
/// 
/// LimbType is the type used for each limb in the BigInt implementation.
type LimbType = u32;
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
    limbs: Vec<LimbType>,
}

impl BigInt {
    /// Create a new BigInt with value zero
    ///
    /// # Returns
    ///
    /// A new BigInt instance representing zero
    ///
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
    pub fn mul_small(&mut self, small: u32) {
        if small == 0 {
            self.limbs.clear();
            self.limbs.push(0);
            return;
        }
        if small == 1 {
            return;
        }
        let mut carry: u64 = 0;
        for limb in &mut self.limbs {
            let prod = (u64::from(*limb)) * u64::from(small) + carry;
            *limb = (prod % LimbType::MAX as u64) as u32;
            carry = prod / LimbType::MAX as u64;
        }
        while carry > 0 {
            self.limbs.push((carry % LimbType::MAX as u64) as u32);
            carry /= LimbType::MAX as u64;
        }
    }

    /// Add a small value to this BigInt
    ///
    /// # Arguments
    ///
    /// * `small` - The value to add, must fit in u32
    pub fn add_small(&mut self, small: u32) {
        let mut carry: u64 = u64::from(small);
        for limb in &mut self.limbs {
            if carry == 0 {
                break;
            }
            let sum = u64::from(*limb) + carry;
            *limb = (sum % LimbType::MAX as u64) as u32;
            carry = sum / LimbType::MAX as u64;
        }
        while carry > 0 {
            self.limbs.push((carry % LimbType::MAX as u64) as u32);
            carry /= LimbType::MAX as u64;
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
    pub fn div_mod_small(&mut self, small: u32) -> u32 {
        // perform division in-place, return remainder
        if small == 0 {
            panic!("division by zero");
        }
        let mut rem: u64 = 0;
        // iterate from high limb to low limb
        for limb in self.limbs.iter_mut().rev() {
            let v = rem * LimbType::MAX as u64 + u64::from(*limb);
            let q = (v / u64::from(small)) as u32;
            rem = v % u64::from(small);
            *limb = q;
        }
        self.normalize();
        rem as u32
    }
}

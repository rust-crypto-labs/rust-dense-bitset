use crate::bitset::BitSet;

use std::fmt;

/// Overload of &, &=, |, |=, ^, ^=, !, <<, <<=, >>, >>=
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

/// Provides an efficient and compact bitset implementation for up to 64 bits
#[derive(Copy, Clone, Hash)]
pub struct DenseBitSet {
    state: u64,
}

impl DenseBitSet {
    /// Generates a bitset from an integer (little endian convention)
    pub fn from_integer(i: u64) -> Self {
        Self { state: i }
    }

    /// Generates a bitset from a string and a base (little endian convention)
    pub fn from_string(s: &str, base: u32) -> Self {
        assert!( 2 <= base && base <= 32, "Only supports base from 2 to 32");
        let val = u64::from_str_radix(s,base);
        let res: u64 = val.expect("Failed to parse string");
        Self { state: res }
    }

    /// Returns an integer representing the bitset (little endian convention)
    pub fn to_integer(&self) -> u64 {
        self.state
    }

    /// Returns an integer representation of the bitsting starting at the given `position` with given `length` (little endian convention)
    pub fn extract(&self, position: usize, length: usize) -> u64 {
        assert!(
            position + length <= 64,
            "This implementation is currently limited to 64 bit bitsets."
        );
        assert!(length > 0, "Cannot extract a zero-width slice.");
        if length < 64 {
            (self.state >> position) & ((1 << length) - 1)
        } else {
            // This special branch is to avoid overflowing when masking
            (self.state >> position)
        }
    }

    /// Returns nothing, mutates the DenseBitSet to insert a value at the given `position` with given `length` (little endian convention)
    /// if length is greater than the value's length, this will add the difference in size as zeros to the left of value
    pub fn insert(&mut self, position: usize, length: usize, value: u64) {
        assert!(
            position + length <= 64,
            "This implementation is currently limited to 64 bit bitsets."
        );
        assert!(length > 0, "Cannot insert zero-width slice");
        if length < 64 {
            let mut u = u64::max_value();
            u ^= ((1 << length) - 1) << position;
            self.state &= u;
            self.state |= value << position;
        } else {
            self.state = value;
        }
    }

    /// Returns true if all bits are set to true
    pub fn all(&self) -> bool {
        self.state == u64::max_value()
    }

    /// Returns true if any of the bits are set to true
    pub fn any(&self) -> bool {
        self.state > 0
    }

    /// Returns true if none of the bits are set to true
    pub fn none(&self) -> bool {
        self.state == 0
    }
}

impl BitSet for DenseBitSet {
    fn set_bit(&mut self, position: usize, value: bool) {
        assert!(
            position < 64,
            "This implementation is currently limited to 64 bit bitsets."
        );
        if value {
            self.state |= 1 << position
        } else {
            self.state &= !(1 << position)
        }
    }

    fn get_bit(&self, position: usize) -> bool {
        assert!(
            position < 64,
            "This implementation is currently limited to 64 bit bitsets."
        );

        (self.state >> position) & 1 == 1
    }

    fn get_weight(&self) -> u32 {
        self.state.count_ones()
    }

    fn reset(&mut self) {
        self.state = 0
    }

    fn flip(&mut self) {
        self.state = !self.state
    }
}

impl fmt::Debug for DenseBitSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut bss = String::new();

        for i in 0..64 {
            bss += if self.get_bit(63-i) { "1" } else { "0" };
        }

        write!(f, "0b{} ({})", bss, self.to_integer())
    }
}

impl PartialEq for DenseBitSet {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.to_integer()
    }
}

impl Eq for DenseBitSet {}

impl BitAnd for DenseBitSet {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self {
            state: self.state & rhs.state,
        }
    }
}

impl BitAndAssign for DenseBitSet {
    fn bitand_assign(&mut self, rhs: Self) {
        self.state &= rhs.state;
    }
}

impl BitOr for DenseBitSet {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self {
            state: self.state | rhs.state,
        }
    }
}

impl BitOrAssign for DenseBitSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.state |= rhs.state;
    }
}

impl BitXor for DenseBitSet {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self {
            state: self.state ^ rhs.state,
        }
    }
}

impl BitXorAssign for DenseBitSet {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.state ^= rhs.state;
    }
}

impl Not for DenseBitSet {
    type Output = Self;
    fn not(self) -> Self {
        Self { state: !self.state }
    }
}

impl Shl<usize> for DenseBitSet {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self {
        if rhs >= 64 {
            Self { state: 0 }
        } else {
            Self {
                state: self.state << rhs,
            }
        }
    }
}

impl ShlAssign<usize> for DenseBitSet {
    fn shl_assign(&mut self, rhs: usize) {
        if rhs >= 64 {
            self.reset();
        } else {
            self.state <<= rhs;
        }
    }
}

impl Shr<usize> for DenseBitSet {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self {
        if rhs >= 64 {
            Self { state: 0 }
        } else {
            Self {
                state: self.state >> rhs,
            }
        }
    }
}

impl ShrAssign<usize> for DenseBitSet {
    fn shr_assign(&mut self, rhs: usize) {
        if rhs >= 64 {
            self.reset();
        } else {
            self.state >>= rhs;
        }
    }
}
use crate::bitset::BitSet;

use std::fmt;
use std::hash::{Hash, Hasher};

/// Overload of &, &=, |, |=, ^, ^=, !, <<, <<=, >>, >>=
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

/// Provides an efficient and compact `BitSet` implementation for up to 64 bits
#[derive(Copy, Clone, Default)]
pub struct DenseBitSet {
    state: u64,
}

impl DenseBitSet {
    /// Returns a new empty bitset
    pub fn new() -> Self {
        Self { state: 0 }
    }

    /// Generates a bitset from an integer (little endian convention)
    pub fn from_integer(i: u64) -> Self {
        Self { state: i }
    }

    /// Generates a bitset from a string and a base (little endian convention)
    ///
    /// The `base` must be an integer between 2 and 32
    pub fn from_string(s: &str, base: u32) -> Self {
        assert!(2 <= base && base <= 32, "Only supports base from 2 to 32");
        let val = u64::from_str_radix(s, base);
        let res: u64 = val.expect("Failed to parse string");
        Self { state: res }
    }

    /// Returns an integer representing the bitset (little endian convention)
    pub fn to_integer(self) -> u64 {
        self.state
    }

    /// Returns an integer representation of the bitsting starting at the given `position` with given `length` (little endian convention)
    pub fn extract(self, position: usize, length: usize) -> u64 {
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

    /// Returns nothing, mutates the `DenseBitSet` to insert `value` at the given `position`.
    ///
    /// Note that `value` is treated as a `length`-bit integer (little endian convention);
    /// if necessary, `value` is padded with zeros (or truncated) to be of the correct length
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

    /// Returns `true` if and only if all bits are set to `true`
    pub fn all(self) -> bool {
        self.state == u64::max_value()
    }

    /// Returns `true` if at least one of the bits is set to `true`
    pub fn any(self) -> bool {
        self.state > 0
    }

    /// Returns `true` if all the bits are set to `false`
    pub fn none(self) -> bool {
        !self.any()
    }

    /// Returns a bit-reversed `DenseBitSet`
    pub fn reverse(self) -> Self {
        let mut v = self.state;
        v = ((v >> 1) & (0x5555555555555555 as u64)) | ((v & (0x5555555555555555 as u64)) << 1);
        v = ((v >> 2) & (0x3333333333333333 as u64)) | ((v & (0x3333333333333333 as u64)) << 2);
        v = ((v >> 4) & (0x0F0F0F0F0F0F0F0F as u64)) | ((v & (0x0F0F0F0F0F0F0F0F as u64)) << 4);

        Self {
            state: v.swap_bytes(),
        }
    }

    /// Right rotation of `shift` bits
    ///
    /// Shifts the bits to the right, wrapping the truncated bits to the end of the set
    pub fn rotr(&mut self, shift: u32) {
        self.state = self.state.rotate_right(shift);
    }

    /// Left rotation of `shift` bits
    ///
    /// Shifts the bits to the left, wrapping the truncated bits to the beginning of the set
    pub fn rotl(&mut self, shift: u32) {
        self.state = self.state.rotate_left(shift);
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

    fn to_string(self) -> String {
        format!("{:064b}", self.state)
    }
}

impl fmt::Debug for DenseBitSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:64b}", self.to_integer())
    }
}

impl PartialEq for DenseBitSet {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.to_integer()
    }
}

impl Eq for DenseBitSet {}

impl Hash for DenseBitSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}

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

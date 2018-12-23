use crate::bitset::BitSet;

use std::fmt;
use std::hash::{Hash, Hasher};

/// Overload of &, &=, |, |=, ^, ^=, !, <<, <<=, >>, >>=
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

/// Provides an efficient and compact `BitSet` implementation for up to 64 bits.
/// 
/// This structure implements `BitSet, Clone, Copy, Default, Debug, Hash, PartialEq, Eq` and bit operations.
#[derive(Copy, Clone, Default)]
pub struct DenseBitSet {
    state: u64,
}

impl DenseBitSet {
    /// Returns a new empty `DenseBitSet`.
    ///
    /// # Example
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// 
    /// let mut bs = DenseBitSet::new();
    /// ```
    pub fn new() -> Self {
        Self { state: 0 }
    }

    /// Generates a bitset from an integer (little endian convention).
    ///
    /// # Example
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// 
    /// let bs = DenseBitSet::from_integer(1234567890);
    /// ```
    pub fn from_integer(i: u64) -> Self {
        Self { state: i }
    }

    /// Generates a bitset from a string and a base (little endian convention).
    ///
    /// The `base` must be an integer between 2 and 32.
    ///
    /// # Example
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// 
    /// let mut bs1 = DenseBitSet::from_string("101010", 2);
    /// let mut bs2 = DenseBitSet::from_string("2a", 16);
    ///
    /// assert_eq!(bs1,bs2);
    /// ```
    /// 
    /// # Panics
    ///  
    /// This function will panic if an incorrect `base` is provided or if invalid
    /// characters are found when parsing.
    pub fn from_string(s: &str, base: u32) -> Self {
        assert!(2 <= base && base <= 32, "Only supports base from 2 to 32");
        let val = u64::from_str_radix(s, base);
        let res: u64 = val.expect("Failed to parse string");
        Self { state: res }
    }

    /// Returns an integer representing the bitset (little endian convention).
    ///
    /// # Example
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// 
    /// let bs = DenseBitSet::from_integer(1234);
    ///
    /// assert_eq!(bs.to_integer(), 1234);
    /// ```
    pub fn to_integer(self) -> u64 {
        self.state
    }

    /// Returns an integer representation of the bitset starting at the given `position` with given `length` (little endian convention).
    ///
    /// # Example
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// 
    /// let bs = DenseBitSet::from_integer(0b11110101010010);
    /// let value = bs.extract(5,6);
    ///
    /// assert_eq!(value, 42);
    /// ```
    /// 
    /// # Panics
    /// This function will panic if `length` is zero or if one tries to
    /// access a bit beyond the 64 bit limit (i.e., `position + length > 64`).
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
    /// if necessary, `value` is padded with zeros (or truncated) to be of the correct length.
    ///
    /// # Example
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// 
    /// let mut bs = DenseBitSet::new();
    /// bs.insert(10, 8, 0b10101011);
    /// bs.insert(3,1,1);
    ///
    /// assert_eq!(bs.to_integer(), 0b101010110000001000)
    /// ```
    /// 
    /// # Panics
    /// This function will panic if `length` is zero, or if one tries to
    /// insert a bit beyond the 64 bit limit (i.e. `position + length > 64`)
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

    /// Returns `true` if and only if all bits are set to `true`.
    ///
    /// # Example
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// use rust_dense_bitset::BitSet;
    /// 
    /// let mut bs = DenseBitSet::from_integer(u64::max_value());
    ///
    /// assert!(bs.all());
    ///
    /// bs.set_bit(28,false);
    /// bs.all(); // -> false
    /// ```
    pub fn all(self) -> bool {
        self.state == u64::max_value()
    }

    /// Returns `true` if at least one of the bits is set to `true`.
    ///
    /// # Example
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// use rust_dense_bitset::BitSet;
    /// 
    /// let mut bs = DenseBitSet::from_integer(2048);
    ///
    /// assert!(bs.any());
    ///
    /// bs.set_bit(11,false);
    /// bs.any(); // -> false
    /// ```
    pub fn any(self) -> bool {
        self.state > 0
    }

    /// Returns `true` if all the bits are set to `false`.
    ///
    /// # Example
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// use rust_dense_bitset::BitSet;
    /// 
    /// let mut bs = DenseBitSet::from_integer(2048);
    /// bs.set_bit(11,false);
    ///
    /// assert!(bs.none());
    /// ```
    pub fn none(self) -> bool {
        !self.any()
    }

    /// Returns a bit-reversed `DenseBitSet`.
    ///
    /// This method is using a constant time bit reversal algorithm for 64 bits integers.
    ///
    /// # Example
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// 
    /// let bs = DenseBitSet::from_integer(0b11110001);
    /// let bs2 = bs.reverse();
    ///
    /// assert_eq!(bs2.to_integer(), 0b1000111100000000000000000000000000000000000000000000000000000000);
    /// ```
    pub fn reverse(self) -> Self {
        let mut v = self.state;
        v = ((v >> 1) & (0x5555555555555555 as u64)) | ((v & (0x5555555555555555 as u64)) << 1);
        v = ((v >> 2) & (0x3333333333333333 as u64)) | ((v & (0x3333333333333333 as u64)) << 2);
        v = ((v >> 4) & (0x0F0F0F0F0F0F0F0F as u64)) | ((v & (0x0F0F0F0F0F0F0F0F as u64)) << 4);

        Self {
            state: v.swap_bytes(),
        }
    }

    /// Right rotation of `shift` bits.
    ///
    /// Shifts the bits to the right, wrapping the truncated bits to the end of the bitset.
    /// 
    /// The rotation is done in-place, so the bitset needs to be mutable.
    /// # Example 
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// 
    /// let mut bs = DenseBitSet::from_integer(0b111000111000111000111);
    /// bs.rotr(10);
    /// 
    /// assert_eq!(bs.to_integer(), 0b111000111000000000000000000000000000000000000000000011100011100 );
    /// ```
    pub fn rotr(&mut self, shift: u32) {
        self.state = self.state.rotate_right(shift);
    }

    /// Left rotation of `shift` bits.
    ///
    /// Shifts the bits to the left, wrapping the truncated bits to the beginning of the bitset.
    ///  
    /// The rotation is done in place, so the bitset needs to be mutable.
    /// # Example 
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// 
    /// let mut bs = DenseBitSet::from_integer(0b111000111000111000111);
    /// bs.rotl(10);
    /// 
    /// assert_eq!(bs.to_integer(), 0b1110001110001110001110000000000 );
    /// ```
    pub fn rotl(&mut self, shift: u32) {
        self.state = self.state.rotate_left(shift);
    }
}

/// This is a compact implementation of the `BitSet` trait over a 64-bit word (which is the native
/// word size for many architectures), allowing for efficient operations and compact memory usage. 
/// 
/// Modifiers and accessors are boundary checked to ensure that operations remain within that 64 bit range.
///
/// Note: The `BitSet` trait must be in scope in order to use methods from this trait. 
impl BitSet for DenseBitSet {
    /// Sets the bit at index `position` to `value`. 
    /// The bitset needs to be mutable for this operation to succeed.
    ///
    /// # Example
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// use rust_dense_bitset::BitSet;
    /// 
    /// let mut bs = DenseBitSet::new();
    /// bs.set_bit(25, true); // This sets the bit at index 25 , hence the 26th bit -> 2^25
    ///
    /// assert_eq!(bs.to_integer(), 33554432);
    /// ```
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

    /// Get the bit at index `position`.
    ///
    /// # Example
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// use rust_dense_bitset::BitSet;
    /// 
    /// let bs = DenseBitSet::from_integer(65536);
    ///
    /// assert!(bs.get_bit(16));
    /// ```
    fn get_bit(&self, position: usize) -> bool {
        assert!(
            position < 64,
            "This implementation is currently limited to 64 bit bitsets."
        );

        (self.state >> position) & 1 == 1
    }

    /// Returns the bitset's Hamming weight (in other words, the number of bits set to true).
    ///
    /// # Example
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// use rust_dense_bitset::BitSet;
    /// 
    /// let bs = DenseBitSet::from_integer(0b01100100111010);
    ///
    /// println!("{}", bs.get_weight()); // -> 7
    /// ```
    fn get_weight(&self) -> u32 {
        self.state.count_ones()
    }

    /// This resets the bitset to its empty state.
    /// (The bitset must be mutable for this operation).
    ///
    /// # Example
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// use rust_dense_bitset::BitSet;
    /// 
    /// let mut bs = DenseBitSet::from_integer(1234567890);
    /// bs.reset();
    ///
    /// assert!(bs.none());
    /// ```
    fn reset(&mut self) {
        self.state = 0
    }

    /// Returns a representation of the bitset as a `String`.
    ///
    /// # Example
    /// ```
    /// use rust_dense_bitset::DenseBitSet;
    /// use rust_dense_bitset::BitSet;
    /// 
    /// let bs = DenseBitSet::from_integer(68719481088);
    ///
    /// println!("{}", bs.to_string()) // -> "0000000000000000000000000001000000000000000000000001000100000000"
    /// ```
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

use std::fmt;

/// Overload of &, &=, |, |=, ^, ^=, !, <<, <<=, >>, >>=
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

pub trait BitSet {
    /// Sets the value of the bit at position `position` to `value`
    fn set_bit(&mut self, position: usize, value: bool);

    /// Gets the value of the bit at position `position`
    fn get_bit(&self, position: usize) -> bool;

    /// Returns the bitset's Hamming weight
    fn get_weight(&self) -> u32;

    /// Resets the bitset
    fn reset(&mut self);

    /// Flips the bitset (0 becomes 1 and vice versa)
    fn flip(&mut self);
}

#[derive(Copy, Clone)]
pub struct DenseBitSet {
    state: u64,
}

impl DenseBitSet {
    /// Generates a bitset from an integer (little endian convention)
    pub fn from_integer(i: u64) -> Self {
        Self { state: i }
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
            bss += if self.get_bit(i) { "1" } else { "0" };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_crash_on_insertion() {
        let mut bs = DenseBitSet::from_integer(0);
        for i in 0..63 {
            bs.set_bit(i, true);
        }
    }

    #[test]
    fn test_hamming_weight() {
        let bs1 = DenseBitSet::from_integer(0);
        let bs2 = DenseBitSet::from_integer(1234567890);
        let bs3 = DenseBitSet::from_integer(u64::max_value());

        assert_eq!(bs1.get_weight(), 0);
        assert_eq!(bs2.get_weight(), 12);
        assert_eq!(bs3.get_weight(), 64);
    }

    #[test]
    fn no_crash_on_read() {
        let bs = DenseBitSet::from_integer(1234567890);
        let mut hw = 0;
        for i in 0..64 {
            hw += if bs.get_bit(i) { 1 } else { 0 };
        }
        assert_eq!(hw, 12, "Error: mismatch between expected and read bits.");
    }

    #[test]
    #[should_panic]
    fn catch_get_overflow() {
        let bs = DenseBitSet::from_integer(1234567890);
        let _r = bs.get_bit(64); // Should panic: bit #64 is out of bounds
    }

    #[test]
    #[should_panic]
    fn catch_set_overflow() {
        let mut bs = DenseBitSet::from_integer(1234567890);
        bs.set_bit(64, true); // Should panic: bit #64 is out of bounds
    }

    #[test]
    fn read_write_test() {
        let mut bs1 = DenseBitSet::from_integer(1234567890);
        let mut bs2 = DenseBitSet::from_integer(0);

        for i in 0..64 {
            bs2.set_bit(i, bs1.get_bit(i));
            bs1.set_bit(i, false);
        }
        assert_eq!(bs1.to_integer(), 0);
        assert_eq!(bs2.to_integer(), 1234567890);
    }

    #[test]
    fn full_extract() {
        let bs = DenseBitSet::from_integer(1234567890);
        assert_eq!(bs.extract(0, 64), 1234567890);
    }

    #[test]
    fn partial_extracts() {
        let bs = DenseBitSet::from_integer(1234567890);
        let e1 = bs.extract(1, 63);
        let e2 = bs.extract(0, 8);
        let e3 = bs.extract(5, 14);

        assert_eq!(e1, 617283945);
        assert_eq!(e2, 210);
        assert_eq!(e3, 12310);
    }

    #[test]
    #[should_panic]
    fn catch_extract_zero_width() {
        let bs = DenseBitSet::from_integer(1234567890);
        let _r = bs.extract(12, 0); // Should panic: 0 is passed as 2nd argument
    }

    #[test]
    #[should_panic]
    fn catch_extract_overflow() {
        let bs = DenseBitSet::from_integer(1234567890);
        let _r = bs.extract(12, 55); // Should panic: 12+55 exceeds the 64 bit boundary
    }

    #[test]
    fn full_insert() {
        let mut bs = DenseBitSet::from_integer(1234567890);
        bs.insert(0, 64, 9876);
        assert_eq!(bs.to_integer(), 9876);
    }

    #[test]
    fn partial_insert() {
        let mut bs = DenseBitSet::from_integer(0b11101101010100011001);
        bs.insert(12, 2, 0b10);
        assert_eq!(bs.to_integer(), 0b11101110010100011001)
    }

    #[test]
    #[should_panic]
    fn catch_insert_zero_width() {
        let mut bs = DenseBitSet::from_integer(1234567890);
        bs.insert(0, 0, 12); // Should panic: 0 is passed as 2nd argument
    }

    #[test]
    #[should_panic]
    fn catch_insert_overflow() {
        let mut bs = DenseBitSet::from_integer(1234567890);
        bs.insert(12, 55, 79885); // Should panic: 12+55 exceeds the 64 bit boundary
    }

    #[test]
    fn test_equality_trait() {
        let bs1 = DenseBitSet::from_integer(1234567890);
        let mut bs2 = DenseBitSet::from_integer(1234567891);

        bs2.set_bit(0, false); // The two bitsets are now equal
        assert_eq!(bs1, bs2);
    }

    #[test]
    fn test_reset() {
        let mut bs = DenseBitSet::from_integer(1234567890);
        bs.reset();
        assert_eq!(bs.to_integer(), 0);
    }

    #[test]
    fn test_all() {
        let bs = DenseBitSet::from_integer(u64::max_value());
        assert_eq!(bs.all(), true);
    }

    #[test]
    fn test_any() {
        let bs = DenseBitSet::from_integer(1234567890);
        assert_eq!(bs.any(), true);
    }

    #[test]
    fn test_none() {
        let bs = DenseBitSet::from_integer(0);
        assert_eq!(bs.none(), true);
    }

    #[test]
    fn test_bitand() {
        let bs1 = DenseBitSet::from_integer(0b10101);
        let bs2 = DenseBitSet::from_integer(0b11100);
        let bs3 = bs1 & bs2;
        assert_eq!(bs3.to_integer(), 0b10100);
    }

    #[test]
    fn test_bitand_assign() {
        let bs1 = DenseBitSet::from_integer(0b11000);
        let mut bs2 = DenseBitSet::from_integer(0b1001);
        bs2 &= bs1;
        assert_eq!(bs2.to_integer(), 0b1000);
    }

    #[test]
    fn test_bitor() {
        let bs1 = DenseBitSet::from_integer(0b10101);
        let bs2 = DenseBitSet::from_integer(0b11100);
        let bs3 = bs1 | bs2;
        assert_eq!(bs3.to_integer(), 0b11101);
    }

    #[test]
    fn test_bitor_assign() {
        let bs1 = DenseBitSet::from_integer(0b11000);
        let mut bs2 = DenseBitSet::from_integer(0b1001);
        bs2 |= bs1;
        assert_eq!(bs2.to_integer(), 0b11001);
    }

    #[test]
    fn test_bitxor() {
        let bs1 = DenseBitSet::from_integer(0b10101);
        let bs2 = DenseBitSet::from_integer(0b11100);
        let bs3 = bs1 ^ bs2;
        assert_eq!(bs3.to_integer(), 0b1001);
    }

    #[test]
    fn test_bitxor_assign() {
        let bs1 = DenseBitSet::from_integer(0b11000);
        let mut bs2 = DenseBitSet::from_integer(0b1001);
        bs2 ^= bs1;
        assert_eq!(bs2.to_integer(), 0b10001);
    }

    #[test]
    fn test_not() {
        let mut bs1 = DenseBitSet::from_integer(1234567890);
        let bs2 = !bs1;
        bs1.flip();
        assert_eq!(bs1, bs2);
    }

    #[test]
    fn test_shl() {
        let bs1 = DenseBitSet::from_integer(0b10101);
        let bs2 = bs1 << 6;
        assert_eq!(bs2.to_integer(), 0b10101000000);
        let bs3 = bs1 << 65;
        assert!(bs3.none());
    }

    #[test]
    fn test_shl_assign() {
        let mut bs1 = DenseBitSet::from_integer(0b11000);
        bs1 <<= 6;
        assert_eq!(bs1.to_integer(), 0b11000000000);
        bs1 <<= 65;
        assert!(bs1.none());
    }

    #[test]
    fn test_shr() {
        let bs1 = DenseBitSet::from_integer(0b101011111111111101);
        let bs2 = bs1 >> 6;
        assert_eq!(bs2.to_integer(), 0b101011111111);
        let bs3 = bs1 >> 65;
        assert!(bs3.none());
    }

    #[test]
    fn test_shr_assign() {
        let mut bs1 = DenseBitSet::from_integer(0b1001111001101);
        bs1 >>= 6;
        assert_eq!(bs1.to_integer(), 0b1001111);
        bs1 >>= 65;
        assert!(bs1.none());
    }

}

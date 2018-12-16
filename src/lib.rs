use std::fmt;

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

pub struct DenseBitSet {
  state: u64
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
    assert!(position + length <= 64, "This implementation is currently limited to 64 bit bitsets.");
    assert!(length > 0, "Cannot extract a zero-width slice.");
    if length < 64 {
      (self.state >> position) & ((1 << length) - 1)
    }
    else {
      // This special branch is to avoid overflowing when masking
      (self.state >> position)
    }
  }
}

impl BitSet for DenseBitSet {
  fn set_bit(&mut self, position: usize, value: bool) {
    assert!(position < 64, "This implementation is currently limited to 64 bit bitsets.");
    if value {
      self.state |= 1 << position
    }
    else {
      self.state &= !(1<<position)
    }
  }

  fn get_bit(&self, position: usize) -> bool {
    assert!(position < 64, "This implementation is currently limited to 64 bit bitsets.");

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
}

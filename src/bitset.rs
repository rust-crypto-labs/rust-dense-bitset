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

    /// Produces a string representation of the bitset (little endian), aligned with 64 bits and with leading zeroes
    fn to_string(self) -> String;
}
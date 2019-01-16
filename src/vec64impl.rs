use crate::bitset::BitSet;
use crate::u64impl::DenseBitSet;

use std::cmp::{max, min};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Overload of &, &=, |, |=, ^, ^=, !, <<, <<=, >>, >>=
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

/// Provides a dense `BitSet` implementation (only limited by available memory)
///
/// Internally, a `Vec<u64>` data structure is used to store information.
///
/// This structure implements `BitSet, Clone, Default, Debug, Hash, PartialEq, Eq` and bit operations.
#[derive(Default, Clone)]
pub struct DenseBitSetExtended {
    state: Vec<u64>,
    size: usize,
}

impl DenseBitSetExtended {
    /// Returns a new empty `DenseBitsetExtended`
    ///    
    /// # Example
    /// ```
    /// # use rust_dense_bitset::DenseBitSetExtended;
    /// let bs = DenseBitSetExtended::new();
    /// ```
    pub fn new() -> Self {
        Self {
            state: vec![],
            size: 0,
        }
    }

    /// Returns an empty `DenseBitsetExtended` with pre-allocated memory of `size` bits.
    ///
    /// This is useful to avoid additional allocations is situations where the bitset's
    /// space requirements are known in advance.
    ///
    /// # Example
    /// ```
    /// # use rust_dense_bitset::{BitSet, DenseBitSetExtended};
    /// let mut bs = DenseBitSetExtended::with_capacity(128);
    /// bs.set_bit(127, true); // No additional allocation performed
    /// ```
    ///
    pub fn with_capacity(size: usize) -> Self {
        let state: Vec<u64> = Vec::with_capacity(1 + (size >> 6));
        Self { state, size: 0 }
    }

    /// Returns a `DenseBitSetExtended` extending a given `DenseBitSet`.
    ///
    /// # Example
    /// ```
    /// # use rust_dense_bitset::{BitSet, DenseBitSet, DenseBitSetExtended};
    /// let dbs = DenseBitSet::from_integer(0b111000111);
    /// let dbse = DenseBitSetExtended::from_dense_bitset(dbs);
    /// println!("{}", dbse.to_string())
    /// ```
    pub fn from_dense_bitset(dbs: DenseBitSet) -> Self {
        let state = vec![dbs.to_integer()];
        let size = 64;
        Self { state, size }
    }

    /// Returns `true` if and only if all bits are set to `true`
    pub fn all(&self) -> bool {
        let l = self.state.len();
        for i in 0..l - 1 {
            if self.state[i] != u64::max_value() {
                return false;
            }
        }
        if self.size % 64 == 0 {
            if self.state[l - 1] != u64::max_value() {
                return false;
            }
        } else if self.state[l - 1] != ((1 << (self.size % 64)) - 1) {
            return false;
        }
        true
    }

    /// Returns `true` if at least one bit is set to `true`
    pub fn any(&self) -> bool {
        for &s in &self.state {
            if s > 0 {
                return true;
            }
        }
        false
    }

    /// Returns `true` if all the bits are set to `false`
    pub fn none(&self) -> bool {
        !self.any()
    }

    /// Returns the size (in bits) of the bitset
    pub fn get_size(&self) -> usize {
        self.size
    }

    /// Returns an integer representation of the bitsting starting at the given `position` with given `length` (little endian convention).
    ///
    /// Note: this method can extract up to 64 bits into an `u64`. For larger extractions, use `subset` instead.
    ///
    /// # Example
    /// ```
    /// # use rust_dense_bitset::{DenseBitSet, DenseBitSetExtended};
    /// let dbs = DenseBitSet::from_integer(0b111000111);
    /// let dbse = DenseBitSetExtended::from_dense_bitset(dbs);
    /// println!("{}", dbse.extract_u64(2, 4)); // Extracts "0001" (displayed with leading zeros)
    /// ```
    ///
    /// # Panics
    ///
    /// The function panics if `length` is zero
    /// ```rust,should_panic
    /// # use rust_dense_bitset::{DenseBitSet, DenseBitSetExtended};
    /// # let dbs = DenseBitSet::from_integer(0b111000111);
    /// # let dbse = DenseBitSetExtended::from_dense_bitset(dbs);
    /// let panic_me = dbse.extract_u64(3, 0);
    /// ```
    pub fn extract_u64(&self, position: usize, length: usize) -> u64 {
        assert!(
            length <= 64,
            "This implementation is currently limited to 64 bit bitsets."
        );
        assert!(length > 0, "Cannot extract a zero-width slice.");
        if position >= self.size {
            // The extraction is beyond any bit set to 1
            return 0;
        }

        let idx = position >> 6;
        let offset = position % 64;
        let actual_length = if position + length > self.size {
            self.size - position
        } else {
            length
        };

        if actual_length + offset < 64 {
            // Remain within the boundary of an element
            (self.get(idx) >> offset) & ((1 << actual_length) - 1)
        } else if actual_length + offset == 64 {
            // Special case to avoid masking overflow
            self.get(idx) >> offset
        } else {
            // Possibly split between neighbour elements

            // Number of bits to take from the next element
            let remainder = actual_length + offset - 64;

            let lsb = self.state[idx] >> offset;

            if self.state.len() == idx + 1 {
                // If there is no next element, assume it is zero
                lsb
            } else {
                // Otherwise get the remaining bits and assemble the response
                let msb = self.state[idx + 1] & ((1 << remainder) - 1);
                (msb << (64 - offset)) | lsb
            }
        }
    }

    /// Returns a `DenseBitSetExtended` of given `length` whose bits are extracted from the given `position`.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_dense_bitset::{BitSet, DenseBitSet, DenseBitSetExtended};
    /// let dbs = DenseBitSet::from_integer(0b111000111);
    /// let dbse = DenseBitSetExtended::from_dense_bitset(dbs);
    /// println!("{}", dbse.subset(2, 4).to_string());
    /// ```
    pub fn subset(&self, position: usize, length: usize) -> Self {
        if length == 0 {
            return Self {
                state: vec![],
                size: 0,
            };
        }

        let segments = 1 + ((length - 1) >> 6);
        let mut state = vec![];

        for l in 0..segments {
            state.push(self.extract_u64(l * 64 + position, 64));
        }
        Self {
            state,
            size: length,
        }
    }

    /// Inserts the first `length` bits of `other` at the given `position` in the current structure.
    ///
    ///  # Example
    /// ```
    /// # use rust_dense_bitset::{DenseBitSet, DenseBitSetExtended};
    /// let mut bs = DenseBitSetExtended::new();
    /// let bs2 =
    ///    DenseBitSetExtended::from_dense_bitset(DenseBitSet::from_integer(0b1011011101111));
    /// bs.insert(&bs2, 60, 13);
    /// ```
    pub fn insert(&mut self, other: &Self, position: usize, length: usize) {
        let l = (length >> 6) + 1;
        let size_before_insertion = self.size;
        if length % 64 == 0 {
            for i in 0..l {
                self.insert_u64(other.state[i], position + i * 64, 64);
            }
        } else {
            for i in 0..l - 1 {
                self.insert_u64(other.state[i], position + i * 64, 64);
            }
            self.insert_u64(other.state[l - 1], position + (l - 1) * 64, length % 64);
        }

        self.size = max(size_before_insertion, position + length);
    }

    /// Inserts a `length`-bit integer as a bitset at the given `position`.
    ///
    /// # Example
    /// ```
    /// # use rust_dense_bitset::DenseBitSetExtended;
    /// let mut bs = DenseBitSetExtended::new();
    /// bs.insert_u64(0b1011011101111, 50, 64);
    /// ```
    pub fn insert_u64(&mut self, value: u64, position: usize, length: usize) {
        let idx = position >> 6;
        let offset = position % 64;

        // First, resize the bitset if necessary
        if 1 + ((position + length - 1) >> 6) > self.state.len() {
            // We need to extend the bitset to accomodate this insertion
            let num_seg = 1 + ((position + length - 1) >> 6) - self.state.len();

            for _ in 0..num_seg {
                self.state.push(0);
            }
        }
        self.size = max(self.size, position + length);

        // Second, perform the actual insertion
        if offset == 0 && length == 64 {
            // Easiest case
            self.state[idx] = value;
        } else if offset + length - 1 < 64 {
            // Easy case: inserting fewer than 64 bits in an u64
            let mut u = u64::max_value();
            u ^= ((1 << length) - 1) << offset;
            self.state[idx] &= u;
            self.state[idx] |= value << offset;
        } else {
            // Not so easy case: we need to split `value` in twain, zero the appropriate bits in the
            // two segments, and perform the insertion

            let lsb = (value & ((1 << (64 - offset)) - 1)) << offset;
            let mask_lsb = u64::max_value() >> (64 - offset);

            let msb = value >> (64 - offset);
            let mask_msb = u64::max_value() << ((position + length) % 64);

            self.state[idx] = (self.state[idx] & mask_lsb) | lsb;
            self.state[idx + 1] = (self.state[idx + 1] & mask_msb) | msb;
        }
    }

    /// Returns a bit-reversed bitset.
    ///
    /// # Example
    /// ```
    /// # use rust_dense_bitset::{BitSet, DenseBitSet, DenseBitSetExtended};
    /// let val = 66612301234;
    /// let dbs = DenseBitSet::from_integer(val);
    /// let ext_dbs = DenseBitSetExtended::from_dense_bitset(dbs);
    /// println!("{}", dbs.to_string());
    /// println!("{}", ext_dbs.reverse().to_string());
    /// ```
    pub fn reverse(&self) -> Self {
        let mut state = vec![];
        for &s in &self.state {
            let bs = DenseBitSet::from_integer(s).reverse().to_integer();
            state.push(bs);
        }
        state.reverse();
        Self {
            state,
            size: self.size,
        }
    }

    /// Returns a left rotation of the bitset by `shift` bits.
    ///
    /// Note: The bitset is extended by `shift` bits by this operation.
    pub fn rotl(self, shift: usize) -> Self {
        // Rotation is periodic
        let shift_amount = shift % self.size;
        let size_before_shift = self.size;

        if shift_amount == 0 {
            return self;
        }

        let mut shifted = (self.clone() << shift).subset(0, size_before_shift);
        let extra = self.subset(size_before_shift - shift_amount, shift_amount);

        shifted.insert(&extra, 0, shift_amount);

        shifted
    }

    /// Returns a right rotation of the bitset by `shift` bits.
    pub fn rotr(self, shift: usize) -> Self {
        // Rotation is periodic
        let shift_amount = shift % self.size;
        let size_before_shift = self.size;

        if shift_amount == 0 {
            return self;
        }

        let extra = self.subset(0, shift_amount);
        let mut shifted = self >> shift_amount;

        shifted.insert(&extra, size_before_shift - shift_amount, shift_amount);

        shifted
    }

    /// Constructs a `DenseBitSetExtended` from a provided `String`.
    ///
    /// # Example
    /// ```
    /// # use rust_dense_bitset::{DenseBitSetExtended};
    /// let bs2 = DenseBitSetExtended::from_string(String::from("f8d5215a52b57ea0aeb294af576a0aeb"), 16);
    /// ```
    ///
    /// # Panics
    /// This function expects a radix between 2 and 32 included, and will otherwise panic.
    /// This function will also panic if incorrect characters are provided.
    pub fn from_string(s: String, radix: u32) -> Self {
        assert!(
            radix.is_power_of_two(),
            "Only power of two radices are supported"
        );
        assert!(radix > 1, "Radix must be > 1");
        assert!(radix <= 32, "Radix must be <= 32");

        let log_radix = u64::from(radix).trailing_zeros();
        let chunk_size = 64 / log_radix as usize;
        let mut size = 0;

        let mut state = vec![];
        let mut cur = s;
        while !cur.is_empty() {
            if cur.len() > chunk_size {
                let (ms, ls) = cur.split_at(cur.len() - chunk_size);
                let val = u64::from_str_radix(ls, radix).expect("Error while parsing input.");
                state.push(val);
                cur = String::from(ms);
                size += 64;
            } else {
                let val = u64::from_str_radix(&cur.to_string(), radix)
                    .expect("Error while parsing input.");
                state.push(val);
                size += cur.len() * (log_radix as usize);
                break;
            }
        }
        Self { state, size }
    }

    /// Returns the position of the first set bit (little endian convention)
    ///
    /// # Example
    /// ```
    /// # use rust_dense_bitset::{DenseBitSet,DenseBitSetExtended};
    /// let dbs = DenseBitSetExtended::from_dense_bitset( DenseBitSet::from_integer(256) ) << 12;
    /// println!("{}", dbs.first_set());
    /// ```
    pub fn first_set(&self) -> usize {
        for i in 0..self.state.len() {
            let cur = self.state[i];
            if cur != 0 {
                return i * 64 + (cur.trailing_zeros() as usize);
            }
        }
        self.size
    }

    fn get(&self, index: usize) -> u64 {
        match index {
            u if u < self.state.len() => self.state[u],
            _ => 0,
        }
    }
}

/// This is an extended implementation of the `BitSet` trait. It dynamically resizes the bitset as necessary
/// to accomodate growing or shrinking operations (e.g. left shifts) and is only limited by available memory.
/// In practice however, we (arbitrarily) limited allocation to 64000 bits.
///
/// Note: The `BitSet` trait must be in scope in order to use methods from this trait.
///
/// Note: The `Copy` trait cannot be implemented for `DenseBitSetExtended` (for the same reasons avec `Vec`).
impl BitSet for DenseBitSetExtended {
    /// Sets the bit at index `position` to `value`.
    fn set_bit(&mut self, position: usize, value: bool) {
        let idx = position >> 6;
        let offset = position % 64;

        assert!(
            idx < 1000,
            "(Temporary?) We don't allow bitsets larger than 64k for now."
        );

        if idx >= self.state.len() {
            if value {
                // This triggers a resize, we only do it if we need to insert a 1
                for _ in 0..=(idx - self.state.len()) {
                    self.state.push(0);
                }
                self.state[idx] |= 1 << offset
            }
        // Note: To insert a zero, we do nothing, as the value is zero by default
        } else if value {
            self.state[idx] |= 1 << offset
        } else {
            self.state[idx] &= !(1 << offset)
        }
        if position >= self.size {
            self.size = position + 1;
        }
    }

    /// Get the bit at index `position`.
    fn get_bit(&self, position: usize) -> bool {
        if position > self.size {
            return false;
        }

        let idx = position >> 6;
        let offset = position % 64;

        (self.state[idx] >> offset) & 1 == 1
    }

    /// Returns the bitset's Hamming weight (in other words, the number of bits set to true).
    fn get_weight(&self) -> u32 {
        let mut hw = 0;
        for &s in &self.state {
            hw += s.count_ones();
        }
        hw
    }

    /// This resets the bitset to its empty state.
    fn reset(&mut self) {
        self.state = vec![];
        self.size = 0
    }

    /// Returns a representation of the bitset as a `String`.
    fn to_string(self) -> String {
        if self.state.is_empty() {
            return format!("{:064b}", 0);
        }

        let mut bss = vec![];

        if self.size % 64 == 0 {
            for s in self.state {
                bss.push(format!("{:064b}", s));
            }
        } else {
            for i in 0..self.state.len() - 1 {
                bss.push(format!("{:064b}", self.state[i]));
            }
            bss.push(format!(
                "{:064b}",
                self.state[self.state.len() - 1] & ((1 << (self.size % 64)) - 1)
            ));
        }
        bss.reverse();
        bss.join("")
    }
}

impl fmt::Debug for DenseBitSetExtended {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut bss = String::new();

        for i in 0..self.state.len() {
            for j in 0..64 {
                bss += if self.get_bit((self.state.len() - i - 1) * 64 + (63 - j)) {
                    "1"
                } else {
                    "0"
                };
            }
        }
        write!(f, "0b{}", bss)
    }
}

impl PartialEq for DenseBitSetExtended {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        for i in 0..self.state.len() {
            if self.state[i] != other.state[i] {
                return false;
            }
        }
        true
    }
}

impl Hash for DenseBitSetExtended {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for s in &self.state {
            s.hash(state);
        }
    }
}

impl Eq for DenseBitSetExtended {}

impl Not for DenseBitSetExtended {
    type Output = Self;
    fn not(self) -> Self {
        let l = self.state.len();
        let mut inv = Self {
            state: Vec::with_capacity(l),
            size: self.size,
        };
        for i in 0..l - 1 {
            inv.state.push(!self.state[i])
        }
        if self.size % 64 == 0 {
            inv.state.push(!self.state[l - 1]);
        } else {
            inv.state
                .push((!self.state[l - 1]) & ((1 << (self.size % 64)) - 1));
        }
        inv
    }
}

impl BitAnd for DenseBitSetExtended {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        let l = min(self.state.len(), rhs.state.len());
        let mut v = Vec::with_capacity(l);

        // Note: there is no need to go further because x & 0 == 0
        for i in 0..l {
            v.push(self.state[i] & rhs.state[i])
        }

        Self {
            state: v,
            size: min(self.size, rhs.size),
        }
    }
}

impl BitAndAssign for DenseBitSetExtended {
    fn bitand_assign(&mut self, rhs: Self) {
        // Note: there is no need to go further because x & 0 == 0
        let l = min(self.state.len(), rhs.state.len());
        for i in 0..l {
            self.state[i] &= rhs.state[i];
        }
        self.size = min(self.size, rhs.size);
    }
}

impl BitOr for DenseBitSetExtended {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        let l = max(self.state.len(), rhs.state.len());
        let mut v = Vec::with_capacity(l);

        for i in 0..l {
            if i < self.state.len() && i < rhs.state.len() {
                // x | y
                v.push(self.state[i] | rhs.state[i])
            } else if i >= self.state.len() {
                // x | 0 == x
                v.push(rhs.state[i])
            } else {
                // x | 0 == x
                v.push(self.state[i])
            }
        }

        Self {
            state: v,
            size: max(self.size, rhs.size),
        }
    }
}

impl BitOrAssign for DenseBitSetExtended {
    fn bitor_assign(&mut self, rhs: Self) {
        let l = max(self.state.len(), rhs.state.len());
        for i in 0..l {
            if i < self.state.len() && i < rhs.state.len() {
                // x | y
                self.state[i] |= rhs.state[i]
            } else if i >= self.state.len() {
                // x | 0 == x
                self.state[i] = rhs.state[i]
            }
            // if rhs.state[i] == 0 we do nothing because x | 0 == x
        }
    }
}

impl BitXor for DenseBitSetExtended {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        let l = max(self.state.len(), rhs.state.len());
        let mut v = Vec::with_capacity(l);

        for i in 0..l {
            if i < self.state.len() && i < rhs.state.len() {
                // x ^ y
                v.push(self.state[i] ^ rhs.state[i])
            } else if i >= self.state.len() {
                // x ^ 0 == x
                v.push(rhs.state[i])
            } else {
                // x ^ 0 == x
                v.push(self.state[i])
            }
        }

        Self {
            state: v,
            size: max(self.size, rhs.size),
        }
    }
}

impl BitXorAssign for DenseBitSetExtended {
    fn bitxor_assign(&mut self, rhs: Self) {
        let l = max(self.state.len(), rhs.state.len());
        for i in 0..l {
            if i < self.state.len() && i < rhs.state.len() {
                // x ^ y
                self.state[i] ^= rhs.state[i]
            } else if i >= self.state.len() {
                // x ^ 0 == x
                self.state[i] = rhs.state[i]
            }
            // if rhs.state[i] == 0 we do nothing because x ^ 0 == x
        }
    }
}

impl Shl<usize> for DenseBitSetExtended {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self {
        let mut v = DenseBitSetExtended::with_capacity(self.size + rhs);

        // Note: this may not be the most efficient implementation
        for i in 0..self.size {
            let source = i;
            let dest = i + rhs;
            v.set_bit(dest, self.get_bit(source));
        }
        v
    }
}

impl ShlAssign<usize> for DenseBitSetExtended {
    fn shl_assign(&mut self, rhs: usize) {
        let trailing_zeros = rhs >> 6;
        let actual_shift = rhs % 64;
        if rhs > (self.state.len() * 64 - self.size) {
            self.state.push(0);
        }
        let l = self.state.len();
        for i in 0..(l - 1) {
            self.state[l - i - 1] = (self.state[l - i - 1] << actual_shift)
                | (self.state[l - i - 2] >> (64 - actual_shift))
        }
        self.state[0] <<= actual_shift;
        for _ in 0..trailing_zeros {
            self.state.insert(0, 0);
        }
        self.size += rhs;
    }
}

impl Shr<usize> for DenseBitSetExtended {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self {
        if rhs >= self.size {
            Self {
                state: vec![],
                size: 0,
            }
        } else {
            let mut v = DenseBitSetExtended::with_capacity(self.size - rhs);

            // Note: this may not be the most efficient implementation
            for i in 0..(self.size - rhs) {
                let source = i + rhs;
                let dest = i;
                v.set_bit(dest, self.get_bit(source));
            }

            v
        }
    }
}

impl ShrAssign<usize> for DenseBitSetExtended {
    fn shr_assign(&mut self, rhs: usize) {
        if rhs >= self.size {
            self.reset();
        }
        let to_drop = rhs >> 6;
        let actual_shift = rhs % 64;
        for _ in 0..to_drop {
            self.state.remove(0);
        }
        let l = self.state.len();
        for i in 0..(l - 1) {
            self.state[i] =
                (self.state[i] >> actual_shift) | (self.state[i + 1] << (64 - actual_shift))
        }
        self.state[l - 1] >>= actual_shift;
        self.size -= rhs;
    }
}

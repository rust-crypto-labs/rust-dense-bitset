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

/// Provides a `BitSet` implementation (only limited by available memory)
#[derive(Clone)]
pub struct DenseBitSetExtended {
    state: Vec<u64>,
    size: usize,
}

impl DenseBitSetExtended {
    /// Returns a new empty extended `DenseBitsetExtended`
    pub fn new() -> Self {
        let state: Vec<u64> = Vec::new();
        return Self { state, size: 0 };
    }

    /// Returns an empty `DenseBitsetExtended` with pre-allocated memory of `size` bits
    ///
    /// This is useful to avoid additional allocations is situations where the bitset's
    /// space requirements are known in advance
    pub fn with_capacity(size: usize) -> Self {
        assert!(
            size < 64_000,
            "(Temporary?) We don't allow bitsets larger than 64k for now."
        );
        let state: Vec<u64> = Vec::with_capacity(1 + (size >> 6));
        Self { state, size: 0 }
    }

    /// Returns a `DenseBitSetExtended` extending a given `DenseBitSet`
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

    /// Returns an integer representation of the bitsting starting at the given `position` with given `length` (little endian convention)
    ///
    /// Note: this method wan extract up to 64 bits into an `u64`. For larger extractions, use `extract`
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
            return (self.state[idx] >> offset) & ((1 << actual_length) - 1);
        } else if actual_length + offset == 64 {
            // Special case to avoid masking overflow
            return self.state[idx] >> offset;
        } else {
            // Possibly split between neighbour elements

            // Number of bits to take from the next element
            let remainder = actual_length + offset - 64;

            let lsb = self.state[idx] >> offset;

            if self.state.len() == idx + 1 {
                // If there is no next element, assume it is zero
                return lsb;
            } else {
                // Otherwise get the remaining bits and assemble the response
                let msb = self.state[idx + 1] & ((1 << remainder) - 1);
                return (msb << (64 - offset)) | lsb;
            }
        }
    }

    /// Returns a `DenseBitSetExtended` of given `length` whose bits are extracted from the given `position`
    pub fn subset(&self, position: usize, length: usize) -> Self {
        let segments = 1 + (length >> 6);
        let mut state = vec![];

        for l in 0..segments {
            state.push(self.extract_u64(l * 64 + position, 64));
        }
        Self {
            state,
            size: length,
        }
    }

    /// Inserts the first `length` bits of `other` at the given `position` in the current structure
    pub fn insert(&mut self, other: &Self, position: usize) {
        let l = other.state.len();
        let size_before_insertion = self.size;
        if other.size % 64 == 0 {
            for i in 0..l {
                self.insert_u64(other.state[i], position + i * 64, 64);
            }
        }
        else {
            for i in 0..l-1 {
                self.insert_u64(other.state[i], position + i * 64, 64);
            }
            self.insert_u64(other.state[l-1], position + (l-1) * 64, other.size % 64);
        }
    
        self.size = max(size_before_insertion, position + other.size);
    }

    /// Inserts a `length`-bit integer as a bitset at the given `position`
    pub fn insert_u64(&mut self, value: u64, position: usize, length: usize) {
        let idx = position >> 6;
        let offset = position % 64;

        // First, resize the bitset if necessary
        if 1 + ((position + length)>>6)  > self.state.len()  {
            // We need to extend the bitset to accomodate this insertion
            let num_seg = 1 + ((position + length)>>6) - self.state.len();

            for _ in 0..num_seg {
                self.state.push(0);
            }
        }
        self.size = max(self.size, position + length);

        // Second, perform the actual insertion
        if offset == 0 && length == 64 {
            // Easiest case
            self.state[idx] = value;
        }
        else if offset + length < 64 {
            // Easy case: inserting fewer than 64 bits in an u64
            let mut u = u64::max_value();
            u ^= ((1 << length) - 1) << offset;
            self.state[idx] &= u;
            self.state[idx] |= value << position;
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

    /// Returns a bit-reversed bitset
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

    /// Returns a left rotation of the bitset by `shift` bits
    pub fn rotl(self, shift: usize) -> Self {
        // Rotation is periodic
        let shift_amount = shift % self.size;
        let size_before_shift = self.size;

        let mut shifted = self << shift;
        let extra = shifted.subset(size_before_shift, shift);

        shifted.insert(&extra, 0);

        shifted
    }

    /// Returns a right rotation of the bitset by `shift` bits
    pub fn rotr(self, shift: usize) -> Self {
        // Rotation is periodic
        let shift_amount = shift % self.size;
        let size_before_shift = self.size;

        let extra = self.subset(0, shift);
        let mut shifted = self >> shift;

        shifted.insert(&extra, size_before_shift);

        shifted
    }

    /// Constructs a `DenseBitSetExtended` from a provided string
    pub fn from_string(s: String, radix: u32) -> Self {
        assert!(
            radix.is_power_of_two(),
            "Only power of two radices are supported"
        );
        assert!(radix > 0, "Radix must be > 0");
        assert!(radix <= 32, "Radix must be <= 32");

        let log_radix = (radix as u64).trailing_zeros();
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
}

impl BitSet for DenseBitSetExtended {
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
        } else {
            if value {
                self.state[idx] |= 1 << offset
            } else {
                self.state[idx] &= !(1 << offset)
            }
        }
        if position >= self.size {
            self.size = position + 1;
        }
    }

    fn get_bit(&self, position: usize) -> bool {
        if position > self.size {
            return false;
        }

        let idx = position >> 6;
        let offset = position % 64;

        (self.state[idx] >> offset) & 1 == 1
    }

    fn get_weight(&self) -> u32 {
        let mut hw = 0;
        for &s in &self.state {
            hw += s.count_ones();
        }
        hw
    }

    fn reset(&mut self) {
        self.state = vec![];
        self.size = 0
    }

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
            } else if i > self.state.len() {
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
            } else if i > self.state.len() {
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
            } else if i > self.state.len() {
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
            } else if i > self.state.len() {
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

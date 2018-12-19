use crate::bitset::BitSet;
use crate::u64impl::DenseBitSet;


use std::hash::{Hash, Hasher};
use std::fmt;
use std::cmp::{min, max};

/// Overload of &, &=, |, |=, ^, ^=, !, <<, <<=, >>, >>=
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

/// Provides a bitset implementation (only limited by available memory)
#[derive(Clone)]
pub struct DenseBitSetExtended {
    state: Vec<u64>,
    size: usize,
}

impl DenseBitSetExtended {

    pub fn new() -> Self {
        let state: Vec<u64> = Vec::new();
        return Self{ state , size: 0 }
    }

    /// Returns a preallocated Extended Dense Bitset of `size` bits
    pub fn with_capacity(size: usize) -> Self {
        assert!(size < 64_000, "(Temporary?) We don't allow bitsets larger than 64k for now.");
        let state : Vec<u64> = Vec::with_capacity(1 + (size >> 6));
        Self { state, size: 0 }
    }

    /// Returns a DenseBitSetExtended from a given DenseBitSet
    pub fn from_dense_bitset(dbs: DenseBitSet) -> Self {
        let state = vec![dbs.to_integer()];
        let size = 64;
        Self { state, size }
    }

    /// Returns true if all bits are set to true
    pub fn all(&self) -> bool {
        let l = self.state.len();
        for i in 0..l-1 {
            if self.state[i] != u64::max_value() {
                return false;
            }
        }
        if self.size % 64 == 0 {
            if self.state[l-1] != u64::max_value() {
                return false
            }
        } else if self.state[l-1] != ((1 << (self.size % 64)) - 1 ) {
            return false;
        }
        true
    }

    /// Returns true if any of the bits are set to true
    pub fn any(&self) -> bool {
        for &s in &self.state {
            if s > 0 {
                return true;
            }
        }
        false
    }

    /// Returns true if none of the bits are set to true
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
        let actual_length = if position + length > self.size { self.size - position } else { length };
        
        if actual_length + offset < 64 {
            // Remain within the boundary of an element
            return (self.state[idx] >> offset) & ((1 << actual_length) - 1);
        }
        else if actual_length + offset == 64 {
            // Special case to avoid masking overflow
            return self.state[idx] >> offset; 
        }
        else {
            // Possibly split between neighbour elements

            // Number of bits to take from the next element
            let remainder = actual_length + offset - 64;

            let lsb = self.state[idx] >> offset;

            if self.state.len() == idx + 1 {
                // If there is no next element, assume it is zero
                return lsb;
            } else {
                // Otherwise get the remaining bits and assemble the response
                let msb = self.state[idx+1]  & ((1 << remainder) - 1);
                return (msb << (64 - offset)) | lsb;
            }
        }
    }

    /// Returns a `DenseBitSetExtended` of given `length` whose bits are extracted from the given `position` 
    pub fn subset(&self, position: usize, length: usize) -> Self {
        let segments = 1+ (length >> 6);
        let mut state = vec![];

        for l in 0..segments {
            state.push ( self.extract_u64( l*64 + position, 64 ) );
        }
        Self { state, size: length }

    }

    /// Inserts a `DenseBitSet` at the given `position`, which extends and overwrites the current one
    pub fn insert(&mut self, other: &Self, position: usize) {
        let l = other.state.len();
        let size_before_insertion = self.size;
        for i in 0..l {
            self.insert_u64(other.state[i], position + i * 64);
        }

        // By default, insert_u64 sets the size to the next multiple of 64
        // but we can truncate more precisely here
        self.size = max ( size_before_insertion, position + l * 64);
        
        println!("vecsize: {}", self.state.len());
    }

    /// Inserts an integer as a bitset at the given `position`
    pub fn insert_u64(&mut self, i: u64, position: usize) {

        let idx = position >> 6;
        let offset = position % 64;

        // First, resize the bitset if necessary
        if 1+idx > self.state.len() {
            // We need to extend the bitset to accomodate this insertion
            let num_seg = 1+idx - self.state.len();

            for _ in 0..=num_seg {
                self.state.push(0);
            }
        }
        self.size = max( self.size, position + 64 );

        // Second, perform the actual insertion
        if offset == 0 {
            // Easy case: the u64 matches an element
            self.state[idx] = i;
        }
        else {
            // We need to split `i` in twain, zero the appropriate bits in the
            // to segments, and perform the insertion

            let lsb = (i & ((1 << (64 - offset))-1)) << offset;
            let msb =  i >> (64 - offset);
            let mask_lsb = u64::max_value() >> (64-offset);
            let mask_msb = u64::max_value() << offset;

            self.state[idx] = (self.state[idx] & mask_lsb) | lsb;
            self.state[idx+1] = (self.state[idx+1] & mask_msb) | msb;
        }
    }
}

impl BitSet for DenseBitSetExtended {
    fn set_bit(&mut self, position: usize, value: bool) {
        let idx = position >> 6;
        let offset = position % 64;

        assert!(idx < 1000, "(Temporary?) We don't allow bitsets larger than 64k for now.");

        if idx >= self.state.len() {
            if value {
                // This triggers a resize, we only do it if we need to insert a 1
                for _ in 0..=(idx - self.state.len()) {
                    self.state.push(0);
                }
                self.state[idx] |= 1 << offset
            }
            // Note: To insert a zero, we do nothing, as the value is zero by default
        }
        else {
            if value {
                self.state[idx] |= 1 << offset
            } else {
                self.state[idx] &= !(1 << offset)
            }
        }
        if position >= self.size {
            self.size = position+1;
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
        if self.state.len() == 0 {
            return format!("{:064b}", 0)
        }

        let mut bss = vec![];

        if self.size % 64 == 0 {
            for s in self.state {
                bss.push( format!("{:064b}", s) );
            }
        }
        else {
            for i in 0..self.state.len() - 1 {
                bss.push( format!("{:064b}", self.state[i]) );
            }
            bss.push( format!("{:064b}", self.state[self.state.len() - 1] & ( (1 << (self.size % 64)) - 1)) );
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
                bss+= if self.get_bit((self.state.len()-i-1)*64+(63-j)) { "1" } else { "0" };
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
        let mut inv = Self { state: Vec::with_capacity(l), size: self.size };
        for i in 0..l-1 {
            inv.state.push(!self.state[i])
        }
        if self.size % 64 == 0 {
            inv.state.push(!self.state[l-1]);
        }
        else {
            inv.state.push( (!self.state[l-1]) & ((1 << (self.size % 64)) -1) );
        }
        inv
    }
}

impl BitAnd for DenseBitSetExtended {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        let l = min (self.state.len(), rhs.state.len() );
        let mut v = Vec::with_capacity(l);

        // Note: there is no need to go further because x & 0 == 0
        for i in 0..l {
            v.push( self.state[i] & rhs.state[i] )
        }

        Self { state: v, size: min(self.size, rhs.size) }
    }
}

impl BitAndAssign for DenseBitSetExtended {
    fn bitand_assign(&mut self, rhs: Self) {
        // Note: there is no need to go further because x & 0 == 0
        let l = min (self.state.len(), rhs.state.len() );
        for i in 0..l {
            self.state[i] &= rhs.state[i];
        }
        self.size = min(self.size, rhs.size);
    }
}

impl BitOr for DenseBitSetExtended {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        let l = max (self.state.len(), rhs.state.len() );
        let mut v = Vec::with_capacity(l);

        for i in 0..l {
            if i < self.state.len() && i < rhs.state.len() {
                // x | y
                v.push( self.state[i] | rhs.state[i] )
            } else if i > self.state.len() {
                // x | 0 == x
                v.push( rhs.state[i] )
            } else {
                // x | 0 == x
                v.push( self.state[i] )
            }
        }

        Self { state: v, size: max(self.size, rhs.size) }
    }
}

impl BitXor for DenseBitSetExtended {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        let l = max (self.state.len(), rhs.state.len() );
        let mut v = Vec::with_capacity(l);

        for i in 0..l {
            if i < self.state.len() && i < rhs.state.len() {
                // x ^ y
                v.push( self.state[i] ^ rhs.state[i] )
            } else if i > self.state.len() {
                // x ^ 0 == x
                v.push( rhs.state[i] )
            } else {
                // x ^ 0 == x
                v.push( self.state[i] )
            }
        }

        Self { state: v, size: max(self.size, rhs.size) }
    }
}

impl Shr<usize> for DenseBitSetExtended {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self {
        if rhs >= self.size {
            Self { state: vec![], size: 0 }
        } else {
            let mut v = DenseBitSetExtended::with_capacity(self.size - rhs);

            // Note: this may not be the most efficient implementation
            for i in 0..(self.size - rhs) {
                let source = i + rhs;
                let dest = i;
                v.set_bit( dest, self.get_bit(source) );
            }

            v
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
            v.set_bit( dest, self.get_bit(source) );
        }
        v
    }
}
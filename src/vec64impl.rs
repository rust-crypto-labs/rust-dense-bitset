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
        if self.state.len() == 0 {
            return format!("{:064b}", 0);
        }

        let mut bss = vec![];
        for s in self.state {
            bss.push(format!("{:064b}", s));
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

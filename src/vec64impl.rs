use crate::bitset::BitSet;

use std::fmt;

/// Overload of &, &=, |, |=, ^, ^=, !, <<, <<=, >>, >>=
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

/// Provides a bitset implementation (only limited by available memory)
#[derive(Clone, Hash)]
pub struct DenseBitSetExtended {
    state: Vec<u64>,
}

impl DenseBitSetExtended {
    /// Returns true if all bits are set to true
    pub fn all(&self) -> bool {
        for s in self.state.iter() {
            if *s != u64::max_value() {
                return false;
            }
        }
        true
    }

    /// Returns true if any of the bits are set to true
    pub fn any(&self) -> bool {
        for s in self.state.iter() {
            if *s > 0 {
                return true;
            }
        }
        false
    }

    /// Returns true if none of the bits are set to true
    pub fn none(&self) -> bool {
        !self.any()
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
                for _ in 0..(self.state.len() - idx) {
                self.state.push(0);
              }
              self.state[idx] |= 1 << offset
            }
            // To insert a zero, we do nothing, as the value is zero by default
        }
        else {
            if value {
                self.state[idx] |= 1 << offset
            } else {
                self.state[idx] &= !(1 << offset)
            }
        }
    }

    fn get_bit(&self, position: usize) -> bool {
        let idx = position >> 6;
        let offset = position % 64;
        if idx > self.state.len() {
          return false;
        }

        (self.state[idx] >> offset) & 1 == 1
    }

    fn get_weight(&self) -> u32 {
        let mut hw = 0;
        for s in self.state.iter() {
            hw += s.count_ones()
        }
        hw
    }

    fn reset(&mut self) {
        self.state = vec![]
    }

    fn flip(&mut self) {
        for i in 0..self.state.len() {
            self.state[i] = !self.state[i]
        }
    }
}
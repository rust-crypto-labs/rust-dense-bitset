extern crate rust_dense_bitset;

use rust_dense_bitset::DenseBitSet;
use rust_dense_bitset::DenseBitSetExtended;

fn main() {
    let bs = DenseBitSet::from_integer(0b11110001);
    let mut bs2 = DenseBitSetExtended::from_dense_bitset(bs);
    let bs_cp = bs2.clone();
    bs2 = bs2.rotl(40);
    bs2 = bs2.rotl(24);
    assert_eq!(bs2, bs_cp);
}

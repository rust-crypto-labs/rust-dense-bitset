#![allow(clippy::suspicious_op_assign_impl)]
#![allow(clippy::unreadable_literal)]

mod bitset;
mod u64impl;
mod vec64impl;

pub use crate::bitset::BitSet;
pub use crate::u64impl::DenseBitSet;
pub use crate::vec64impl::DenseBitSetExtended;

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for data initialization methods
    // dbs : from_integer, from_string
    // dbse : from_dense_bitset, from_string
    //
    // Test of functionality (The method is doing what is expected)
    // Test of panic cases

    #[test]
    fn test_from_integer_dbs() {
        let mut bs1 = DenseBitSet::from_integer(2047);
        bs1.set_bit(11, true);
        assert_eq!(bs1.to_integer(), 4095);
    }

    #[test]
    fn test_from_dense_bitset_dbse() {
        let bs1 = DenseBitSet::from_integer(2048);
        let mut bs2 = DenseBitSetExtended::from_dense_bitset(bs1);
        bs2.set_bit(70, true);
        assert_eq!(bs2.get_weight(), 2);
        assert_eq!(bs2.get_size(), 71);
        assert!(bs2.get_bit(11));
    }

    #[test]
    fn test_from_string_dbs() {
        let bs1 = DenseBitSet::from_string("101110001", 2);
        let bs2 = DenseBitSet::from_string("FFFFF", 16);
        let bs3 = DenseBitSet::from_string("123465", 10);

        assert_eq!(bs1.to_integer(), 0b101110001);
        assert_eq!(bs2.to_integer(), 0xfffff);
        assert_eq!(bs3.to_integer(), 123465);
    }

    #[test]
    fn test_from_string_dbse() {
        let val = "11111000110101010010000101011010010100101011010101111110101000001010111010110010100101001010111101010111011010100000101011101011";
        let bs1 = DenseBitSetExtended::from_string(String::from(val), 2);
        assert_eq!(bs1.to_string(), val);

        let bs2 =
            DenseBitSetExtended::from_string(String::from("f8d5215a52b57ea0aeb294af576a0aeb"), 16);
        assert_eq!(bs2.to_string(), val);
    }

    // Panics if the string contains a char that
    // can't be converted to an integer value in the specified base

    #[test]
    #[should_panic]
    fn catch_invalid_string_dbs_incorrect_char() {
        let _bs = DenseBitSet::from_string("Hello World!", 12);
    }

    #[test]
    #[should_panic]
    fn catch_invalid_string_dbse_incorrect_char() {
        let _bs = DenseBitSetExtended::from_string(String::from("Hello World!"), 12);
    }

    // Panics if the radix is > 32

    #[test]
    #[should_panic]
    fn catch_invalid_string_dbs_incorrect_radix() {
        let _bs = DenseBitSet::from_string("1234", 33);
    }

    #[test]
    #[should_panic]
    fn catch_invalid_string_dbse_incorrect_radix() {
        let _bs = DenseBitSetExtended::from_string(String::from("1234"), 33);
    }

    // Panics if radix is < 2

    #[test]
    #[should_panic]
    fn catch_invalid_string_dbs_incorrect_radix2() {
        let _bs = DenseBitSet::from_string("0000", 1);
    }

    #[test]
    #[should_panic]
    fn catch_invalid_string_dbse_incorrect_radix2() {
        let _bs = DenseBitSetExtended::from_string(String::from("0000"), 1);
    }

    // Tests for information data
    // generic : any, all, none, first_set
    // dbse : get_size

    #[test]
    fn test_all_dbs() {
        let mut bs = DenseBitSet::from_integer(u64::max_value());
        assert!(bs.all());
        bs.set_bit(3, false);
        assert!(!bs.all());
    }

    #[test]
    fn test_all_dbse() {
        let mut bs =
            DenseBitSetExtended::from_string(String::from("fffffffffffffffffffffffffffff"), 16);
        assert!(bs.all());
        bs.set_bit(28, false);
        assert!(!bs.all());
    }

    #[test]
    fn test_any_dbs() {
        let mut bs = DenseBitSet::from_integer(1234567890);
        assert!(bs.any());
        bs.reset();
        assert!(!bs.any());
    }

    #[test]
    fn test_any_dbse() {
        let mut bs = DenseBitSetExtended::with_capacity(10);
        bs.set_bit(1234, true);
        assert!(bs.any());
        bs.reset();
        assert!(!bs.any());
    }

    #[test]
    fn test_none_dbs() {
        let mut bs = DenseBitSet::from_integer(0);
        assert!(bs.none());
        bs.set_bit(3, true);
        assert!(!bs.none());
    }

    #[test]
    fn test_none_dbse() {
        let mut bs = DenseBitSetExtended::with_capacity(10);
        bs.set_bit(1234, true);
        bs.set_bit(1234, false);
        assert!(bs.none());
        bs.set_bit(1235, true);
        assert!(!bs.none());
    }

    #[test]
    fn test_get_size_dbse() {
        let mut bs = DenseBitSetExtended::from_string(String::from("deadbeef"), 16);
        assert_eq!(bs.get_size(), 32);
        bs.set_bit(8975, false);
        assert_eq!(bs.get_size(), 8976);
    }

    #[test]
    fn test_first_set_dbs() {
        let dbs = DenseBitSet::from_integer(256);
        assert_eq!(8, dbs.first_set());
    }

    #[test]
    fn test_first_set_dbse() {
        let dbs = DenseBitSetExtended::from_dense_bitset(DenseBitSet::from_integer(256)) << 223;
        assert_eq!(231, dbs.first_set());
    }

    // Tests for set manipulations
    // generic : reverse, rotr, rotl
    // dbs: insert, extract
    // dbse: insert(set), subset, insert_u64, extract_u64

    #[test]
    fn test_rotr_dbs() {
        let mut bs = DenseBitSet::from_integer(0b0001110101);
        let bs_cp = bs;
        bs.rotr(40);
        assert_eq!(bs.to_integer(), 0b1110101000000000000000000000000);
        bs.rotr(24);
        assert_eq!(bs, bs_cp);
    }

    #[test]
    fn test_rotl_dbs() {
        let mut bs = DenseBitSet::from_integer(0b0001110101);
        let bs_cp = bs;
        bs.rotl(10);
        assert_eq!(bs.to_integer(), 0b11101010000000000);
        bs.rotl(54);
        assert_eq!(bs, bs_cp);
    }

    #[test]
    fn test_rotr_dbse() {
        let bs = DenseBitSet::from_integer(0b11110001);
        let mut bs2 = DenseBitSetExtended::from_dense_bitset(bs);
        let bs_cp = bs2.clone();
        bs2 = bs2.rotr(40);
        bs2 = bs2.rotr(24);
        assert_eq!(bs2, bs_cp);
    }

    #[test]
    fn test_rotl_dbse() {
        let bs = DenseBitSet::from_integer(0b11110001);
        let mut bs2 = DenseBitSetExtended::from_dense_bitset(bs);
        let bs_cp = bs2.clone();
        bs2 = bs2.rotl(40);
        bs2 = bs2.rotl(24);
        assert_eq!(bs2, bs_cp);
    }

    #[test]
    fn test_reverse_dbs() {
        let bs = DenseBitSet::from_integer(666123);
        let srev = bs.to_string().chars().rev().collect::<String>();
        assert_eq!(srev, bs.reverse().to_string());
    }

    #[test]
    fn test_reverse_dbse() {
        let bs = DenseBitSetExtended::from_dense_bitset(DenseBitSet::from_integer(666123)) >> 63;
        let rs = bs.reverse();
        let srev = bs.to_string().chars().rev().collect::<String>();
        assert_eq!(srev, rs.to_string());
    }

    // Tests for extract and insert on dbs

    #[test]
    fn no_crash_on_insertion_dbs() {
        let mut bs = DenseBitSet::from_integer(0);
        for i in 0..63 {
            bs.set_bit(i, true);
        }
    }

    #[test]
    fn no_crash_on_insertion_dbse() {
        let mut bs = DenseBitSetExtended::with_capacity(10);
        for i in 0..1024 {
            bs.set_bit(i, true);
        }
    }

    #[test]
    fn no_crash_on_read_dbs() {
        let bs = DenseBitSet::from_integer(1234567890);
        let mut hw = 0;
        for i in 0..64 {
            hw += if bs.get_bit(i) { 1 } else { 0 };
        }
        assert_eq!(hw, 12, "Error: mismatch between expected and read bits.");
    }

    #[test]
    #[should_panic]
    fn catch_get_overflow_dbs() {
        let bs = DenseBitSet::from_integer(1234567890);
        let _r = bs.get_bit(64); // Should panic: bit #64 is out of bounds
    }

    #[test]
    #[should_panic]
    fn catch_set_overflow_dbs() {
        let mut bs = DenseBitSet::from_integer(1234567890);
        bs.set_bit(64, true); // Should panic: bit #64 is out of bounds
    }

    #[test]
    fn read_write_test_dbs() {
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
    fn full_extract_dbs() {
        let bs = DenseBitSet::from_integer(1234567890);
        assert_eq!(bs.extract(0, 64), 1234567890);
    }

    #[test]
    fn partial_extracts_dbs() {
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
    fn catch_extract_zero_width_dbs() {
        let bs = DenseBitSet::from_integer(1234567890);
        let _r = bs.extract(12, 0); // Should panic: 0 is passed as 2nd argument
    }

    #[test]
    #[should_panic]
    fn catch_extract_overflow_dbs() {
        let bs = DenseBitSet::from_integer(1234567890);
        let _r = bs.extract(12, 55); // Should panic: 12+55 exceeds the 64 bit boundary
    }

    #[test]
    fn full_insert_dbs() {
        let mut bs = DenseBitSet::from_integer(1234567890);
        bs.insert(0, 64, 9876);
        assert_eq!(bs.to_integer(), 9876);
    }

    #[test]
    fn partial_insert_dbs() {
        let mut bs = DenseBitSet::from_integer(0b11101101010100011001);
        bs.insert(12, 2, 0b10);
        assert_eq!(bs.to_integer(), 0b11101110010100011001)
    }

    #[test]
    #[should_panic]
    fn catch_insert_zero_width_dbs() {
        let mut bs = DenseBitSet::from_integer(1234567890);
        bs.insert(0, 0, 12); // Should panic: 0 is passed as 2nd argument
    }

    #[test]
    #[should_panic]
    fn catch_insert_overflow_dbs() {
        let mut bs = DenseBitSet::from_integer(1234567890);
        bs.insert(12, 55, 79885); // Should panic: 12+55 exceeds the 64 bit boundary
    }

    // Tests for insert and extract functions for dbse

    #[test]
    fn test_extract_u64_dbse() {
        let offset = 140;
        let bs =
            DenseBitSetExtended::from_dense_bitset(DenseBitSet::from_integer(1234567890)) << offset;
        let e1 = bs.extract_u64(1 + offset, 63);
        let e2 = bs.extract_u64(offset, 8);
        let e3 = bs.extract_u64(5 + offset, 14);

        assert_eq!(e1, 617283945);
        assert_eq!(e2, 210);
        assert_eq!(e3, 12310);
    }

    #[test]
    #[should_panic]
    fn catch_extract_u64_zero_width_dbse() {
        let bs = DenseBitSetExtended::new();
        let _r = bs.extract_u64(12, 0); // Should panic: 0 is passed as 2nd argument
    }

    #[test]
    #[should_panic]
    fn catch_extract_u64_overflow_dbse() {
        let bs = DenseBitSetExtended::from_dense_bitset(DenseBitSet::from_integer(1234567890));
        let _r = bs.extract_u64(12, 75); // Should panic: 12+75 exceeds the 64 bit size limit
    }

    #[test]
    fn test_subset_dbse() {
        let bs = DenseBitSetExtended::from_dense_bitset(DenseBitSet::from_integer(1234567890));
        let e1 = bs.subset(0, 12);
        let e2 = bs.subset(4, 128).subset(0, 4);

        assert_eq!(
            e1.to_string(),
            "0000000000000000000000000000000000000000000000000000001011010010"
        );
        assert_eq!(
            e2.to_string(),
            "0000000000000000000000000000000000000000000000000000000000001101"
        );
    }

    #[test]
    fn test_insert_u64_dbse() {
        let mut bs = DenseBitSetExtended::new();
        bs.insert_u64(0b1011011101111, 50, 64);

        assert_eq!(bs.to_string(), "00000000000000000000000000000000000000000000000000000000000000000101101110111100000000000000000000000000000000000000000000000000");
    }

    #[test]
    fn test_insert_dbse() {
        let mut bs = DenseBitSetExtended::new();
        let bs2 =
            DenseBitSetExtended::from_dense_bitset(DenseBitSet::from_integer(0b1011011101111));
        bs.insert(&bs2, 60, 13);

        assert_eq!(bs.to_string(), "00000000000000000000000000000000000000000000000000000001011011101111000000000000000000000000000000000000000000000000000000000000");
    }

    // Tests for `BitSet` trait methods implementations
    // generic : set_bit, get_bit, get_weight, reset, to_string

    #[test]
    fn test_reset_dbs() {
        let mut bs = DenseBitSet::from_integer(1234567890);
        bs.reset();
        assert_eq!(bs.to_integer(), 0);
    }

    #[test]
    fn test_reset_dbse() {
        let mut bs = DenseBitSetExtended::from_string(String::from("0101"), 2);
        bs.reset();
        assert_eq!(
            bs.to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn test_hamming_weight_dbs() {
        let bs1 = DenseBitSet::from_integer(0);
        let bs2 = DenseBitSet::from_integer(1234567890);
        let bs3 = DenseBitSet::from_integer(u64::max_value());

        assert_eq!(bs1.get_weight(), 0);
        assert_eq!(bs2.get_weight(), 12);
        assert_eq!(bs3.get_weight(), 64);
    }

    #[test]
    fn test_hamming_weight_dbse() {
        let bs1 = DenseBitSet::from_integer(1234567890);
        let mut bs2 = DenseBitSetExtended::from_dense_bitset(bs1);

        bs2.set_bit(78, true);
        bs2.set_bit(289, true);

        assert_eq!(bs2.get_weight(), 14);
    }

    #[test]
    fn test_to_string_dbs() {
        let bs1 = DenseBitSet::from_integer(7891234);
        let bs2 = DenseBitSet::from_integer(65536);
        assert_eq!(
            bs1.to_string(),
            "0000000000000000000000000000000000000000011110000110100100100010"
        );
        assert_eq!(
            bs2.to_string(),
            "0000000000000000000000000000000000000000000000010000000000000000"
        )
    }

    #[test]
    fn test_to_string_dbse() {
        let mut bs1 = DenseBitSetExtended::with_capacity(100);
        bs1.set_bit(99, true);
        assert_eq!(bs1.to_string(), "00000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000")
    }

    // Tests for other Traits implementations

    #[test]
    fn test_equality_trait_dbs() {
        let bs1 = DenseBitSet::from_integer(1234567890);
        let mut bs2 = DenseBitSet::from_integer(1234567891);

        bs2.set_bit(0, false); // The two bitsets are now equal
        assert_eq!(bs1, bs2);
    }

    #[test]
    fn test_equality_trait_dbse() {
        let mut bs1 = DenseBitSetExtended::with_capacity(2000);
        let mut bs2 = DenseBitSetExtended::with_capacity(2000);

        bs1.set_bit(1290, true);
        bs2.set_bit(1290, true); // The two bitsets are now equal
        assert_eq!(bs1, bs2);
    }

    #[test]
    fn test_bitand_dbs() {
        let bs1 = DenseBitSet::from_integer(0b10101);
        let bs2 = DenseBitSet::from_integer(0b11100);
        let bs3 = bs1 & bs2;
        assert_eq!(bs3.to_integer(), 0b10100);
    }

    #[test]
    fn test_bitand_dbse() {
        let mut bs1 = DenseBitSetExtended::with_capacity(10);
        let mut bs2 = DenseBitSetExtended::with_capacity(10);

        bs1.set_bit(1, true);
        bs1.set_bit(70, true);
        bs1.set_bit(72, true);
        bs1.set_bit(74, true);
        bs2.set_bit(1, true);
        bs2.set_bit(2, true);
        bs2.set_bit(72, true);
        bs2.set_bit(73, true);
        bs2.set_bit(74, true);

        let bs3 = bs1 & bs2;
        assert_eq!(bs3.to_string(), "00000000000000000000000000000000000000000000000000000101000000000000000000000000000000000000000000000000000000000000000000000010");
    }

    #[test]
    fn test_bitand_assign_dbs() {
        let bs1 = DenseBitSet::from_integer(0b11000);
        let mut bs2 = DenseBitSet::from_integer(0b1001);
        bs2 &= bs1;
        assert_eq!(bs2.to_integer(), 0b1000);
    }

    #[test]
    fn test_bitand_assign_dbse() {
        let mut bs1 = DenseBitSetExtended::with_capacity(10);
        let mut bs2 = DenseBitSetExtended::with_capacity(10);

        bs1.set_bit(1, true);
        bs1.set_bit(70, true);
        bs1.set_bit(72, true);
        bs1.set_bit(74, true);
        bs2.set_bit(1, true);
        bs2.set_bit(2, true);
        bs2.set_bit(72, true);
        bs2.set_bit(73, true);
        bs2.set_bit(74, true);
        bs2 &= bs1;

        assert_eq!(bs2.to_string(), "00000000000000000000000000000000000000000000000000000101000000000000000000000000000000000000000000000000000000000000000000000010");
    }

    #[test]
    fn test_bitor_dbs() {
        let bs1 = DenseBitSet::from_integer(0b10101);
        let bs2 = DenseBitSet::from_integer(0b11100);
        let bs3 = bs1 | bs2;
        assert_eq!(bs3.to_integer(), 0b11101);
    }

    #[test]
    fn test_bitor_dbse() {
        let mut bs1 = DenseBitSetExtended::with_capacity(10);
        let mut bs2 = DenseBitSetExtended::with_capacity(10);

        bs1.set_bit(1, true);
        bs1.set_bit(70, true);
        bs1.set_bit(72, true);
        bs1.set_bit(74, true);
        bs2.set_bit(1, true);
        bs2.set_bit(2, true);
        bs2.set_bit(72, true);
        bs2.set_bit(73, true);
        bs2.set_bit(74, true);

        let bs3 = bs2 | bs1;

        assert_eq!(bs3.to_string(), "00000000000000000000000000000000000000000000000000000111010000000000000000000000000000000000000000000000000000000000000000000110");
    }

    #[test]
    fn test_bitor_assign_dbs() {
        let bs1 = DenseBitSet::from_integer(0b11000);
        let mut bs2 = DenseBitSet::from_integer(0b1001);
        bs2 |= bs1;
        assert_eq!(bs2.to_integer(), 0b11001);
    }

    #[test]
    fn test_bitor_assign_dbse() {
        let mut bs1 = DenseBitSetExtended::with_capacity(10);
        let mut bs2 = DenseBitSetExtended::with_capacity(10);

        bs1.set_bit(1, true);
        bs1.set_bit(70, true);
        bs1.set_bit(72, true);
        bs1.set_bit(74, true);
        bs2.set_bit(1, true);
        bs2.set_bit(2, true);
        bs2.set_bit(72, true);
        bs2.set_bit(73, true);
        bs2.set_bit(74, true);
        bs2 |= bs1;

        assert_eq!(bs2.to_string(), "00000000000000000000000000000000000000000000000000000111010000000000000000000000000000000000000000000000000000000000000000000110");
    }

    #[test]
    fn test_bitxor_dbs() {
        let bs1 = DenseBitSet::from_integer(0b10101);
        let bs2 = DenseBitSet::from_integer(0b11100);
        let bs3 = bs1 ^ bs2;
        assert_eq!(bs3.to_integer(), 0b1001);
    }

    #[test]
    fn test_bitxor_dbse() {
        let mut bs1 = DenseBitSetExtended::with_capacity(10);
        let mut bs2 = DenseBitSetExtended::with_capacity(10);

        bs1.set_bit(1, true);
        bs1.set_bit(70, true);
        bs1.set_bit(72, true);
        bs1.set_bit(74, true);
        bs2.set_bit(1, true);
        bs2.set_bit(2, true);
        bs2.set_bit(72, true);
        bs2.set_bit(73, true);
        bs2.set_bit(74, true);

        let bs3 = bs2 ^ bs1;

        assert_eq!(bs3.to_string(), "00000000000000000000000000000000000000000000000000000010010000000000000000000000000000000000000000000000000000000000000000000100");
    }

    #[test]
    fn test_bitxor_assign_dbs() {
        let bs1 = DenseBitSet::from_integer(0b11000);
        let mut bs2 = DenseBitSet::from_integer(0b1001);
        bs2 ^= bs1;
        assert_eq!(bs2.to_integer(), 0b10001);
    }

    #[test]
    fn test_bitxor_assign_dbse() {
        let mut bs1 = DenseBitSetExtended::with_capacity(10);
        let mut bs2 = DenseBitSetExtended::with_capacity(10);

        bs1.set_bit(1, true);
        bs1.set_bit(70, true);
        bs1.set_bit(72, true);
        bs1.set_bit(74, true);
        bs2.set_bit(1, true);
        bs2.set_bit(2, true);
        bs2.set_bit(72, true);
        bs2.set_bit(73, true);
        bs2.set_bit(74, true);
        bs2 ^= bs1;

        assert_eq!(bs2.to_string(), "00000000000000000000000000000000000000000000000000000010010000000000000000000000000000000000000000000000000000000000000000000100");
    }

    #[test]
    fn test_not_dbs() {
        let mut bs1 = DenseBitSet::from_integer(0b111010100011101011);
        bs1 = !bs1;
        assert_eq!(
            bs1.to_integer(),
            0b1111111111111111111111111111111111111111111111000101011100010100
        );
    }

    #[test]
    fn test_not_dbse() {
        let mut bs1 = DenseBitSetExtended::from_string(String::from("ff00ff00ff00ff00"), 16);
        bs1 = !bs1;
        assert_eq!(
            bs1.to_string(),
            "0000000011111111000000001111111100000000111111110000000011111111"
        )
    }

    #[test]
    fn test_shl_dbs() {
        let bs1 = DenseBitSet::from_integer(0b10101);
        let bs2 = bs1 << 6;
        assert_eq!(bs2.to_integer(), 0b10101000000);
        let bs3 = bs1 << 65;
        assert!(bs3.none());
    }

    #[test]
    fn test_shl_dbse() {
        let mut bs1 = DenseBitSetExtended::with_capacity(2);
        bs1.set_bit(60, true);
        let bs2 = bs1 << 46;
        assert!(bs2.get_bit(106));
    }

    #[test]
    fn test_shl_assign_dbs() {
        let mut bs1 = DenseBitSet::from_integer(0b11000);
        bs1 <<= 6;
        assert_eq!(bs1.to_integer(), 0b11000000000);
        bs1 <<= 65;
        assert!(bs1.none());
    }

    #[test]
    fn test_shl_assign_dbse() {
        let mut bs1 = DenseBitSetExtended::with_capacity(2);
        bs1.set_bit(60, true);
        bs1 <<= 46;
        assert!(bs1.get_bit(106));
    }

    #[test]
    fn test_shr_dbs() {
        let bs1 = DenseBitSet::from_integer(0b101011111111111101);
        let bs2 = bs1 >> 6;
        assert_eq!(bs2.to_integer(), 0b101011111111);
        let bs3 = bs1 >> 65;
        assert!(bs3.none());
    }

    #[test]
    fn test_shr_dbse() {
        let mut bs1 = DenseBitSetExtended::with_capacity(2);
        bs1.set_bit(100, true);
        let bs2 = bs1 >> 46;
        assert!(bs2.get_bit(54));
    }

    #[test]
    fn test_shr_assign_dbs() {
        let mut bs1 = DenseBitSet::from_integer(0b1001111001101);
        bs1 >>= 6;
        assert_eq!(bs1.to_integer(), 0b1001111);
        bs1 >>= 65;
        assert!(bs1.none());
    }

    #[test]
    fn test_shr_assign_dbse() {
        let mut bs1 = DenseBitSetExtended::with_capacity(2);
        bs1.set_bit(100, true);
        bs1 >>= 46;
        assert!(bs1.get_bit(54));
    }

    // Test for README.md source code

    #[test]
    fn test_readme() {
        let mut bs = DenseBitSetExtended::from_string(
            String::from("f001eddadf411eddec0de5ca1ab1ec0feefeeb1e01dc0b01"),
            16,
        );
        let bs2 = DenseBitSetExtended::from_string(String::from("0J2aG5BaMRS443FEBRGS5DTMV2A"), 32);
        bs = bs.rotr(17) | (bs2 << 43);
        bs.set_bit(123, true);
        println!("{}", bs.subset(3, 64).to_string());
    }
}

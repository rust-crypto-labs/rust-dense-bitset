mod u64impl;
mod vec64impl;
mod bitset;

pub use crate::u64impl::DenseBitSet;
pub use crate::vec64impl::DenseBitSetExtended;
pub use crate::bitset::BitSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_dbs() {
        let bs1 = DenseBitSet::from_integer(7891234);
        let bs2 = DenseBitSet::from_integer(65536);
        assert_eq!(bs1.to_string(), "0000000000000000000000000000000000000000011110000110100100100010");
        assert_eq!(bs2.to_string(), "0000000000000000000000000000000000000000000000010000000000000000")
    }

    #[test]
    fn test_to_string_dbse() {
        let mut bs1 = DenseBitSetExtended::with_capacity(100);
        bs1.set_bit(99, true);
        assert_eq!(bs1.to_string(), "00000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000")
    }

    #[test]
    fn test_from_string_dbs(){
        let bs1 = DenseBitSet::from_string("101110001", 2);
        let bs2 = DenseBitSet::from_string("FFFFF", 16);
        let bs3 = DenseBitSet::from_string("123465", 10);

        assert_eq!(bs1.to_integer(),0b101110001);
        assert_eq!(bs2.to_integer(),0xfffff);
        assert_eq!(bs3.to_integer(),123465);
    }

    #[test]
    #[should_panic]
    fn catch_invalid_string_dbs(){
        let _bs = DenseBitSet::from_string("Hello World!", 12);
    }

    #[test]
    #[should_panic]
    fn catch_invalid_base_dbs(){
        let _bs = DenseBitSet::from_string("AZRZR=", 64);
    }

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
    fn test_hamming_weight_dbs() {
        let bs1 = DenseBitSet::from_integer(0);
        let bs2 = DenseBitSet::from_integer(1234567890);
        let bs3 = DenseBitSet::from_integer(u64::max_value());

        assert_eq!(bs1.get_weight(), 0);
        assert_eq!(bs2.get_weight(), 12);
        assert_eq!(bs3.get_weight(), 64);
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
    fn test_reset_dbs() {
        let mut bs = DenseBitSet::from_integer(1234567890);
        bs.reset();
        assert_eq!(bs.to_integer(), 0);
    }

    #[test]
    fn test_all_dbs() {
        let bs = DenseBitSet::from_integer(u64::max_value());
        assert_eq!(bs.all(), true);
    }

    #[test]
    fn test_any_dbs() {
        let bs = DenseBitSet::from_integer(1234567890);
        assert_eq!(bs.any(), true);
    }

    #[test]
    fn test_any_dbse() {
        let mut bs = DenseBitSetExtended::with_capacity(10);
        bs.set_bit(1234, true);
        assert_eq!(bs.any(), true);
    }

    #[test]
    fn test_none_dbs() {
        let bs = DenseBitSet::from_integer(0);
        assert_eq!(bs.none(), true);
    }

    #[test]
    fn test_none_dbse() {
        let mut bs = DenseBitSetExtended::with_capacity(10);
        bs.set_bit(1234, true);
        bs.set_bit(1234, false);
        assert_eq!(bs.none(), true);
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
    fn test_not_dbs() {
        let mut bs1 = DenseBitSet::from_integer(0b111010100011101011);
        bs1 = !bs1;
        assert_eq!(bs1.to_integer(), 0b1111111111111111111111111111111111111111111111000101011100010100);
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
    fn test_shl_assign_dbs() {
        let mut bs1 = DenseBitSet::from_integer(0b11000);
        bs1 <<= 6;
        assert_eq!(bs1.to_integer(), 0b11000000000);
        bs1 <<= 65;
        assert!(bs1.none());
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
    fn test_shr_assign_dbs() {
        let mut bs1 = DenseBitSet::from_integer(0b1001111001101);
        bs1 >>= 6;
        assert_eq!(bs1.to_integer(), 0b1001111);
        bs1 >>= 65;
        assert!(bs1.none());
    }

}


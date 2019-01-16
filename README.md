# rust-dense-bitset
[![Latest version](https://img.shields.io/crates/v/rust-dense-bitset.svg)](https://crates.io/crates/rust-dense-bitset)
[![Documentation](https://docs.rs/rust-dense-bitset/badge.svg)](https://docs.rs/rust-dense-bitset)
[![Build Status](https://travis-ci.org/ovheurdrive/rust-dense-bitset.svg?branch=master)](https://travis-ci.org/ovheurdrive/rust-dense-bitset)
![Long time support rustc version](https://img.shields.io/badge/rustc-1.31%2B-green.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

Efficient and flexible self-contained bitset rust library. 

The library is safe rust only, fully documented, and uses the most efficient algorithms whenever possible.

## Implementations

* `DenseBitSet` is a compact 64-bit bitset supporting in particular
    * Individual bit setting (`set_bit`) and getting (`get_bit`)
    * Bitwise operations `&, ^, |, !, <<, >>` and rotations
    * [Hamming weight](https://en.wikipedia.org/wiki/Hamming_weight), [bit reversal](https://en.wikipedia.org/wiki/Bit-reversal_permutation), [find first set](https://en.wikipedia.org/wiki/Find_first_set)
    * Conversion from and to integers and strings
    * Insertion and extraction of bitsets
* `DenseBitSetExtended` implements the same functionality, extending the bitset as necessary to accomodate as many bits as needed. Memory can be preallocated and new allocation is only performed if necessary.

### Usage 

```rust
use rust_dense_bitset::{BitSet, DenseBitSetExtended};

let mut bs = DenseBitSetExtended::from_string(
    String::from("f001eddadf411eddec0de5ca1ab1ec0feefeeb1e01dc0b01"),
    16,
);

let bs2 = DenseBitSetExtended::from_string(
    String::from("0J2aG5BaMRS443FEBRGS5DTMV2A"),
    32
);

bs = bs.rotr(17) | (bs2 << 43);
bs.set_bit(123, true);

println!("{}", bs.subset(3, 64).to_string());
```

### Known limits and caveats

- The data structure does not make use of compression and is therefore not particularly suited to sparse bitsets: in this scenario alternatives such as the [hibitset](https://github.com/slide-rs/hibitset) library can be considered instead.

- `clippy` incorrectly reports issues with "suspicious operators" in the shift operators. (To avoid errors we deactivated suspicious_op_assign_impl lint)

### Running the tests

Each individual function is tested. Run the tests with

```
cargo test
```

### Running the benchmarks

The `Criterion` dependency is used to provide precise benchmarkings. Benchmarks can be run with
```
cargo bench
```

## Documentation

Generate the documentation with

```
cargo doc
```

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

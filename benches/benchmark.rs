#[macro_use]
extern crate criterion;

extern crate rsdbs;

mod benchmarks {
    use criterion::Criterion;
    use rsdbs::{BitSet, DenseBitSet, DenseBitSetExtended};

    pub fn bench_reverse_dbs(c: &mut Criterion) {
        let bs = DenseBitSet::from_integer(666123);

        c.bench_function("dbs::reverse", move |b| b.iter(|| bs.reverse()));
    }
    pub fn bench_first_set_dbs(c: &mut Criterion) {
        let dbs = DenseBitSet::from_integer(256);
        c.bench_function("dbs::first_set", move |b| b.iter(|| dbs.first_set()));
    }
    pub fn bench_first_set_dbse(c: &mut Criterion) {
        let dbs = DenseBitSetExtended::from_dense_bitset(DenseBitSet::from_integer(256)) << 223;
        c.bench_function("dbse::first_set", move |b| b.iter(|| dbs.first_set()));
    }

    pub fn bench_reverse_dbse(c: &mut Criterion) {
        let bs = DenseBitSetExtended::from_dense_bitset(DenseBitSet::from_integer(666123)) >> 1063;
        c.bench_function("dbse::reverse", move |b| b.iter(|| bs.reverse()));
    }
    pub fn bench_to_string_dbs(c: &mut Criterion) {
        let bs = DenseBitSet::from_integer(7891234);
        c.bench_function("dbs::to_string", move |b| b.iter(|| bs.to_string()));
    }
    pub fn bench_to_string_dbse(c: &mut Criterion) {
        let mut bs = DenseBitSetExtended::with_capacity(100);
        bs.set_bit(99, true);
        c.bench_function("dbse::to_string", move |b| {
            b.iter(|| bs.clone().to_string())
        });
    }

    pub fn bench_from_string_dbs(c: &mut Criterion) {
        c.bench_function("dbs::from_string", |b| {
            b.iter(|| DenseBitSet::from_string("deadc0fee", 16))
        });
    }
    pub fn bench_from_string_dbse(c: &mut Criterion) {
        c.bench_function("dbse::from_string", |b| {
            b.iter(|| {
                DenseBitSetExtended::from_string(
                    String::from("f8d5215a52b57ea0aeb294af576a0aeb"),
                    16,
                )
            })
        });
    }
    pub fn bench_set_bit_dbs(c: &mut Criterion) {
        let mut bs = DenseBitSet::from_integer(0);
        c.bench_function("dbs::set_bit", move |b| b.iter(|| bs.set_bit(37, true)));
    }

    pub fn bench_set_bit_dbse(c: &mut Criterion) {
        let mut bs = DenseBitSetExtended::with_capacity(100);
        c.bench_function("dbse::set_bit", move |b| b.iter(|| bs.set_bit(37, true)));
    }
    pub fn bench_hamming_weight_dbs(c: &mut Criterion) {
        let bs = DenseBitSet::from_integer(1234567890);
        c.bench_function("dbs::hamming_weight", move |b| b.iter(|| bs.get_weight()));
    }
    pub fn bench_get_bit_dbs(c: &mut Criterion) {
        let bs = DenseBitSet::from_integer(1234567890);
        c.bench_function("dbs::get_bit", move |b| b.iter(|| bs.get_bit(37)));
    }
    pub fn bench_extract_dbs(c: &mut Criterion) {
        let bs = DenseBitSet::from_integer(1234567890);
        c.bench_function("dbs::extract", move |b| b.iter(|| bs.extract(5, 14)));
    }
    pub fn bench_insert_dbs(c: &mut Criterion) {
        let mut bs = DenseBitSet::from_integer(1234567890);
        c.bench_function("dbs::insert", move |b| b.iter(|| bs.insert(12, 2, 0b10)));
    }
    pub fn bench_equality_dbs(c: &mut Criterion) {
        let bs1 = DenseBitSet::from_integer(1234567890);
        let mut bs2 = DenseBitSet::from_integer(1234567891);

        bs2.set_bit(0, false); // The two bitsets are now equal

        c.bench_function("dbs::equals", move |b| b.iter(|| bs1 == bs2));
    }

    pub fn bench_equality_dbse(c: &mut Criterion) {
        let mut bs1 = DenseBitSetExtended::with_capacity(2000);
        let mut bs2 = DenseBitSetExtended::with_capacity(2000);

        bs1.set_bit(1290, true);
        bs2.set_bit(1290, true); // The two bitsets are now equal
        c.bench_function("dbse::equals", move |b| b.iter(|| bs1 == bs2));
    }
    pub fn bench_reset_dbs(c: &mut Criterion) {
        let mut bs1 = DenseBitSet::from_integer(1234567890);

        c.bench_function("dbs::reset", move |b| b.iter(|| bs1.reset()));
    }
    pub fn bench_all_dbs(c: &mut Criterion) {
        let bs = DenseBitSet::from_integer(u64::max_value());
        c.bench_function("dbs::all", move |b| b.iter(|| bs.all()));
    }

    pub fn bench_any_dbs(c: &mut Criterion) {
        let bs = DenseBitSet::from_integer(1234567890);
        c.bench_function("dbs::any", move |b| b.iter(|| bs.any()));
    }
    pub fn bench_any_dbse(c: &mut Criterion) {
        let mut bs = DenseBitSetExtended::with_capacity(10);
        bs.set_bit(1234, true);
        c.bench_function("dbse::any", move |b| b.iter(|| bs.any()));
    }
    pub fn bench_none_dbs(c: &mut Criterion) {
        let bs = DenseBitSet::from_integer(1234567890);
        c.bench_function("dbs::none", move |b| b.iter(|| bs.none()));
    }
    pub fn bench_none_dbse(c: &mut Criterion) {
        let mut bs = DenseBitSetExtended::with_capacity(10);
        bs.set_bit(1234, true);
        c.bench_function("dbse::none", move |b| b.iter(|| bs.none()));
    }
    pub fn bench_rotr_dbs(c: &mut Criterion) {
        let mut bs = DenseBitSet::from_integer(1234567890);
        c.bench_function("dbs::rotr", move |b| b.iter(|| bs.rotr(17)));
    }
    pub fn bench_rotl_dbs(c: &mut Criterion) {
        let mut bs = DenseBitSet::from_integer(1234567890);
        c.bench_function("dbs::rotl", move |b| b.iter(|| bs.rotl(17)));
    }
    pub fn bench_bitand_assign_dbs(c: &mut Criterion) {
        let mut bs1 = DenseBitSet::from_integer(0b10101);
        let bs2 = DenseBitSet::from_integer(0b11100);

        c.bench_function("dbs::and_assign", move |b| b.iter(|| bs1 &= bs2));
    }
    pub fn bench_bitand_dbs(c: &mut Criterion) {
        let bs1 = DenseBitSet::from_integer(0b10101);
        let bs2 = DenseBitSet::from_integer(0b11100);

        c.bench_function("dbs::and", move |b| b.iter(|| bs1 & bs2));
    }
    pub fn bench_bitor_dbs(c: &mut Criterion) {
        let bs1 = DenseBitSet::from_integer(0b10101);
        let bs2 = DenseBitSet::from_integer(0b11100);

        c.bench_function("dbs::or", move |b| b.iter(|| bs1 | bs2));
    }

    pub fn bench_bitor_assign_dbs(c: &mut Criterion) {
        let mut bs1 = DenseBitSet::from_integer(0b10101);
        let bs2 = DenseBitSet::from_integer(0b11100);

        c.bench_function("dbs::or_assign", move |b| b.iter(|| bs1 |= bs2));
    }

    pub fn bench_bitxor_assign_dbs(c: &mut Criterion) {
        let mut bs1 = DenseBitSet::from_integer(0b10101);
        let bs2 = DenseBitSet::from_integer(0b11100);

        c.bench_function("dbs::xor_assign", move |b| b.iter(|| bs1 ^= bs2));
    }

    pub fn bench_not_dbs(c: &mut Criterion) {
        let bs1 = DenseBitSet::from_integer(0b111010100011101011);

        c.bench_function("dbs::not", move |b| b.iter(|| !bs1));
    }

    pub fn bench_bitxor_dbs(c: &mut Criterion) {
        let bs1 = DenseBitSet::from_integer(0b10101);
        let bs2 = DenseBitSet::from_integer(0b11100);

        c.bench_function("dbs::xor", move |b| b.iter(|| bs1 ^ bs2));
    }

}

criterion_group!(
    benches,
    benchmarks::bench_all_dbs,
    benchmarks::bench_any_dbs,
    benchmarks::bench_any_dbse,
    benchmarks::bench_bitand_dbs,
    benchmarks::bench_bitand_assign_dbs,
    benchmarks::bench_bitor_dbs,
    benchmarks::bench_bitor_assign_dbs,
    benchmarks::bench_bitxor_dbs,
    benchmarks::bench_bitxor_assign_dbs,
    benchmarks::bench_equality_dbs,
    benchmarks::bench_equality_dbse,
    benchmarks::bench_extract_dbs,
    benchmarks::bench_first_set_dbs,
    benchmarks::bench_first_set_dbse,
    benchmarks::bench_from_string_dbs,
    benchmarks::bench_from_string_dbse,
    benchmarks::bench_get_bit_dbs,
    benchmarks::bench_hamming_weight_dbs,
    benchmarks::bench_insert_dbs,
    benchmarks::bench_none_dbs,
    benchmarks::bench_none_dbse,
    benchmarks::bench_not_dbs,
    benchmarks::bench_reset_dbs,
    benchmarks::bench_reverse_dbs,
    benchmarks::bench_reverse_dbse,
    benchmarks::bench_rotl_dbs,
    benchmarks::bench_rotr_dbs,
    benchmarks::bench_set_bit_dbs,
    benchmarks::bench_set_bit_dbse,
    benchmarks::bench_to_string_dbs,
    benchmarks::bench_to_string_dbse,
);

criterion_main!(benches);

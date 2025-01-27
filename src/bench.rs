use std::hint::black_box as bb;

use criterion::Criterion;

use duration_unit_lookup::*;

fn one_big_match(c: &mut Criterion) {
    c.bench_function(&format!("one-big-match/short"), |b| {
        b.iter(|| {
            let input = bb(b"y 5 months");
            let (unit, remaining) = one_big_match::lookup(input).unwrap();
            assert_eq!(unit, Unit::Year);
            assert_eq!(remaining, b" 5 months");
        })
    });
    c.bench_function(&format!("one-big-match/medium"), |b| {
        b.iter(|| {
            let input = bb(b"months 5 days");
            let (unit, remaining) = one_big_match::lookup(input).unwrap();
            assert_eq!(unit, Unit::Month);
            assert_eq!(remaining, b" 5 days");
        })
    });
    c.bench_function(&format!("one-big-match/long"), |b| {
        b.iter(|| {
            let input = bb(b"milliseconds 5 nanoseconds");
            let (unit, remaining) = one_big_match::lookup(input).unwrap();
            assert_eq!(unit, Unit::Millisecond);
            assert_eq!(remaining, b" 5 nanoseconds");
        })
    });
}

fn one_big_match_prefix(c: &mut Criterion) {
    c.bench_function(&format!("one-big-match-prefix/short"), |b| {
        b.iter(|| {
            let input = bb(b"y 5 months");
            let (unit, remaining) =
                one_big_match_prefix::lookup(input).unwrap();
            assert_eq!(unit, Unit::Year);
            assert_eq!(remaining, b" 5 months");
        })
    });
    c.bench_function(&format!("one-big-match-prefix/medium"), |b| {
        b.iter(|| {
            let input = bb(b"months 5 days");
            let (unit, remaining) =
                one_big_match_prefix::lookup(input).unwrap();
            assert_eq!(unit, Unit::Month);
            assert_eq!(remaining, b" 5 days");
        })
    });
    c.bench_function(&format!("one-big-match-prefix/long"), |b| {
        b.iter(|| {
            let input = bb(b"milliseconds 5 nanoseconds");
            let (unit, remaining) =
                one_big_match_prefix::lookup(input).unwrap();
            assert_eq!(unit, Unit::Millisecond);
            assert_eq!(remaining, b" 5 nanoseconds");
        })
    });
}

fn aho_corasick(c: &mut Criterion) {
    c.bench_function(&format!("aho-corasick/short"), |b| {
        b.iter(|| {
            let input = bb(b"y 5 months");
            let (unit, remaining) = aho::lookup(input).unwrap();
            assert_eq!(unit, Unit::Year);
            assert_eq!(remaining, b" 5 months");
        })
    });
    c.bench_function(&format!("aho-corasick/medium"), |b| {
        b.iter(|| {
            let input = bb(b"months 5 days");
            let (unit, remaining) = aho::lookup(input).unwrap();
            assert_eq!(unit, Unit::Month);
            assert_eq!(remaining, b" 5 days");
        })
    });
    c.bench_function(&format!("aho-corasick/long"), |b| {
        b.iter(|| {
            let input = bb(b"milliseconds 5 nanoseconds");
            let (unit, remaining) = aho::lookup(input).unwrap();
            assert_eq!(unit, Unit::Millisecond);
            assert_eq!(remaining, b" 5 nanoseconds");
        })
    });
}

fn phf(c: &mut Criterion) {
    c.bench_function(&format!("phf/short"), |b| {
        b.iter(|| {
            let input = bb(b"y 5 months");
            let (unit, remaining) = phf::lookup(input).unwrap();
            assert_eq!(unit, Unit::Year);
            assert_eq!(remaining, b" 5 months");
        })
    });
    c.bench_function(&format!("phf/medium"), |b| {
        b.iter(|| {
            let input = bb(b"months 5 days");
            let (unit, remaining) = phf::lookup(input).unwrap();
            assert_eq!(unit, Unit::Month);
            assert_eq!(remaining, b" 5 days");
        })
    });
    c.bench_function(&format!("phf/long"), |b| {
        b.iter(|| {
            let input = bb(b"milliseconds 5 nanoseconds");
            let (unit, remaining) = phf::lookup(input).unwrap();
            assert_eq!(unit, Unit::Millisecond);
            assert_eq!(remaining, b" 5 nanoseconds");
        })
    });
}

fn hashify(c: &mut Criterion) {
    c.bench_function(&format!("hashify/short"), |b| {
        b.iter(|| {
            let input = bb(b"y 5 months");
            let (unit, remaining) = hashify::lookup(input).unwrap();
            assert_eq!(unit, Unit::Year);
            assert_eq!(remaining, b" 5 months");
        })
    });
    c.bench_function(&format!("hashify/medium"), |b| {
        b.iter(|| {
            let input = bb(b"months 5 days");
            let (unit, remaining) = hashify::lookup(input).unwrap();
            assert_eq!(unit, Unit::Month);
            assert_eq!(remaining, b" 5 days");
        })
    });
    c.bench_function(&format!("hashify/long"), |b| {
        b.iter(|| {
            let input = bb(b"milliseconds 5 nanoseconds");
            let (unit, remaining) = hashify::lookup(input).unwrap();
            assert_eq!(unit, Unit::Millisecond);
            assert_eq!(remaining, b" 5 nanoseconds");
        })
    });
}

fn by_trie1(c: &mut Criterion) {
    c.bench_function(&format!("by-trie1/short"), |b| {
        b.iter(|| {
            let input = bb(b"y 5 months");
            let (unit, remaining) = by_trie1::lookup(input).unwrap();
            assert_eq!(unit, Unit::Year);
            assert_eq!(remaining, b" 5 months");
        })
    });
    c.bench_function(&format!("by-trie1/medium"), |b| {
        b.iter(|| {
            let input = bb(b"months 5 days");
            let (unit, remaining) = by_trie1::lookup(input).unwrap();
            assert_eq!(unit, Unit::Month);
            assert_eq!(remaining, b" 5 days");
        })
    });
    c.bench_function(&format!("by-trie1/long"), |b| {
        b.iter(|| {
            let input = bb(b"milliseconds 5 nanoseconds");
            let (unit, remaining) = by_trie1::lookup(input).unwrap();
            assert_eq!(unit, Unit::Millisecond);
            assert_eq!(remaining, b" 5 nanoseconds");
        })
    });
}

fn by_trie2(c: &mut Criterion) {
    c.bench_function(&format!("by-trie2/short"), |b| {
        b.iter(|| {
            let input = bb(b"y 5 months");
            let (unit, remaining) = by_trie2::lookup(input).unwrap();
            assert_eq!(unit, Unit::Year);
            assert_eq!(remaining, b" 5 months");
        })
    });
    c.bench_function(&format!("by-trie2/medium"), |b| {
        b.iter(|| {
            let input = bb(b"months 5 days");
            let (unit, remaining) = by_trie2::lookup(input).unwrap();
            assert_eq!(unit, Unit::Month);
            assert_eq!(remaining, b" 5 days");
        })
    });
    c.bench_function(&format!("by-trie2/long"), |b| {
        b.iter(|| {
            let input = bb(b"milliseconds 5 nanoseconds");
            let (unit, remaining) = by_trie2::lookup(input).unwrap();
            assert_eq!(unit, Unit::Millisecond);
            assert_eq!(remaining, b" 5 nanoseconds");
        })
    });
}

fn by_trie3(c: &mut Criterion) {
    c.bench_function(&format!("by-trie3/short"), |b| {
        b.iter(|| {
            let input = bb(b"y 5 months");
            let (unit, remaining) = by_trie3::lookup(input).unwrap();
            assert_eq!(unit, Unit::Year);
            assert_eq!(remaining, b" 5 months");
        })
    });
    c.bench_function(&format!("by-trie3/medium"), |b| {
        b.iter(|| {
            let input = bb(b"months 5 days");
            let (unit, remaining) = by_trie3::lookup(input).unwrap();
            assert_eq!(unit, Unit::Month);
            assert_eq!(remaining, b" 5 days");
        })
    });
    c.bench_function(&format!("by-trie3/long"), |b| {
        b.iter(|| {
            let input = bb(b"milliseconds 5 nanoseconds");
            let (unit, remaining) = by_trie3::lookup(input).unwrap();
            assert_eq!(unit, Unit::Millisecond);
            assert_eq!(remaining, b" 5 nanoseconds");
        })
    });
}

fn by_trie4(c: &mut Criterion) {
    c.bench_function(&format!("by-trie4/short"), |b| {
        b.iter(|| {
            let input = bb(b"y 5 months");
            let (unit, remaining) = by_trie4::lookup(input).unwrap();
            assert_eq!(unit, Unit::Year);
            assert_eq!(remaining, b" 5 months");
        })
    });
    c.bench_function(&format!("by-trie4/medium"), |b| {
        b.iter(|| {
            let input = bb(b"months 5 days");
            let (unit, remaining) = by_trie4::lookup(input).unwrap();
            assert_eq!(unit, Unit::Month);
            assert_eq!(remaining, b" 5 days");
        })
    });
    c.bench_function(&format!("by-trie4/long"), |b| {
        b.iter(|| {
            let input = bb(b"milliseconds 5 nanoseconds");
            let (unit, remaining) = by_trie4::lookup(input).unwrap();
            assert_eq!(unit, Unit::Millisecond);
            assert_eq!(remaining, b" 5 nanoseconds");
        })
    });
}

fn by_trie5(c: &mut Criterion) {
    c.bench_function(&format!("by-trie5/short"), |b| {
        b.iter(|| {
            let input = bb(b"y 5 months");
            let (unit, remaining) = by_trie5::lookup(input).unwrap();
            assert_eq!(unit, Unit::Year);
            assert_eq!(remaining, b" 5 months");
        })
    });
    c.bench_function(&format!("by-trie5/medium"), |b| {
        b.iter(|| {
            let input = bb(b"months 5 days");
            let (unit, remaining) = by_trie5::lookup(input).unwrap();
            assert_eq!(unit, Unit::Month);
            assert_eq!(remaining, b" 5 days");
        })
    });
    c.bench_function(&format!("by-trie5/long"), |b| {
        b.iter(|| {
            let input = bb(b"milliseconds 5 nanoseconds");
            let (unit, remaining) = by_trie5::lookup(input).unwrap();
            assert_eq!(unit, Unit::Millisecond);
            assert_eq!(remaining, b" 5 nanoseconds");
        })
    });
}

fn by_gendfa1(c: &mut Criterion) {
    c.bench_function(&format!("by-gendfa1/short"), |b| {
        b.iter(|| {
            let input = bb(b"y 5 months");
            let (unit, remaining) = by_gendfa1::lookup(input).unwrap();
            assert_eq!(unit, Unit::Year);
            assert_eq!(remaining, b" 5 months");
        })
    });
    c.bench_function(&format!("by-gendfa1/medium"), |b| {
        b.iter(|| {
            let input = bb(b"months 5 days");
            let (unit, remaining) = by_gendfa1::lookup(input).unwrap();
            assert_eq!(unit, Unit::Month);
            assert_eq!(remaining, b" 5 days");
        })
    });
    c.bench_function(&format!("by-gendfa1/long"), |b| {
        b.iter(|| {
            let input = bb(b"milliseconds 5 nanoseconds");
            let (unit, remaining) = by_gendfa1::lookup(input).unwrap();
            assert_eq!(unit, Unit::Millisecond);
            assert_eq!(remaining, b" 5 nanoseconds");
        })
    });
}

fn by_gencdfa1(c: &mut Criterion) {
    c.bench_function(&format!("by-gencdfa1/short"), |b| {
        b.iter(|| {
            let input = bb(b"y 5 months");
            let (unit, remaining) = by_gencdfa1::lookup(input).unwrap();
            assert_eq!(unit, Unit::Year);
            assert_eq!(remaining, b" 5 months");
        })
    });
    c.bench_function(&format!("by-gencdfa1/medium"), |b| {
        b.iter(|| {
            let input = bb(b"months 5 days");
            let (unit, remaining) = by_gencdfa1::lookup(input).unwrap();
            assert_eq!(unit, Unit::Month);
            assert_eq!(remaining, b" 5 days");
        })
    });
    c.bench_function(&format!("by-gencdfa1/long"), |b| {
        b.iter(|| {
            let input = bb(b"milliseconds 5 nanoseconds");
            let (unit, remaining) = by_gencdfa1::lookup(input).unwrap();
            assert_eq!(unit, Unit::Millisecond);
            assert_eq!(remaining, b" 5 nanoseconds");
        })
    });
}

criterion::criterion_group!(
    benches,
    one_big_match,
    one_big_match_prefix,
    aho_corasick,
    phf,
    hashify,
    by_trie1,
    by_trie2,
    by_trie3,
    by_trie4,
    by_trie5,
    by_gendfa1,
    by_gencdfa1,
);
criterion::criterion_main!(benches);

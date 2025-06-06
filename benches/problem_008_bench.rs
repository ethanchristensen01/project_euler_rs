use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use project_euler::problem_008::{BIG_NUM_STR, max_product_subslice, numstring_to_numslice};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let inputs = [4, 8, 12, 13, 16];
    c.bench_function("008_numstring_to_numslice", |b| {
        b.iter(|| numstring_to_numslice(black_box(BIG_NUM_STR)));
    });
    let nums = numstring_to_numslice(BIG_NUM_STR).expect("num str input should be valid");
    for input in inputs {
        c.bench_with_input(
            BenchmarkId::new("008_max_product_sublice", input),
            &input,
            |b, i| b.iter(|| max_product_subslice(black_box(&nums), black_box(*i))),
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

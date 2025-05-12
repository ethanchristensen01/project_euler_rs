use std::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use project_euler::problem_007::nth_prime;

fn criterion_benchmark(c: &mut Criterion) {
  let inputs: Vec<usize> = vec![
    10,
    100,
    1_000,
    10_000
  ];
  for input in inputs {
    c.bench_with_input(
      BenchmarkId::new("nth_prime", input),
      &input,
      |b, i| b.iter(|| nth_prime(black_box(*i)))
    );
  }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
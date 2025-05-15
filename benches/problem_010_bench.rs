use criterion::{criterion_group, criterion_main, Criterion};
use project_euler::{problem_010::Problem010, Problem};

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("010_sum_primes_lt_2M", |b| b.iter(Problem010::solve));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
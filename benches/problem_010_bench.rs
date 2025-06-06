use criterion::{Criterion, criterion_group, criterion_main};
use project_euler::{Problem, problem_010::Problem010};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("010_sum_primes_lt_2M", |b| b.iter(Problem010::solve));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

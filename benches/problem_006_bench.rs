use std::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use project_euler::problem_006::{find_palindrome_product, is_palindrome, Int};

fn criterion_benchmark(c: &mut Criterion) {
  let inputs: Vec<Int> = vec![
    123, 121,
    123_456, 123_321
  ];
  c.bench_function("006_find_palindrome_product", |b| b.iter(find_palindrome_product));
  for input in inputs {
    c.bench_with_input(
      BenchmarkId::new("006_is_palindrome", input),
      &input,
      |b, i| b.iter(|| is_palindrome(black_box(*i)))
    );
  }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
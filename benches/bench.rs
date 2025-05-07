use std::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use palindrome::{find_palindrome_product, is_palindrome, Int};

fn criterion_benchmark(c: &mut Criterion) {
  let inputs: Vec<Int> = vec!(
    123, 121,
    123456, 123326, 123351, 123421, 123321
  );
  c.bench_function("find_palindrome_product", |b| b.iter(find_palindrome_product));
  inputs.into_iter().for_each(|input| {
    c.bench_with_input(
      BenchmarkId::new("is_palindrome", input),
      &input,
      |b, i| b.iter(|| is_palindrome(black_box(*i)))
    );
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
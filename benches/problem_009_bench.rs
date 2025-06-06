use criterion::{Criterion, criterion_group, criterion_main};
use project_euler::problem_009::{find_pythagorean_triple_1000, get_possible_products};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("009_find_pythagorean_triple", |b| {
        b.iter(find_pythagorean_triple_1000);
    });
    c.bench_function("009_get_possible_factors", |b| {
        b.iter(get_possible_products);
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

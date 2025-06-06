use criterion::{Criterion, criterion_group, criterion_main};
use project_euler::{Problem, problem_011::Problem011};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(Problem011::name(), |b| b.iter(Problem011::solve));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

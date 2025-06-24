use criterion::{Criterion, criterion_group, criterion_main};
use project_euler::problem_007::{Solution007, Problem007};
use project_euler::{ProblemDescriptor, ProblemSolution};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(Problem007::get_descriptor_str().as_str(), |b| {
        b.iter(Solution007::solve);
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion::{Criterion, criterion_group, criterion_main};
use project_euler::problem_010::{Solution010, Problem010};
use project_euler::{ProblemDescriptor, ProblemSolution};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(Problem010::get_descriptor_str().as_str(), |b| {
        b.iter(Solution010::solve);
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

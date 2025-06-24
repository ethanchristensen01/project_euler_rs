use criterion::{Criterion, criterion_group, criterion_main};
use project_euler::problem_004::{Solution004, Problem004};
use project_euler::{ProblemDescriptor, ProblemSolution};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(Problem004::get_descriptor_str().as_str(), |b| {
        b.iter(Solution004::solve);
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion::{Criterion, criterion_group, criterion_main};
use extra_benchmark_testing::{sort_algo_1, sort_algo_2};

fn sort_benchmark(c: &mut Criterion) {
    let mut numbers: Vec<i32> = vec![
        1, 2, 3, 6, 5, 4, 8, 52, 2, 1, 5, 4, 4, 5, 8, 54, 2, 1, 0, 55, 5, 2, 0, 55, 5, 5,
    ];

    c.bench_function("Sorting Algorithm", |b| {
        b.iter(|| sort_algo_1(&mut numbers))
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);

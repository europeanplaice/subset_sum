use criterion::{black_box, criterion_group, criterion_main, Criterion};
use subset_sum::dp::find_subset;
use subset_sum::dp::find_subset_fast_only_positive;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("find_subset", 
        |b| b.iter(|| find_subset(black_box(&vec![1, 2, 3, 4, 5]), black_box(10))));

    c.bench_function("find_subset_fast_only_positive", 
    |b| b.iter(|| 
        find_subset_fast_only_positive(black_box(&vec![1, 2, 3, 4, 5]), 
        black_box(10))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
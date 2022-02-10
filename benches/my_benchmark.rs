use criterion::{black_box, criterion_group, criterion_main, Criterion};
use subset_sum::dp::find_subset;
use subset_sum::dp::find_subset_fast_only_positive;

fn criterion_benchmark(c: &mut Criterion) {
    let mut a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    a.sort();
    let a2: Vec<u32> = a.iter().map(|x| *x as u32).collect();
    c.bench_function("find_subset", 
        |b| b.iter(|| find_subset(black_box(&a), black_box(21))));

    c.bench_function("find_subset_fast_only_positive", 
    |b| b.iter(|| 
        find_subset_fast_only_positive(black_box(&a2), 
        black_box(21))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
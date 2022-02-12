use criterion::{black_box, criterion_group, criterion_main, Criterion};
use subset_sum::dp::find_subset;
use subset_sum::dp::find_subset_fast_only_positive;

fn criterion_benchmark(c: &mut Criterion) {
    let a = vec![5, 6, 7, 8, 9, 10, 11];
    let a2: Vec<u32> = a.iter().map(|x| *x as u32).collect();
    c.bench_function("find_subset", 
        |b| b.iter(|| find_subset(&a, 21)));

    c.bench_function("find_subset_fast_only_positive", 
    |b| b.iter(|| 
        find_subset_fast_only_positive(&a2, 
        21)));

    let d = vec![-3,  10,  56, -33,  65,  -9,   8,  72,  63,  35];
    c.bench_function("find_subset_complicated", 
        |b| b.iter(|| find_subset(&d, 7)));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use esprit_index::{all_files, index};

const SAMPLE_SIZE: usize = 60;

fn benchmark_all_files(c: &mut Criterion) {
    let mut group = c.benchmark_group("index");
    group.sample_size(SAMPLE_SIZE);
    group.bench_function("index_all_files", |b| {
        b.iter(|| {
            let result = all_files().expect("all_files failed");
            black_box(result);
        });
    });
    group.finish();
}

fn benchmark_index_workspace(c: &mut Criterion) {
    let root = std::env::current_dir().expect("failed to get current directory");
    let mut group = c.benchmark_group("index");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("index_workspace", |b| {
        b.iter(|| {
            let result = index(black_box(&root)).expect("index failed");
            black_box(result);
        });
    });
    group.finish();
}

fn benchmark_search(c: &mut Criterion) {
    let query = black_box("Cargo");
    let mut group = c.benchmark_group("search");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("search_query", |b| {
        b.iter(|| {
            let result = esprit_index::search(query).expect("search failed");
            black_box(result);
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    benchmark_all_files,
    benchmark_index_workspace,
    benchmark_search
);

criterion_main!(benches);

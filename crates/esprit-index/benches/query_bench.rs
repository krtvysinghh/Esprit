use criterion::{criterion_group, criterion_main, Criterion};
use esprit_index::all_files;

fn benchmark_all_files(c: &mut Criterion) {
    c.bench_function("query/all_files", |b| {
        b.iter(|| {
            let files = all_files().expect("all_files failed");
            std::hint::black_box(files);
        });
    });
}

criterion_group!(benches, benchmark_all_files);
criterion_main!(benches);

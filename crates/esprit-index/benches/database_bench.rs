use criterion::{criterion_group, criterion_main, Criterion};
use esprit_index::{delete_file, insert_file, rename_file, update_file, IndexDatabase};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

fn bench_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock error")
        .as_nanos();

    std::env::temp_dir().join(format!("esprit-bench-{name}-{nanos}.txt"))
}

fn benchmark_persistent_insert(c: &mut Criterion) {
    let path = bench_path("persistent-insert");
    fs::write(&path, "benchmark").expect("failed to create benchmark file");
    let database = IndexDatabase::open().expect("failed to open database");

    c.bench_function("database/persistent_insert_file", |b| {
        b.iter(|| {
            database
                .insert_file(&path)
                .expect("persistent insert failed");
        });
    });

    let _ = database.delete_file(&path);
    let _ = fs::remove_file(&path);
}

fn benchmark_persistent_update(c: &mut Criterion) {
    let path = bench_path("persistent-update");
    fs::write(&path, "benchmark").expect("failed to create benchmark file");
    let database = IndexDatabase::open().expect("failed to open database");
    database.insert_file(&path).expect("initial insert failed");

    c.bench_function("database/persistent_update_file", |b| {
        b.iter(|| {
            database
                .update_file(&path)
                .expect("persistent update failed");
        });
    });

    let _ = database.delete_file(&path);
    let _ = fs::remove_file(&path);
}

fn benchmark_insert(c: &mut Criterion) {
    let path = bench_path("insert");
    fs::write(&path, "benchmark").expect("failed to create benchmark file");

    c.bench_function("database/insert_file", |b| {
        b.iter(|| {
            insert_file(&path).expect("insert_file failed");
        });
    });

    let _ = delete_file(&path);
    let _ = fs::remove_file(&path);
}

fn benchmark_update(c: &mut Criterion) {
    let path = bench_path("update");
    fs::write(&path, "benchmark").expect("failed to create benchmark file");
    insert_file(&path).expect("initial insert failed");

    c.bench_function("database/update_file", |b| {
        b.iter(|| {
            update_file(&path).expect("update_file failed");
        });
    });

    let _ = delete_file(&path);
    let _ = fs::remove_file(&path);
}

fn benchmark_delete(c: &mut Criterion) {
    let path = bench_path("delete");
    fs::write(&path, "benchmark").expect("failed to create benchmark file");

    c.bench_function("database/delete_file", |b| {
        b.iter(|| {
            insert_file(&path).expect("insert_file failed");
            delete_file(&path).expect("delete_file failed");
        });
    });

    let _ = delete_file(&path);
    let _ = fs::remove_file(&path);
}

fn benchmark_rename(c: &mut Criterion) {
    let old = bench_path("rename-old");
    let new = bench_path("rename-new");

    fs::write(&old, "benchmark").expect("failed to create benchmark file");
    insert_file(&old).expect("initial insert failed");

    c.bench_function("database/rename_file", |b| {
        b.iter(|| {
            if new.exists() {
                let _ = fs::remove_file(&new);
            }

            fs::rename(&old, &new).expect("filesystem rename failed");
            rename_file(&old, &new).expect("rename_file failed");
            fs::rename(&new, &old).expect("filesystem rename failed");
            rename_file(&new, &old).expect("rename_file failed");
        });
    });

    let _ = delete_file(&old);
    let _ = delete_file(&new);
    let _ = fs::remove_file(&old);
    let _ = fs::remove_file(&new);
}

criterion_group!(
    benches,
    benchmark_persistent_insert,
    benchmark_persistent_update,
    benchmark_insert,
    benchmark_update,
    benchmark_delete,
    benchmark_rename
);
criterion_main!(benches);

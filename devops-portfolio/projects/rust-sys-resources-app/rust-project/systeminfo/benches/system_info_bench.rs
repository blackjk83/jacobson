use criterion::{black_box, criterion_group, criterion_main, Criterion};
use systeminfo::get_system_info;

fn benchmark_system_info(c: &mut Criterion) {
    c.bench_function("get_system_info", |b| {
        b.iter(|| {
            black_box(get_system_info().unwrap());
        })
    });
}

criterion_group!(benches, benchmark_system_info);
criterion_main!(benches);

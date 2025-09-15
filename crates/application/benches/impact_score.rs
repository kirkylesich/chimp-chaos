use application::usecases::compute_impact_score;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_impact_score(c: &mut Criterion) {
    c.bench_function("impact_score", |b| {
        b.iter(|| {
            let _ = compute_impact_score(100.0, 60.0, 200.0, 300.0).unwrap();
        })
    });
}

criterion_group!(benches, bench_impact_score);
criterion_main!(benches);

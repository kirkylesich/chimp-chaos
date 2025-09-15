use criterion::{criterion_group, criterion_main, Criterion};

fn bench_promql_parse(c: &mut Criterion) {
    c.bench_function("promql_window_format", |b| {
        b.iter(|| {
            let d = 120i64;
            let baseline = format!("now-{}s_to_now-{}s", 2 * d, d);
            let failure = format!("now_to_now+{}s", d);
            assert!(!baseline.is_empty() && !failure.is_empty());
        })
    });
}

criterion_group!(benches, bench_promql_parse);
criterion_main!(benches);

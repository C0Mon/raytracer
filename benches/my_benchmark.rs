use criterion::{Criterion, criterion_group, criterion_main};

fn render_bench() -> std::io::Result<()> {
    raytracer::run()?;
    Ok(())
}

fn my_benchmark(c: &mut Criterion) {
    c.bench_function("RenderBench", |b| b.iter(|| render_bench()));
}

fn custom_criterion() -> Criterion {
    Criterion::default().sample_size(10)
}

criterion_group! {
    name = benches;
    config = custom_criterion();
    targets = my_benchmark
}
criterion_main!(benches);

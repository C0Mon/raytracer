use criterion::{criterion_group, criterion_main, Criterion};

fn render_bench() -> std::io::Result<()>  {
    raytracer::run()?;
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("RenderBench", |b| b.iter(||render_bench()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
#[cfg(feature = "bench")]
use airdash_id::SnowflakeGenerator;
#[cfg(feature = "bench")]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[cfg(feature = "bench")]
pub fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("snowflake_generator", |b| {
    b.iter(|| {
      let mut generator = SnowflakeGenerator::new(23, 12);

      black_box(generator.generate())
    })
  });

  c.bench_function("snowflake_generator_group", |b| {
    b.iter(|| {
      let generator = SnowflakeGenerator::new(23, 12);

      black_box(generator.take(50_000).collect::<Vec<_>>())
    })
  });
}

#[cfg(feature = "bench")]
criterion_group!(benches, criterion_benchmark);
#[cfg(feature = "bench")]
criterion_main!(benches);

#[cfg(not(feature = "bench"))]
fn main() {}

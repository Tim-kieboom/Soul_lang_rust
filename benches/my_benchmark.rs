use criterion::{criterion_group, criterion_main, Criterion};

fn bench_tokenize_file(_c: &mut Criterion) {
	
}

criterion_group!(benches, bench_tokenize_file);
criterion_main!(benches);
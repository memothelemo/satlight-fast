use criterion::{black_box, criterion_group, criterion_main, Criterion};
use satlight_parser::tokenizer::tokenize as tokens;

// #[global_allocator]
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

const SAMPLE_SOURCE: &str = include_str!("sample.txt");

fn tokenize(criterion: &mut Criterion) {
    #[allow(unused_must_use)]
    criterion.bench_function("tokenize sample.txt", |b| {
        b.iter(|| for _token in tokens(black_box(SAMPLE_SOURCE)) {})
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = tokenize
}

criterion_main!(benches);

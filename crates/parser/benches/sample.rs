use criterion::{black_box, criterion_group, criterion_main, Criterion};
use satlight_parser::{
    parser::Parser,
    tokenizer::{tokenize as tokens, Tokens},
};

const SAMPLE_SOURCE: &str = include_str!("sample.txt");

fn tokenize(criterion: &mut Criterion) {
    #[allow(unused_must_use)]
    criterion.bench_function("tokenize sample.txt", |b| {
        b.iter(|| for _token in tokens(black_box(SAMPLE_SOURCE)) {})
    });

    #[allow(unused_must_use)]
    criterion.bench_function("parse sample.txt", |b| {
        b.iter(|| {
            let mut parser = Parser::new(Tokens::new(black_box(SAMPLE_SOURCE), true));
            parser.parse_ast().unwrap()
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = tokenize
}

criterion_main!(benches);

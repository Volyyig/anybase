use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use anybase::convert_base;
use anybase::Converter;
fn bench_convert_base(c: &mut Criterion) {
    let src_table = "0123456789abcdefghijklmnopqrstuvwxyz";
    let dst_table = "0123456789ABCDEF";

    // Construct a large integer string
    let input = "z".repeat(1000); // 1000 characters in base36

    c.bench_function("default", |b| {
        b.iter(|| {
            let out = convert_base(
                black_box(&input),
                black_box(src_table),
                black_box(dst_table),
            )
            .unwrap();
            black_box(out);
        })
    });


    c.bench_function("object", |b| {
        b.iter(|| {
            let out = Converter::new(src_table, dst_table)
                .convert(black_box(&input))
            .unwrap();
            black_box(out);
        })
    });
}

criterion_group!(benches, bench_convert_base);
criterion_main!(benches);
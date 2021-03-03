use criterion::{criterion_group, criterion_main, Criterion};

use chess::board::Board;

fn benchmark_new_board(c: &mut Criterion) {
    c.bench_function("new board", |bencher| {
        bencher.iter(|| Board::new());
    });
}

criterion_group!(benches, benchmark_new_board);
criterion_main!(benches);
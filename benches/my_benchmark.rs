use criterion::{criterion_group, criterion_main, Criterion};
use rusty_chess::board::mailbox::Mailbox;

fn bench_start_pos_move_gen(c: &mut Criterion) {
    let start_pos = Mailbox::setup_board(None).unwrap();
    c.bench_function("start pos move gen", |b| {
        b.iter(|| start_pos.get_valid_moves())
    });
}

fn bench_kiwipete_move_gen(c: &mut Criterion) {
    let start_pos = Mailbox::setup_board(None).unwrap();
    c.bench_function("kiwipete move gen", |b| {
        b.iter(|| start_pos.get_valid_moves())
    });
}

criterion_group!(
    movegen_tests,
    bench_start_pos_move_gen,
    bench_kiwipete_move_gen
);
criterion_main!(movegen_tests);

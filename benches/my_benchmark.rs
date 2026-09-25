use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rusty_chess::board::mailbox::Mailbox;

fn benchmark_start_pos_move_gen(c: &mut Criterion) {
    let start_pos = Mailbox::setup_board(None).unwrap();
    c.bench_function("start pos move gen", |b| {
        b.iter(|| start_pos.get_valid_moves())
    });
}

criterion_group!(board_tests, benchmark_start_pos_move_gen);
criterion_main!(board_tests);

extern crate criterion;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn fast_fibonacci(nth:u64) -> u64  {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for _ in 1..nth {
        c = a+b;
        a = b;
        b = c;
    }
    c
}

fn bench_fib_slow(c: &mut Criterion) {
    c.bench_function("slow fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

fn bench_fib_fast(c: &mut Criterion) {
    c.bench_function("fast fib 20", |b| b.iter(|| fast_fibonacci(black_box(20))));
}


criterion_group!(benches, bench_fib_slow, bench_fib_fast);  // 基准测试
criterion_main!(benches);  // cargo bench
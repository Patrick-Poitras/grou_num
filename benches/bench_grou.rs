use criterion::{black_box, criterion_group, criterion_main, Criterion};

use grou_num::grou::Grou;

fn grou_create_clone(c: &mut Criterion) {
    c.bench_function("create", |b| (b.iter(|| {
        let x = Grou::from(black_box(vec![100]));
        let y = x.clone();
        black_box(x == y);
    })));
}

fn grou_add(c: &mut Criterion) {
    let x = black_box(Grou::from(vec![1,2,3,4,5]));
    let y = black_box(x.clone());
    c.bench_function("add-5digits", |b| {
        b.iter(|| &x + &y );
    });
}

fn grou_add_assign(c: &mut Criterion) {
    let mut x = black_box(Grou::from(0));
    let y = Grou::from(vec![1,1,1,2,2,2]);
    c.bench_function("add-assign-x1000", |b| (b.iter(|| {
        for _ in 0..1000 {
            x += &y;
        }
    })));
}

fn grou_verylarge_addition(c : &mut Criterion) {
    let mut base_vector: Vec<u32> = Vec::new();
    for i in 0u32..500u32 {
        base_vector.push(i);
    }
    let x = black_box(Grou::from(base_vector.clone()));
    let y = black_box(Grou::from(base_vector.clone()));
    c.bench_function("add-verylarge", |b| {
        b.iter(|| &x + &y);
    });
}

fn grou_fibonacci(n: usize){
    let mut x = black_box(Grou::from(1));
    let mut y = black_box(Grou::from(1));

    for _ in 0..n {
        let z = &x + &y;
        x = y;
        y = z;
    }
}

fn grou_fibonacci_1000(c: &mut Criterion) {
    c.bench_function("fibonacci-1000", |b| {
        b.iter(|| grou_fibonacci(1000));
    });
}

fn grou_fibonacci_5000(c: &mut Criterion) {
    c.bench_function("fibonacci-5000", |b| {
        b.iter(|| grou_fibonacci(5000));
    });
}

criterion_group!(grou_addition, 
    grou_create_clone,
    grou_add,
    grou_add_assign,
    grou_verylarge_addition,
    grou_fibonacci_1000,
    grou_fibonacci_5000);

criterion_main!(grou_addition);
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
    let mut base_vector: Vec<u64> = Vec::new();
    for i in 0u64..500u64 {
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

fn grou_fibonacci_100k(c: &mut Criterion) {
    c.bench_function("fibonacci-100k", |b| {
        b.iter(|| grou_fibonacci(1_00_000));
    });
}

fn partial_eq_len_10(c : &mut Criterion) {
    let x = black_box(Grou::from(vec![1,2,3,4,5,6,7,8,9,10]));
    let y = Grou::from(vec![0,2,3,4,5,6,7,8,9,10]);

    c.bench_function("partial_eq-len-10", |b| {
        b.iter(|| &x > &y)
    });
}

fn partial_eq_len_500(c : &mut Criterion) {
    let mut v : Vec<u64> = Vec::new();
    let mut w : Vec<u64> = Vec::new();
    v.push(1);
    w.push(0);

    for i in 2..=500 {
        v.push(i);
        w.push(i);
    }

    let v = Grou::from(v);
    let w = Grou::from(w);

    c.bench_function("partial_eq-len-500", |b| {
        b.iter(|| &v > &w);
    });
}

fn sub_len_50(c : &mut Criterion) {
    let mut v = Vec::<u64>::new();
    let mut w = black_box(Vec::<u64>::new());

    for i in 0..50 {
        v.push(i);
        w.push(i*3);
    }

    let v = Grou::from(v);
    let w = Grou::from(w);

    c.bench_function("sub-len-50", |b| {
        b.iter(|| &w - &v);
    });
}

fn sub_len_500(c : &mut Criterion) {
    let mut v = Vec::<u64>::new();
    let mut w = black_box(Vec::<u64>::new());

    for i in 0..500 {
        v.push(i);
        w.push(i * 3);
    }

    let v = Grou::from(v);
    let w = Grou::from(w);

    c.bench_function("sub-len-500", |b| {
        b.iter(|| &w - &v);
    });
}

criterion_group!(grou_addition, 
    grou_create_clone,
    grou_add,
    grou_add_assign,
    grou_verylarge_addition,
    grou_fibonacci_1000,
    grou_fibonacci_5000,
    partial_eq_len_10,
    partial_eq_len_500,
    sub_len_50,
    sub_len_500,
    grou_fibonacci_100k,
    );

criterion_main!(grou_addition);
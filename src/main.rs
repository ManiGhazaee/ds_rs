use std::{thread::Builder, time::Instant};

use ds_rs::{
    matrix::Matrix,
    PerfRelative,
};
use rand::Rng;

#[allow(dead_code)]
fn perf_test() {
    let mut l1 = ds_rs::linked_list::LinkedList::new();
    let mut l2 = std::collections::linked_list::LinkedList::new();
    let perf = PerfRelative::new("ds_rs", "std");

    perf.test(
        "push_back",
        10000,
        || {
            l1.push_back(42);
        },
        || {
            l2.push_back(42);
        },
    );

    perf.test(
        "pop_back",
        5000,
        || {
            let _ = l1.pop_back();
        },
        || {
            let _ = l2.pop_back();
        },
    );

    perf.test(
        "pop_front",
        5000,
        || {
            let _ = l1.pop_front();
        },
        || {
            let _ = l2.pop_front();
        },
    );

    perf.test(
        "push_front",
        1000,
        || {
            let _ = l1.push_front(68);
        },
        || {
            let _ = l2.push_front(68);
        },
    );

    let mut iter1 = l1.iter();
    let mut iter2 = l2.iter();
    perf.test(
        "iter",
        900,
        || {
            iter1.next();
        },
        || {
            iter2.next();
        },
    );

    perf.test(
        "clear",
        1,
        || {
            l1.clear();
        },
        || {
            l2.clear();
        },
    );
}

fn main() {
    let mut rng = rand::thread_rng();

    const N: usize = 1024;
    const _BYTES: usize = 4 * 8 * N * N * 2;
    const FLOP: f32 = (N * N * 2 * N) as f32;

    let v1 = (0..N)
        .map(|_| (0..N).map(|_| rng.gen::<f32>()).collect::<Vec<f32>>())
        .collect::<Vec<Vec<f32>>>();

    let v2 = (0..N)
        .map(|_| (0..N).map(|_| rng.gen::<f32>()).collect::<Vec<f32>>())
        .collect::<Vec<Vec<f32>>>();

    Builder::new()
        .stack_size(1_500_000_000)
        .spawn(move || {
            let a1 = Matrix::<f32, N, N>::from(&v1);
            let a2 = Matrix::<f32, N, N>::from(&v2);

            // let av1 = MatrixVec::new(v1);
            // let av2 = MatrixVec::new(v2);

            println!("filled");

            let inst = Instant::now();
            let a3 = a1.mult_par_transpose(&a2);
            let elpsd1 = inst.elapsed();

            // let inst = Instant::now();
            // let av3 = av1.mult_transpose(&av2);
            // let elpsd2 = inst.elapsed();

            println!("{}", a3.get(0, 0).unwrap());
            println!("{}", a3.get(1023, 1023).unwrap());
            // println!("{}", av3.get(0, 0).unwrap());
            // println!("{}", av3.get(1023, 1023).unwrap());
            println!("Matrix    GFLOP/S: {}", (FLOP / elpsd1.as_secs_f32()) / 1e9);
            // println!("MatrixVec GFLOP/S: {}", (FLOP / elpsd2.as_secs_f32()) / 1e9);
        })
        .unwrap()
        .join()
        .unwrap();
}

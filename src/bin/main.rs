use std::{env, thread::Builder, time::Instant};

use ds_rs::
    matrix::{array, vec}
;
use rand::Rng;

fn main() {
    env::set_var("RUST_BACKTRACE", "true");
}

#[allow(dead_code)]
fn linked_list() {
    let mut l1 = ds_rs::linked_list::rawptr::LinkedList::new();
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

#[allow(dead_code)]
fn matrix_mult() {
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
            let a1 = array::Matrix::<f32, N, N>::from(&v1);
            let a2 = array::Matrix::<f32, N, N>::from(&v2);

            let av1 = vec::Matrix::new(v1);
            let av2 = vec::Matrix::new(v2);

            println!("filled");

            let inst = Instant::now();
            let a3 = a1.mult_par_transpose(&a2);
            let elpsd1 = inst.elapsed();

            let inst = Instant::now();
            let av3 = av1.mult_transpose(&av2);
            let elpsd2 = inst.elapsed();

            println!("{}", a3.get(0, 0).unwrap());
            println!("{}", a3.get(1023, 1023).unwrap());
            println!("{}", av3.get(0, 0).unwrap());
            println!("{}", av3.get(1023, 1023).unwrap());
            println!("Matrix    GFLOP/S: {}", (FLOP / elpsd1.as_secs_f32()) / 1e9);
            println!(
                "vec::Matrix GFLOP/S: {}",
                (FLOP / elpsd2.as_secs_f32()) / 1e9
            );
        })
        .unwrap()
        .join()
        .unwrap();
}

#[allow(dead_code)]
fn matrix_add() {
    let mut rng = rand::thread_rng();
    const N: usize = 1024;
    let v1 = (0..N)
        .map(|_| (0..N).map(|_| rng.gen::<f32>()).collect::<Vec<f32>>())
        .collect::<Vec<Vec<f32>>>();
    let v2 = (0..N)
        .map(|_| (0..N).map(|_| rng.gen::<f32>()).collect::<Vec<f32>>())
        .collect::<Vec<Vec<f32>>>();
    Builder::new()
        .stack_size(1_500_000_000)
        .spawn(move || {
            let perf = PerfRelative::new("add", "add_par");
            let m1 = array::Matrix::<f32, N, N>::from(&v1);
            let m2 = array::Matrix::<f32, N, N>::from(&v2);

            let mut x = array::Matrix::<f32, N, N>::from(&vec![vec![0.0; N]; N]);
            let mut y = array::Matrix::<f32, N, N>::from(&vec![vec![0.0; N]; N]);

            perf.test("x", 1, || x = m1.add(&m2), || y = m1.add_par(&m2));
        })
        .unwrap()
        .join()
        .unwrap();
}

pub struct PerfRelative<'a> {
    f1_name: &'a str,
    f2_name: &'a str,
}

impl<'a> PerfRelative<'a> {
    pub fn new(f1_name: &'a str, f2_name: &'a str) -> Self {
        PerfRelative { f1_name, f2_name }
    }
    pub fn test<F1, F2>(&self, name: &str, iterations: usize, mut f1: F1, mut f2: F2)
    where
        F1: FnMut(),
        F2: FnMut(),
    {
        let name1 = self.f1_name;
        let name2 = self.f2_name;

        let start_time1 = Instant::now();
        for _ in 0..iterations {
            f1();
        }
        let end_time1 = Instant::now();
        let elapsed_time1 = end_time1 - start_time1;

        let start_time2 = Instant::now();
        for _ in 0..iterations {
            f2();
        }
        let end_time2 = Instant::now();
        let elapsed_time2 = end_time2 - start_time2;

        let average_time_per_iteration1 = elapsed_time1 / iterations as u32;
        let average_time_per_iteration2 = elapsed_time2 / iterations as u32;

        println!("{} {} iters per iter:", name, iterations);
        println!("    {}: {:?}", name1, average_time_per_iteration1);
        println!("    {}: {:?}", name2, average_time_per_iteration2);

        let speed_ratio = if elapsed_time1 < elapsed_time2 {
            elapsed_time2.as_secs_f64() / elapsed_time1.as_secs_f64()
        } else {
            elapsed_time1.as_secs_f64() / elapsed_time2.as_secs_f64()
        };

        if elapsed_time1 < elapsed_time2 {
            println!("    {} {:.2}x faster than {}", name1, speed_ratio, name2);
        } else {
            println!("    {} {:.2}x faster than {}", name2, speed_ratio, name1);
        }
    }
}

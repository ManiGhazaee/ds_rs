use std::{cmp::Ordering, time::Instant};

use rand::Rng;

pub mod binary_tree;
pub mod linked_list;
pub mod queue;
pub mod graph;
pub mod matrix;

pub enum DurationType {
    Nano,
    Micro,
    Milli,
}

pub fn perf<F>(name: &str, duration_type: DurationType, mut f: F)
where
    F: FnMut(),
{
    const NUM_ITERATIONS: u128 = 1000;

    let start_time = Instant::now();
    for _ in 0..NUM_ITERATIONS {
        f();
    }

    let elapsed_time = start_time.elapsed();
    let (duration, d_type) = match duration_type {
        DurationType::Nano => (elapsed_time.as_nanos(), "nano s"),
        DurationType::Micro => (elapsed_time.as_micros(), "micro s"),
        DurationType::Milli => (elapsed_time.as_millis(), "ms"),
    };
    let average_time_per_iteration = duration / NUM_ITERATIONS;

    println!("{}: {:?} {}", name, average_time_per_iteration, d_type);
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

pub fn is_heap_by<T, F>(vec: &[binary_tree::Node<T>], compare: F) -> bool
where
    F: Fn(&T, &T) -> Ordering,
{
    for i in (0..vec.len()).rev() {
        let parent = if !vec[i].is_root() {
            vec[i].parent()
        } else {
            continue;
        };
        let vals = if let (Some(val), Some(pval)) = (vec[i].val(), parent.val()) {
            (val, pval)
        } else {
            continue;
        };
        if let Ordering::Less = compare(&*vals.0, &*vals.1) {
            return false;
        }
    }
    true
}

pub fn is_max_heap<T: PartialOrd>(vec: &Vec<binary_tree::Node<T>>) -> bool {
    is_heap_by(&vec[..], |a, b| b.partial_cmp(a).unwrap())
}

pub fn is_min_heap<T: PartialOrd>(vec: &Vec<binary_tree::Node<T>>) -> bool {
    is_heap_by(&vec[..], |a, b| a.partial_cmp(b).unwrap())
}

pub fn rand_vec_gen(len: usize) -> Vec<isize> {
    let mut rng = rand::thread_rng();
    (0..len)
        .into_iter()
        .map(|_| rng.gen_range(isize::MIN..isize::MAX))
        .collect()
}

use std::fmt::Debug;

pub mod bigint;
pub mod graph;
pub mod linked_list;
pub mod matrix;
pub mod queue;
pub mod stack;
pub mod tree;

/// t1: len,
/// t2: element
#[macro_export]
macro_rules! rng_vec {
    ($t1:ty, $t2:ty) => {{
        let mut rng = rand::thread_rng();
        let len: $t1 = rand::Rng::gen(&mut rng);
        let v: Vec<$t2> = (0..len as usize)
            .map(|_| rand::Rng::gen(&mut rng))
            .collect();
        v
    }};
}

pub fn circular_slice_assert_eq<T: Clone + PartialEq + Debug>(x: &[T], y: &[T]) {
    let mut vx = x.to_vec();
    let vy = y.to_vec();

    if vx.len() != vy.len() {
        panic!(
            "circular_slice assertion failed\nleft:  {:?}\nright: {:?}",
            vx, vy
        );
    }
    if vx.len() < 2 {
        return;
    }

    let mut eq = false;
    for _ in 0..vx.len() {
        if vx == vy {
            eq = true
        }
        let last = vx.pop().unwrap();
        vx.insert(0, last);
    }
    if !eq {
        panic!(
            "circular_slice assertion failed\nleft:  {:?}\nright: {:?}",
            vx, vy
        );
    }
}

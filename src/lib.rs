pub mod bigint;
pub mod binary_tree;
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

use crate::matrix::array::Matrix;

pub mod directed {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Graph<const L: usize, W: Copy> {
        matrix: Matrix<W, L, L>,
    }

    impl<const L: usize, W: Copy + Default> Graph<L, W> {
        pub fn new() -> Self {
            Self {
                matrix: Matrix::new([[W::default(); L]; L]),
            }
        }

        pub fn get_edge(&self, from: usize, to: usize) -> Option<&W> {
            self.matrix.get(from, to)
        }

        pub fn get_edge_mut(&mut self, from: usize, to: usize) -> Option<&mut W> {
            self.matrix.get_mut(from, to)
        }

        /// # Panics
        /// if from >= L || to >= L
        pub fn get_edge_unwrap(&self, from: usize, to: usize) -> &W {
            self.matrix.get(from, to).unwrap()
        }

        /// # Panics
        /// if from >= L || to >= L
        pub fn get_edge_mut_unwrap(&mut self, from: usize, to: usize) -> &mut W {
            self.matrix.get_mut(from, to).unwrap()
        }
    }
}

pub mod undirected {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Graph<const L: usize, W: Copy> {
        matrix: Matrix<W, L, L>,
    }

    impl<const L: usize, W: Copy + Default> Graph<L, W> {
        pub fn new() -> Self {
            Self {
                matrix: Matrix::new([[W::default(); L]; L]),
            }
        }

        pub fn get_edge(&self, x: usize, y: usize) -> Option<&W> {
            self.matrix.get(x, y)
        }

        /// # Returns
        /// old weight
        pub fn insert_edge(&mut self, x: usize, y: usize, weight: W) -> Option<W> {
            let old = self.matrix.get(x, y).map(|i| *i)?;
            unsafe {
                *self.matrix.get_unchecked_mut(x, y) = weight;
                *self.matrix.get_unchecked_mut(y, x) = weight;
            }
            Some(old)
        }

        /// # Panics
        /// if x >= L || y >= L
        pub fn get_edge_unwrap(&self, x: usize, y: usize) -> &W {
            self.matrix.get(x, y).unwrap()
        }

        /// # Panics
        /// if x >= L || y >= L
        pub fn insert_edge_unwrap(&mut self, x: usize, y: usize, weight: W) -> W {
            let old = self.matrix.get(x, y).map(|i| *i).unwrap();
            unsafe {
                *self.matrix.get_unchecked_mut(x, y) = weight;
                *self.matrix.get_unchecked_mut(y, x) = weight;
            }
            old
        }
    }
}

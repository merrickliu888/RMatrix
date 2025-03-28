use crate::Matrix;
use std::cmp::min;

static BLOCK_SIZE: usize = 8;

/// Representing matrix as a one-dimensional vector with blocking
#[derive(Debug)]
pub struct BlockedMatrix {
    data: Vec<f64>,
    shape: (usize, usize),
}

impl BlockedMatrix {
    fn new_from_vec(data: Vec<f64>, rows: usize, cols: usize) -> Self {
        Self {
            data,
            shape: (rows, cols),
        }
    }

    pub fn get_data_vec(&self) -> &Vec<f64> {
        &self.data
    }

    #[inline(always)]
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.num_cols() + col]
    }

    fn multiply_block(&self, bi: usize, bj: usize, bk: usize, other: &Self, res: &mut Vec<f64>) {
        let bi_end = min(bi + BLOCK_SIZE, self.num_rows());
        let bj_end = min(bj + BLOCK_SIZE, other.num_cols());
        let bk_end = min(bk + BLOCK_SIZE, self.num_cols());

        for i in bi..bi_end {
            for j in bj..bj_end {
                for k in bk..bk_end {
                    res[i * other.num_cols() + j] += self.get(i, k) * other.get(k, j);
                }
            }
        }
    }
}

impl Matrix for BlockedMatrix {
    fn new(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        let data = data.into_iter().flatten().collect();
        Self::new_from_vec(data, rows, cols)
    }

    fn shape(&self) -> (usize, usize) {
        self.shape
    }

    fn num_rows(&self) -> usize {
        self.shape.0
    }

    fn num_cols(&self) -> usize {
        self.shape.1
    }

    fn get_data(&self) -> Vec<Vec<f64>> {
        self.data
            .chunks(self.num_cols())
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    fn zeroes(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.0; rows * cols],
            shape: (rows, cols),
        }
    }

    fn identity(size: usize) -> Self {
        let mut data = vec![0.0; size * size];

        for i in 0..size {
            data[i * size + i] = 1.0;
        }

        BlockedMatrix::new_from_vec(data, size, size)
    }

    fn matrix_addition(&self, other: &Self) -> Self {
        let mut data = vec![0.0; self.num_rows() * self.num_cols()];

        for bx in (0..self.num_rows()).step_by(BLOCK_SIZE) {
            for by in (0..self.num_cols()).step_by(BLOCK_SIZE) {
                let num_rows_to_process = min(BLOCK_SIZE, self.num_rows() - bx);
                let num_cols_to_process = min(BLOCK_SIZE, self.num_cols() - by);

                for i in bx..bx + num_rows_to_process {
                    for j in by..by + num_cols_to_process {
                        data[i * self.num_cols() + j] = self.get(i, j) + other.get(i, j);
                    }
                }
            }
        }

        Self::new_from_vec(data, self.num_rows(), self.num_cols())
    }

    fn matrix_subtraction(&self, other: &Self) -> Self {
        let mut data = vec![0.0; self.num_rows() * self.num_cols()];

        for bx in (0..self.num_rows()).step_by(BLOCK_SIZE) {
            for by in (0..self.num_cols()).step_by(BLOCK_SIZE) {
                let num_rows_to_process = min(BLOCK_SIZE, self.num_rows() - bx);
                let num_cols_to_process = min(BLOCK_SIZE, self.num_cols() - by);

                for i in bx..bx + num_rows_to_process {
                    for j in by..by + num_cols_to_process {
                        data[i * self.num_cols() + j] = self.get(i, j) - other.get(i, j);
                    }
                }
            }
        }

        Self::new_from_vec(data, self.num_rows(), self.num_cols())
    }

    fn matrix_multiplication(&self, other: &Self) -> Self {
        let self_rows = self.num_rows();
        let self_cols = self.num_cols();
        let other_cols = other.num_cols();

        let mut res = vec![0.0; self_rows * other_cols];

        for bi in (0..self_rows).step_by(BLOCK_SIZE) {
            for bj in (0..other_cols).step_by(BLOCK_SIZE) {
                for bk in (0..self_cols).step_by(BLOCK_SIZE) {
                    self.multiply_block(bi, bj, bk, other, &mut res);
                }
            }
        }

        Self::new_from_vec(res, self_rows, other_cols)
    }

    fn scalar_multiplication(&self, scalar: f64) -> Self {
        let mut data = vec![0.0; self.num_rows() * self.num_cols()];

        for bx in (0..self.num_rows()).step_by(BLOCK_SIZE) {
            for by in (0..self.num_cols()).step_by(BLOCK_SIZE) {
                let num_rows_to_process = min(BLOCK_SIZE, self.num_rows() - bx);
                let num_cols_to_process = min(BLOCK_SIZE, self.num_cols() - by);

                for i in bx..bx + num_rows_to_process {
                    for j in by..by + num_cols_to_process {
                        data[i * self.num_cols() + j] = self.get(i, j) * scalar;
                    }
                }
            }
        }

        Self::new_from_vec(data, self.num_rows(), self.num_cols())
    }
}

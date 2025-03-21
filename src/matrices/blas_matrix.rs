// Using Apple's Accelerate framework

use crate::Matrix;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum CBLAS_LAYOUT {
    CblasRowMajor = 101,
    CblasColMajor = 102,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum CBLAS_TRANSPOSE {
    CblasNoTrans = 111,
    CblasTrans = 112,
    CblasConjTrans = 113,
}

#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {
    pub unsafe fn cblas_dgemm(
        layout: CBLAS_LAYOUT,
        transa: CBLAS_TRANSPOSE,
        transb: CBLAS_TRANSPOSE,
        m: i32,
        n: i32,
        k: i32,
        alpha: f64,
        a: *const f64,
        lda: i32,
        b: *const f64,
        ldb: i32,
        beta: f64,
        c: *mut f64,
        ldc: i32,
    );
}

#[derive(Debug)]
pub struct BlasMatrix {
    data: Vec<f64>,
    shape: (usize, usize),
}

impl BlasMatrix {
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
}

impl Matrix for BlasMatrix {
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

        BlasMatrix::new_from_vec(data, size, size)
    }

    fn matrix_addition(&self, other: &Self) -> Self {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        Self::new_from_vec(data, self.num_rows(), self.num_cols())
    }

    fn matrix_subtraction(&self, other: &Self) -> Self {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();
        Self::new_from_vec(data, self.num_rows(), self.num_cols())
    }

    fn matrix_multiplication(&self, other: &Self) -> Self {
        let self_rows = self.num_rows();
        let self_cols = self.num_cols();
        let other_cols = other.num_cols();

        let mut res = vec![0.0; self_rows * other_cols];

        unsafe {
            cblas_dgemm(
                CBLAS_LAYOUT::CblasRowMajor,
                CBLAS_TRANSPOSE::CblasNoTrans,
                CBLAS_TRANSPOSE::CblasNoTrans,
                self_rows as i32,
                other_cols as i32,
                self_cols as i32,
                1.0,
                self.data.as_ptr(),
                self_cols as i32,
                other.data.as_ptr(),
                other_cols as i32,
                0.0,
                res.as_mut_ptr(),
                other_cols as i32,
            );
        }

        Self::new_from_vec(res, self_rows, other_cols)
    }

    fn scalar_multiplication(&self, scalar: f64) -> Self {
        let data = self.data.iter().map(|a| a * scalar).collect();
        Self::new_from_vec(data, self.num_rows(), self.num_cols())
    }
}

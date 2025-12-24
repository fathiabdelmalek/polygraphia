use crate::error::PolygraphiaError;
use crate::utils::math;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    size: usize,
    data: Vec<i32>,
}

impl Matrix {
    pub fn new(size: usize, data: Vec<i32>) -> Result<Self, PolygraphiaError> {
        if data.len() != size * size {
            return Err(PolygraphiaError::InvalidInput(format!(
                "Matrix data length {} doesn't match size {size}x{size}",
                data.len()
            )));
        }
        Ok(Matrix { size, data })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&self, row: usize, col: usize) -> i32 {
        self.data[row * self.size + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: i32) {
        self.data[row * self.size + col] = value;
    }

    pub fn determinant(&self) -> i32 {
        match self.size {
            1 => self.data[0],
            2 => self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0),
            3 => {
                let a = self.get(0, 0)
                    * (self.get(1, 1) * self.get(2, 2) - self.get(1, 2) * self.get(2, 1));
                let b = self.get(0, 1)
                    * (self.get(1, 0) * self.get(2, 2) - self.get(1, 2) * self.get(2, 0));
                let c = self.get(0, 2)
                    * (self.get(1, 0) * self.get(2, 1) - self.get(1, 1) * self.get(2, 0));
                a - b + c
            }
            _ => {
                let mut det = 0;
                for col in 0..self.size {
                    let minor = self.minor(0, col);
                    let cofactor = if col % 2 == 0 { 1 } else { -1 };
                    det += cofactor * self.get(0, col) * minor.determinant();
                }
                det
            }
        }
    }

    fn minor(&self, skip_row: usize, skip_col: usize) -> Matrix {
        let new_size = self.size - 1;
        let mut data = Vec::with_capacity(new_size * new_size);
        for row in 0..self.size {
            if row == skip_row {
                continue;
            }
            for col in 0..self.size {
                if col == skip_col {
                    continue;
                }
                data.push(self.get(row, col));
            }
        }
        Matrix::new(new_size, data).unwrap()
    }

    fn adjugate(&self) -> Matrix {
        let mut adj_data = vec![0; self.size * self.size];
        for row in 0..self.size {
            for col in 0..self.size {
                let minor = self.minor(row, col);
                let cofactor = if (row + col) % 2 == 0 { 1 } else { -1 };
                adj_data[col * self.size + row] = cofactor * minor.determinant();
            }
        }
        Matrix::new(self.size, adj_data).unwrap()
    }

    pub fn mod_inverse(&self, modulus: i32) -> Result<Matrix, PolygraphiaError> {
        let det = self.determinant();
        let det_mod = det.rem_euclid(modulus);
        if math::gcd(det_mod as u8, modulus as u8) != 1 {
            return Err(PolygraphiaError::InvalidKey(format!(
                "Matrix determinant {det_mod} is not coprime with {modulus}"
            )));
        }
        let det_inv = math::mod_inverse(det_mod as u8, modulus as u8)? as i32;
        let adj = self.adjugate();
        let mut inv_data = vec![0; self.size * self.size];
        for (i, item) in inv_data.iter_mut().enumerate().take(self.size * self.size) {
            *item = (adj.data[i] * det_inv).rem_euclid(modulus);
        }
        Matrix::new(self.size, inv_data)
    }

    pub fn multiply_vector(&self, vec: &[i32]) -> Vec<i32> {
        let mut result = vec![0; self.size];
        for (row, item) in result.iter_mut().enumerate().take(self.size) {
            let mut sum = 0;
            for (col, _item) in vec.iter().enumerate().take(self.size) {
                sum += self.get(row, col) * vec[col];
            }
            *item = sum;
        }
        result
    }
}

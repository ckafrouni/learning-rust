use super::errors::*;

#[derive(PartialEq, PartialOrd, Debug, Default)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Result<Self, MatrixError> {
        if rows * cols != data.len() {
            Err(MatrixError::DimensionMismatch)
        } else {
            Ok(Self { rows, cols, data })
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn det(&self) -> Result<f64, MatrixError> {
        if self.rows != self.cols {
            return Err(MatrixError::InvalidOperation);
        }
        match self.rows {
            0 => Ok(1.0), // The determinant of a 0x0 matrix is somewhat conventionally 1
            1 => Ok(self.data[0]),
            2 => Ok(self.data[0] * self.data[3] - self.data[1] * self.data[2]),
            _ => {
                let mut det = 0.0;
                for col in 0..self.cols {
                    det += if col % 2 == 0 { 1.0 } else { -1.0 }
                        * self.data[col]
                        * self.minor(0, col)?.det()?;
                }
                Ok(det)
            }
        }
    }

    fn minor(&self, row: usize, col: usize) -> Result<Matrix, MatrixError> {
        let mut minor = Vec::with_capacity((self.rows - 1) * (self.cols - 1));
        for r in 0..self.rows {
            if r == row {
                continue;
            }
            for c in 0..self.cols {
                if c == col {
                    continue;
                }
                minor.push(self.data[r * self.cols + c]);
            }
        }
        Matrix::new(self.rows - 1, self.cols - 1, minor).map_err(|_| MatrixError::InvalidOperation)
    }

    pub fn transpose(&mut self) {
        let (rows, cols) = self.dimensions();
        let mut data = vec![0.0; rows * cols];

        for i in 0..rows {
            for j in 0..cols {
                data[j * rows + i] = self.data[i * cols + j];
            }
        }

        self.rows = cols;
        self.cols = rows;
        self.data = data;
    }

    pub fn inverse(&self) -> Result<Matrix, MatrixError> {
        if self.rows != self.cols {
            return Err(MatrixError::InvalidOperation);
        }
        let det = self.det()?;
        if det == 0.0 {
            return Err(MatrixError::SingularMatrix);
        }

        todo!("Inverse computation not implemented")
    }

    pub fn add(&mut self, other: &Matrix) -> Result<(), MatrixError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(MatrixError::DimensionMismatch);
        }

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += b;
        }

        Ok(())
    }

    pub fn sub(&mut self, other: &Matrix) -> Result<(), MatrixError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(MatrixError::DimensionMismatch);
        }

        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a -= b;
        }

        Ok(())
    }

    pub fn scalar_prod(&mut self, scalar: f64) {
        for value in self.data.iter_mut() {
            *value *= scalar;
        }
    }

    pub fn dot_prod(&mut self, other: &Matrix) -> Result<(), MatrixError> {
        if self.cols != other.rows {
            return Err(MatrixError::InvalidOperation);
        }

        let (rows, cols) = (self.rows, other.cols);
        let mut data = vec![0.0; rows * cols];

        for i in 0..rows {
            for j in 0..cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i * self.cols + k] * other.data[k * other.cols + j]
                }
                data[i * cols + j] = sum;
            }
        }

        self.rows = rows;
        self.cols = cols;
        self.data = data;

        Ok(())
    }
}

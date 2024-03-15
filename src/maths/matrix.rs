use core::ops::{Add, Mul, Sub};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Matrix {
	matrix: Vec<Vec<i32>>,
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for row in self.matrix.iter() {
            for entry in row {
                s += &format!("{:20?}", entry);
            }
            s += "\n";
        }
        write!(f, "{}", s)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        let num_rows = self.matrix.len();
        let num_columns = self.matrix.first().unwrap().len();

		if num_rows != other.matrix.len() || num_columns != other.matrix.first().unwrap().len() {
			return false;
		}
        for i in 0..num_rows {
            for j in 0..num_columns {
                if self.matrix[i][j] != other.matrix[i][j] {
                    return false;
                }
            }
        }

        true
    }
}

impl Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, other: Matrix) -> Matrix {

		let num_rows = self.matrix.len();
        let num_columns = self.matrix.first().unwrap().len();

        assert_eq!(
            num_rows,
            other.matrix.len(),
            "Matrices have different number of rows"
        );
        assert_eq!(
            num_columns,
            other.matrix.first().unwrap().len(),
            "Matrices have different number of columns"
        );

        let mut result = self.clone();

        for i in 0..num_rows {
            for j in 0..num_columns {
                result.matrix[i][j] = self.matrix[i][j] + other.matrix[i][j];
            }
        }

        result
    }
}

impl Sub<Matrix> for Matrix {
	type Output = Matrix;

    fn sub(self, other: Matrix) -> Matrix {
		let num_rows = self.matrix.len();
		let num_columns = self.matrix.first().unwrap().len();

        assert_eq!(
            num_rows,
            other.matrix.len(),
            "Matrices have different number of rows"
        );
        assert_eq!(
            num_columns,
            other.matrix.first().unwrap().len(),
            "Matrices have different number of columns"
        );

        let mut result = self.clone();

        for i in 0..num_rows {
            for j in 0..num_columns {
                result.matrix[i][j] = self.matrix[i][j] - other.matrix[i][j];
            }
        }

        result
    }
}

impl Mul<Matrix> for Matrix {
	type Output = Matrix;

	fn mul(self, other: Matrix) -> Matrix {
		let num_rows = self.matrix.len();
		let num_columns = self.matrix.first().unwrap().len();

		assert_eq!(
			num_columns,
			other.matrix.len(),
			"Number of columns in first matrix must be equal to number of rows in second matrix"
		);

		let mut result = Matrix::new(vec![
			vec![0; other.matrix.first().unwrap().len()];
			self.matrix.len()
		]);
		
		for i in 0..num_rows {
            for j in 0..other.matrix.first().unwrap().len() {
				result.matrix[i][j] = 0;
				for k in 0..num_columns {
					result.matrix[i][j] += self.matrix[i][k] * other.matrix[k][j];
				}
			}
		}

		result
	}
}

impl Matrix {
	pub fn new(matrix: Vec<Vec<i32>>) -> Matrix {
		Matrix { matrix }
	}

	pub fn is_square(self) -> bool {
        let num_rows = self.matrix.len();
        let num_columns = self.matrix.first().unwrap().len();

        if num_rows == 0 {
            return false;
        }

        num_rows == num_columns
    }

	pub fn mul_vec(self, vec: Vec<i32>) -> Vec<i32> {
        assert_eq!(
            vec.len(),
            self.matrix.len(),
            "Vector and matrix can't be multiplied by the other"
        );

        let mut result = vec![0; self.matrix.first().unwrap().len()];

        for i in 0..self.matrix.first().unwrap().len() {
            for j in 0..self.matrix.len() {
                result[i] += vec[j] * self.matrix[j][i];
            }
        }

        result
    }

	pub fn set_element(&mut self, row: usize, column: usize, new_value: i32) {
        let num_rows = self.matrix.len();
        let num_columns = self.matrix.first().unwrap().len();

        assert!(
            row < num_rows && column < num_columns,
            "Index out of bounds"
        );

        for i in 0..num_rows {
            for j in 0..num_columns {
                if i == row && j == column {
                    self.matrix[i][j] = new_value;
                }
            }
        }
    }

	pub fn get_element(&self, row: usize, column: usize) -> i32 {
        let num_rows = self.matrix.len();
        let num_columns = self.matrix.first().unwrap().len();

        assert!(
            row < num_rows && column < num_columns,
            "Index out of bounds"
        );

        self.matrix[row][column]
    }
}
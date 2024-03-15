mod maths;
mod fhe;
mod utils;


#[cfg(test)]
mod test {
    use crate::maths::matrix::Matrix;
    #[test]
    fn test_matrix() {
        let a = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::new(vec![vec![5, 6], vec![7, 8]]);
        let c = a.clone() + b.clone();
        assert_eq!(c, Matrix::new(vec![vec![6, 8], vec![10, 12]]));
        let d = a.clone() - b.clone();
        assert_eq!(d, Matrix::new(vec![vec![-4, -4], vec![-4, -4]]));
        let e = a.clone() * b.clone();
        assert_eq!(e, Matrix::new(vec![vec![19, 22], vec![43, 50]]));
    }
}
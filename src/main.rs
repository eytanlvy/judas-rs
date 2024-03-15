mod maths;
fn main() {
	println!("Hello, world!");
	let a = crate::maths::matrix::Matrix::new(vec![vec![1, 2], vec![3, 4]]);
        let b = crate::maths::matrix::Matrix::new(vec![vec![5, 6], vec![7, 8]]);
        let c = a + b;
        println!("{}", c);
        assert_eq!(c, crate::maths::matrix::Matrix::new(vec![vec![6, 8], vec![10, 12]]));
}
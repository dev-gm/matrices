#![feature(box_into_inner)]
use crate::group::*;
use std::ops;

pub struct Matrix<T: Group> {
    size: (usize, usize),
    matrix: Vec<Vec<T>>,
}

impl<T: Group> Matrix<T> {
    pub fn from_vec(matrix: Vec<Vec<T>>) -> Self {
        Self {
            size: (matrix.len(), matrix[0].len()),
            matrix,
        }
    }

    pub fn new_add_identity(size: (usize, usize)) -> Self {
        let mut matrix = Vec::new();
        for x in 0..size.0 {
            matrix.push(Vec::new());
            for y in 0..size.1 {
                matrix[x].push(Box::into_inner(T::empty().operation(OperationType::Add)
                    .expect("Can't get addition identity for group that doesn't have addition operations").1));
            }
        }
        Self::from_vec(matrix)
    }

    pub fn new_mul_identity(n: usize) -> Self {
        let mut matrix = Vec::new();
        for x in 0..n {
            matrix.push(Vec::new());
            for y in 0..n {
                if x == y {
                    matrix[x].push(Box::into_inner(T::empty().operation(OperationType::Mul)
                        .expect("Can't get multiplicative identity for group that doesn't have multiplication operations").1));
                } else {
                    matrix[x].push(T::empty().zero());
                }
            }
        }
        Self::from_vec(matrix)
    }

    pub fn square(&self) -> bool {
        self.size.0 == self.size.1
    }

    pub fn det(&self) -> T {
        self.det_checked().unwrap()
    }

    pub fn det_checked(&self) -> Result<T, String> {
        if !self.square() {
            Err(String::from("Getting the determinant for a non n*n matrix is undefined!"))
        } else {}
    }

    pub fn pow(&self, exp: isize) -> Self {
        self.pow_checked(exp).unwrap()
    }

    pub fn pow_checked(&self, exp: isize) -> Result<Self, String> {
        panic!("Z*Z where Z doesn't have dimensions of n*n is undefined!");
    }
}

impl<T: Group + ops::Add<Output = T>> Group for Matrix<T> {
    fn zero(&self) -> Box<Self> {
        let mut matrix = Vec::new();
        for x in 0..
    }

    fn operation(&self, key: OperationType) -> Option<(Operation<Self>, Box<Self>)> {

    }
}

impl<T: Group> ops::Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}

impl<T: Group> ops::IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}

impl<T: Group + ops::Add<Output = T>> ops::Add for Matrix<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.size != rhs.size {
            panic!("Matrices with different sizes cannot be added together!");
        } else {
            for x in 0..self.size.0 {
                for y in 0..self.size.1 {
                    self[x][y] = self[x][y] + rhs[x][y];
                }
            }
            self
        }
    }
}

impl<T: Group + ops::Add<Output = T>> ops::AddAssign for Matrix<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T: Group + ops::Sub<Output = T>> ops::Sub for Matrix<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.size != rhs.size {
            panic!("Subtraction between matrices of different sizes is undefined!");
        } else {
            for x in 0..self.size.0 {
                for y in 0..self.size.1 {
                    self[x][y] = self[x][y] - rhs[x][y];
                }
            }
            self
        }
    }
}

impl<T: Group + ops::Sub<Output = T>> ops::SubAssign for Matrix<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T: Group + ops::Mul<Output = T> + ops::Add<Output = T>> ops::Mul for Matrix<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.size.1 != rhs.size.0 {
            panic!("Multiplication for a matrix with {:?} dimensions and one with {:?} dimensions is undefined!", self.size, rhs.size);
        } else {
            let mut matrix = Vec::new();
            for row in 0..self.size.0 {
                matrix.push(Vec::new());
                for col in 0..rhs.size.1 {
                    let mut sum = self[row][0] * self[col][0];
                    for i in 1..self.size.1 {
                        sum = sum + (self[row][i] * self[col][i]);
                    }
                    matrix[row].push(sum);
                }
            }
            Matrix::from_vec(matrix)
        }
    }
}

impl<T: Group + ops::Mul<Output = T> + ops::Add<Output = T>> ops::MulAssign for Matrix<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl <T: Group + ops::Div<Output = T> + ops::Mul<Output = T> + ops::Add<Output = T>> ops::Div for Matrix<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if self.square() != rhs.square() {
            panic!("Dividing a non n*n matrix by a non n*n matrix is undefined!");
        } else {
            self * rhs.pow(-1)
        }
    }
}

impl<T: Group + ops::Div<Output = T> + ops::Mul<Output = T> + ops::Add<Output = T>> ops::DivAssign for Matrix<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

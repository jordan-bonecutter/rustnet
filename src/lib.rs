#![feature(generic_const_exprs, specialization)]
use std::array::{self, from_fn};

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

pub trait Zero {
    fn zero() -> Self;
}

impl<T> Zero for T where T: From<u8> {
    fn zero() -> Self {
        0.into()
    }
}

#[derive(Debug, Clone)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> 
where [(); ROWS * COLS]: Sized {
    data: [T; ROWS * COLS],
}

impl<T: Zero, const ROWS: usize, const COLS: usize> Zero for Matrix<T, ROWS, COLS>
where [(); ROWS * COLS]: Sized {
    default fn zero() -> Self {
        Self{ data: array::from_fn(|_| T::zero()) }
    }
}

impl<T: Zero + Copy, const ROWS: usize, const COLS: usize> Zero for Matrix<T, ROWS, COLS>
where [(); ROWS * COLS]: Sized {
    fn zero() -> Self {
        Self{ data: [T::zero(); ROWS * COLS] }
    }
}

impl<T: Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> 
where [(); ROWS * COLS]: Sized {
    fn apply<U, F: Fn(T) -> U>(mut self, f: F) -> Matrix<U, ROWS, COLS> {
        Matrix::<U, ROWS, COLS>{ data: array::from_fn(|idx| f(self.data[idx])) }
    }

    fn apply_into<F: Fn(T) -> T>(mut self, f: F) -> Matrix<T, ROWS, COLS> {
        self.data.iter_mut().for_each(|el| {
            *el = f(*el);
        });
        self
    }
}

impl<T: std::ops::Mul<Output = T> + Copy, const ROWS: usize, const COLS: usize> std::ops::Mul<T> for Matrix<T, ROWS, COLS>
where [(); ROWS * COLS]: Sized {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self {
        self.apply_into(|el| el * rhs)
    }
}

impl<T, const ROWS: usize, const COLS: usize> std::ops::Index<(usize, usize)> for Matrix<T, ROWS, COLS> 
where [(); ROWS * COLS]: Sized {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 + (index.1 * COLS)]
    }
}

impl<T, const ROWS: usize, const COLS: usize> std::ops::IndexMut<(usize, usize)> for Matrix<T, ROWS, COLS> 
where [(); ROWS * COLS]: Sized {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 + (index.1 * COLS)]
    }
}

impl<T: std::ops::Mul<Output = T> + Copy + std::ops::Add<Output = T> + Zero, const M: usize, const N: usize, const L: usize> std::ops::Mul<&Matrix<T, N, L>> for &Matrix<T, M, N>
where [(); M * N]: Sized,
      [(); N * L]: Sized,
      [(); M * L]: Sized {
    type Output = Matrix<T, M, L>;

    fn mul(self, rhs: &Matrix<T, N, L>) -> Matrix<T, M, L> {
        Matrix::<T, M, L>{ data: array::from_fn(|idx| {
            let row = idx / L;
            let col = idx % L;
            let mut accumulator = T::zero();
            for i in 0..N {
                accumulator = accumulator + (self[(i, row)] * rhs[(col, i)]);
            }
            accumulator
        }) }
    }
}

impl<T: Zero + Copy + std::ops::Add<Output = T> + std::ops::Mul<Output = T>, const ROWS: usize> Matrix<T, ROWS, 1> 
where [(); ROWS * 1]: Sized {
    fn norm_squared(self) -> T {
        self.data.into_iter().fold(T::zero(), |accum, cur| accum + (cur*cur))
    }
}

impl<T: Sqrt + Zero + Copy + std::ops::Add<Output = T> + std::ops::Mul<Output = T>, const ROWS: usize> Matrix<T, ROWS, 1>
where [(); ROWS * 1]: Sized {
    fn norm(self) -> T {
        self.norm_squared().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut m1 = Matrix::<f64, 2, 3>::zero();
        m1[(0, 0)] = 1.;
        m1[(1, 0)] = 2.;
        m1[(2, 0)] = 3.;
        m1[(0, 1)] = 4.;
        m1[(1, 1)] = 5.;
        m1[(2, 1)] = 6.;
        let mut m2 = Matrix::<f64, 3, 2>::zero();
        m2[(0, 0)] = 10.;
        m2[(1, 0)] = 11.;
        m2[(0, 1)] = 20.;
        m2[(1, 1)] = 21.;
        m2[(0, 2)] = 30.;
        m2[(1, 2)] = 31.;
        let res = (&m1) * (&m2);
        assert_eq!(res[(0, 0)], 140.);
        assert_eq!(res[(1, 0)], 146.);
        assert_eq!(res[(0, 1)], 320.);
        assert_eq!(res[(1, 1)], 335.);
    }
}

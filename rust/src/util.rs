use std::{
    iter::Extend,
    ops::{Index, IndexMut},
};

/// A very simple implementation of a matrix abstraction. It's
/// simply a Vec an associated 'width' that is used for indexing.
#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Matrix<T> {
    pub fn wrap(inner: Vec<T>, width: usize) -> Self {
        Matrix { data: inner, width }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }
    pub fn row(&self, i: usize) -> &[T] {
        return &self.data[(i * self.width)..(i * self.width + self.width)];
    }

    /// Returns a Matrix with memory allocated for size*size elements.
    pub fn with_capacity(size: usize) -> Matrix<T> {
        Matrix {
            data: Vec::with_capacity(size * size),
            width: size,
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    /// Provides a view into the inner vector, e.g. for cases where all
    /// elements need to be iterated in order
    pub fn inner(&self) -> &[T] {
        &self.data
    }
}

impl<T: Clone> Matrix<T> {
    /// Creates a size*size Matrix with the given initial element
    /// occupying all positions.
    pub fn with_element(size: usize, element: T) -> Matrix<T> {
        Matrix {
            data: vec![element; size * size],
            width: size,
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let i = y * self.width + x;
        return &self.data[i];
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let i = y * self.width + x;
        return &mut self.data[i];
    }
}

impl<T> Extend<T> for Matrix<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter.into_iter());
    }
}

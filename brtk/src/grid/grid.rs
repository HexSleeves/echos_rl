use core::slice;
use std::ops::{Index, IndexMut};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Reflect, Debug, Clone)]
pub struct Grid<T> {
    size: (usize, usize),
    data: Vec<T>,
}

// Constructors
impl<T> Grid<T> {
    /// Create a new `Grid` from a `(width, height)` and `Vec<T>`.
    /// Panics if the data length doesn't match width * height.
    pub fn new(size: (usize, usize), data: Vec<T>) -> Self {
        assert_eq!(data.len(), size.0 * size.1, "Data length must match grid dimensions");
        Self { size, data }
    }

    /// Create a new `Grid` from a `(width, height)` without checking data length.
    /// Use with caution, as improper sizes can lead to unexpected behavior.
    pub const fn new_unchecked(size: (usize, usize), data: Vec<T>) -> Self { Self { size, data } }

    /// Create a new `Grid` from a `(width, height)` cloning the value
    pub fn new_clone(size: (usize, usize), value: T) -> Self
    where
        T: Clone,
    {
        let capacity = size.0 * size.1;
        Self::new(size, vec![value; capacity])
    }

    /// Create a new `Grid` filled with the specified value
    /// Alias for new_clone with a more intuitive name
    pub fn new_fill(size: (usize, usize), value: T) -> Self
    where
        T: Clone,
    {
        Self::new_clone(size, value)
    }

    /// Create a new `Grid` from a `(width, height)` obtaining a value from a `Fn(index, position)
    /// -> T`. Uses row-major order (iterates over rows first, then columns).
    pub fn new_fn(size: (usize, usize), mut f: impl FnMut(usize, (usize, usize)) -> T) -> Self {
        let capacity = size.0 * size.1;
        let mut data = Vec::with_capacity(capacity);
        let mut idx = 0;

        for y in 0..size.1 {
            for x in 0..size.0 {
                data.push(f(idx, (x, y)));
                idx += 1;
            }
        }

        Self { size, data }
    }
}

impl<T> Grid<T> {
    /// Obtain the size of this `Grid`
    #[inline]
    pub const fn size(&self) -> (usize, usize) { self.size }

    /// Obtain the width of this `Grid`
    #[inline]
    pub const fn width(&self) -> usize { self.size.0 }

    /// Obtain the height of this `Grid`
    #[inline]
    pub const fn height(&self) -> usize { self.size.1 }

    /// Determine if a position is inside of this `Grid`
    ///
    /// NOTE: A position converted to an index may not be `in_bounds` yet still pass
    /// `is_valid`. Given a `Grid` with size (3, 3), a position (0, 4) is not inside this
    /// `Grid` but provides a valid index.
    #[inline]
    pub const fn in_bounds(&self, position: (i32, i32)) -> bool {
        position.0 >= 0
            && position.0 < self.width() as i32
            && position.1 >= 0
            && position.1 < self.height() as i32
    }

    /// Determine if an index is valid in this `Grid`
    ///
    /// NOTE: A position converted to an index may not be `in_bounds` yet still pass
    /// `is_valid`. Given a `Grid` with size (3, 3), a position (0, 4) is not inside this
    /// `Grid` but provides a valid index.
    #[inline]
    pub fn is_valid(&self, index: usize) -> bool { index < self.data.len() }
}

// Index/Position Conversion
impl<T> Grid<T> {
    /// Converts a position into an index
    pub const fn position_to_index(&self, position: (i32, i32)) -> Option<usize> {
        if self.in_bounds(position) {
            Some(self.position_to_index_unchecked(position))
        } else {
            None
        }
    }

    /// Converts a position into an index
    #[inline]
    pub const fn position_to_index_unchecked(&self, position: (i32, i32)) -> usize {
        (position.1 * self.width() as i32 + position.0) as usize
    }

    /// Converts an index into a position
    pub const fn index_to_position(&self, index: usize) -> Option<(i32, i32)> {
        let position = self.index_to_position_unchecked(index);
        if self.in_bounds(position) {
            Some(position)
        } else {
            None
        }
    }

    /// Converts an index into a position
    #[inline]
    pub const fn index_to_position_unchecked(&self, index: usize) -> (i32, i32) {
        ((index % self.width() as usize) as i32, (index / self.width() as usize) as i32)
    }
}

// Accessors
impl<T> Grid<T> {
    /// Borrow the full `Grid`
    #[inline]
    pub const fn data(&self) -> &Vec<T> { &self.data }

    /// Mutably borrow the full `Grid`
    #[inline]
    pub fn data_mut(&mut self) -> &mut Vec<T> { &mut self.data }

    /// Borrow a value at an index
    #[inline]
    pub fn get_index(&self, index: usize) -> Option<&T> { self.data.get(index) }

    /// Mutably borrow a value at an index
    #[inline]
    pub fn get_mut_index(&mut self, index: usize) -> Option<&mut T> { self.data.get_mut(index) }

    /// Borrow a value at a position
    pub fn get(&self, position: (i32, i32)) -> Option<&T> {
        if self.in_bounds(position) {
            self.get_index(self.position_to_index_unchecked(position))
        } else {
            None
        }
    }

    /// Mutably borrow a value at a position
    pub fn get_mut(&mut self, position: (i32, i32)) -> Option<&mut T> {
        if self.in_bounds(position) {
            self.get_mut_index(self.position_to_index_unchecked(position))
        } else {
            None
        }
    }
}

// Iterators
impl<T> Grid<T> {
    pub fn iter(&self) -> slice::Iter<T> { self.data.iter() }

    pub fn iter_mut(&mut self) -> slice::IterMut<T> { self.data.iter_mut() }
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output { &self.data[index] }
}

impl<T> IndexMut<usize> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.data[index] }
}

impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (i32, i32)) -> &Self::Output {
        self.get(index).expect("Invalid index position")
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        self.get_mut(index).expect("Invalid index position")
    }
}

impl<T> Index<(u32, u32)> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (u32, u32)) -> &Self::Output {
        let index = (index.0 as i32, index.1 as i32);
        self.get(index).expect("Invalid index position")
    }
}

impl<T> IndexMut<(u32, u32)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        let index = (index.0 as i32, index.1 as i32);
        self.get_mut(index).expect("Invalid index position")
    }
}

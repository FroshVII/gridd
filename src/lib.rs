//! A generic, dirt-simple, two-dimensional grid.
//!
//! # Coordinates
//! Grid entries are accessed with the `Coord` type, which is of the form
/// (column, row), where column >= 0 and row >= 0.
///
/// # Vectors
/// Instead of offering pre-defined relational getters/iterators such as
/// `neighbors` or `diagonals`, Gridd provides `Vector`s for users to build
/// their own spatial relationship methods. `Vector`s are just `struct`s with
/// positive or negative `col_offset` and `row_offset` fields, enabling the
/// codification of arbitrary relations.
///
/// Scalar multiplication is reflexively defined on `Vectors` for `i32`
/// values and addition/subtraction is defined between vectors, each in the
/// traditional manner.
///
/// # Static Grids
/// Currently, Gridd only supports grids of a fixed size.

use std::ops::{Add, Mul, Sub};

//////////////////////////////////////////////////////////////////////////////
// Type Aliases
//////////////////////////////////////////////////////////////////////////////

/// Coordinates of the form (column, row), where column >= 0 and row >= 0.
pub type Coord = (usize, usize);

//////////////////////////////////////////////////////////////////////////////
// 2D Vectors
//////////////////////////////////////////////////////////////////////////////

/// A two-dimensional vector used to relate grid elements spatially.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vector {
    pub col_offset: i32,
    pub row_offset: i32,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            col_offset: self.col_offset + other.col_offset,
            row_offset: self.row_offset + other.row_offset,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            col_offset: self.col_offset - other.col_offset,
            row_offset: self.row_offset - other.row_offset,
        }
    }
}

impl Mul<i32> for Vector {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self::Output {
        Self {
            col_offset: scalar * self.col_offset,
            row_offset: scalar * self.row_offset,
        }
    }
}

impl Mul<Vector> for i32 {
    type Output = Vector;

    fn mul(self, vec: Vector) -> Self::Output {
        vec * self
    }
}

impl From<(i32, i32)> for Vector {
    fn from((c, r): (i32, i32)) -> Self {
        Self {
            col_offset: c,
            row_offset: r,
        }
    }
}

impl Vector {
    //////////////////////////////////
    // Constants
    //////////////////////////////////

    /// Northern unit vector: (col: +0, row: -1).
    pub const NORTH: Vector = Vector {
        col_offset: 0,
        row_offset: -1,
    };

    /// Eastern unit vector: (col: +1, row: +0).
    pub const EAST: Vector = Vector {
        col_offset: 1,
        row_offset: 0,
    };

    /// Southern unit vector: (col: +0, row: +1).
    pub const SOUTH: Vector = Vector {
        col_offset: 0,
        row_offset: 1,
    };

    /// Western unit vector: (col: -1, row: +0).
    pub const WEST: Vector = Vector {
        col_offset: -1,
        row_offset: 0,
    };

    /// Get the coordinate pointed to by a `Vector` from a given `Coord`.
    ///
    /// Returns `None` when either `Coord` component would be negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use gridd::{Coord, Vector};
    ///
    /// let coord: Coord = (3, 5);
    ///
    /// let v1 = Vector {
    ///     col_offset: -3,
    ///     row_offset: 2,
    /// };
    /// let v2 = Vector {
    ///     col_offset: -4,
    ///     row_offset: 5,
    /// };
    ///
    /// assert_eq!(Some((0, 7)), v1.rcoord(coord));
    /// assert_eq!(None, v2.rcoord(coord));
    /// ```
    pub fn rcoord(&self, (col, row): Coord) -> Option<Coord> {
        let c = self.col_offset + (col as i32);
        let r = self.row_offset + (row as i32);

        if c >= 0 && r >= 0 {
            Some((c as usize, r as usize))
        } else {
            None
        }
    }

    //////////////////////////////////
    // Instantiation
    //////////////////////////////////

    /// Create a new `Vector` from the sum of cardinal vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use gridd::Vector;
    ///
    /// assert_eq!(
    ///     Vector::cardinal_sum(1, 0, 1, 0),
    ///     Vector::NORTH + Vector::SOUTH
    /// );
    /// assert_eq!(
    ///     Vector::cardinal_sum(0, 2, 0, 3),
    ///     2 * Vector::EAST + 3 * Vector::WEST
    /// );
    /// ```
    pub fn cardinal_sum(n: i32, e: i32, s: i32, w: i32) -> Self {
        Vector::NORTH * n
        + Vector::EAST * e
        + Vector::SOUTH * s
        + Vector::WEST * w
    }
}

//////////////////////////////////////////////////////////////////////////////
// Fixed-Size 2D Grids
//////////////////////////////////////////////////////////////////////////////

/// Two-dimensional, zero-indexed grid. Cannot change dimensions.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StaticGrid<T> {
    col_count: usize,
    row_count: usize,
    data: Vec<T>,
}

impl<T> StaticGrid<T> {
    //////////////////////////////////
    // Utilities
    //////////////////////////////////

    /// Get the flat-vector index from the column and row indices.
    fn flat_index(&self, (col, row): Coord) -> usize {
        col + self.col_count * row
    }

    //////////////////////////////////
    // Instantiation
    //////////////////////////////////

    /// Create a new `StaticGrid` populated with a default value.
    pub fn new(col_count: usize, row_count: usize, default: T) -> Self
    where
        T: Copy,
    {
        let capactiy = row_count * col_count;

        Self {
            col_count: col_count,
            row_count: row_count,
            data: vec![default; capactiy],
        }
    }

    /// Create a new `StaticGrid` in a square shape, populated with a default
    /// value.
    pub fn square(side_len: usize, default: T) -> Self
    where
        T: Copy,
    {
        Self::new(side_len, side_len, default)
    }

    //////////////////////////////////
    // Get & Set
    //////////////////////////////////

    /// Get an immutable reference to a cell's value.
    pub fn get(&self, coord: Coord) -> Option<&T> {
        if self.contains(coord) {
            let index = self.flat_index(coord);

            Some(&self.data[index])
        } else {
            None
        }
    }

    /// Get a mutable reference to a cell's value.
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        if self.contains(coord) {
            let index = self.flat_index(coord);

            Some(&mut self.data[index])
        } else {
            None
        }
    }

    /// Get an immutable reference to a cell's value offset from an anchor.
    pub fn rget(&self, anchor: Coord, vec: Vector) -> Option<&T> {
        match vec.rcoord(anchor) {
            Some(coord) => self.get(coord),
            _ => None,
        }
    }

    /// Get a mutable reference to a cell's value offset from an anchor.
    pub fn rget_mut(&mut self, anchor: Coord, vec: Vector) -> Option<&mut T> {
        match vec.rcoord(anchor) {
            Some(coord) => self.get_mut(coord),
            _ => None,
        }
    }

    /// Set a cell's value.
    pub fn set(&mut self, coord: Coord, new_val: T) {
        match self.get_mut(coord) {
            Some(val) => {
                *val = new_val;
            }
            None => (),
        }
    }

    //////////////////////////////////
    // Boolean Operations
    //////////////////////////////////

    /// Determine if a coordinate is within the grid
    pub fn contains(&self, (col, row): Coord) -> bool {
        col < self.col_count && row < self.row_count
    }

    //////////////////////////////////
    // Advanced Operations
    //////////////////////////////////

    /// Return a transposition of the grid
    pub fn transposition(&self) -> Self {
        unimplemented!();
    }
}

//////////////////////////////////////////////////////////////////////////////
// Unit Tests
//////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mut() {
        let mut grid = StaticGrid::new(1, 1, 'a');
        let value = grid.get_mut((0, 0)).unwrap();

        assert_eq!(&'a', value);
        *value = 'b';
        assert_eq!(&'b', value);
    }

    #[test]
    fn test_get_set() {
        let mut grid = StaticGrid::new(5, 5, 'a');

        assert_eq!(Some(&'a'), grid.get((2, 3)));
        assert_eq!(Some(&'a'), grid.get((3, 3)));
        assert_eq!(Some(&'a'), grid.get((3, 4)));

        grid.set((2, 3), 'b');
        grid.set((3, 3), 'c');
        grid.set((3, 4), 'd');

        assert_eq!(Some(&'b'), grid.get((2, 3)));
        assert_eq!(Some(&'c'), grid.get((3, 3)));
        assert_eq!(Some(&'d'), grid.get((3, 4)));
    }

    #[test]
    fn test_rget() {
        let mut grid = StaticGrid::new(5, 5, 'a');

        assert_eq!(Some(&'a'), grid.get((2, 3)));

        grid.set((2, 3), 'b');

        assert_eq!(Some(&'b'), grid.rget((2, 4), Vector::NORTH));
        assert_eq!(Some(&mut 'b'), grid.rget_mut((2, 4), Vector::NORTH));
    }
}

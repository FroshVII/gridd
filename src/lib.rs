//! A generic, dirt-simple, two-dimensional grid.
//!
//! # Grids & Coordinates
//! The `Grid` structure represents a two-dimensional grid containing
//! arbitrary data within its cells. Data is accessed via the `Coord` type,
//! which is equivalent to the `(usize, usize)` type. Cell indices are of the
//! form (column, row) where column and row are non-negative values.
//!
//! # Offset Vectors
//! Gridd offers `Offset`s for working with positional relationships. This
//! allows the API to stay small while still offering a more convenient
//! abstraction for relational methods and iterators. Here's how you might
//! implement a `knight_moves` method using Gridd:
//!
//! ```
//! use gridd::{Coord, Grid, Offset};
//!
//! struct ChessGame<T> {
//!     board: Grid<T>
//! }
//!
//! impl<T> ChessGame<T> {
//!     pub(crate) fn rotate(os: &mut Offset) {
//!         let new_c = os.row_offset;
//!
//!         os.row_offset = os.col_offset;
//!         os.col_offset = new_c * (-1);
//!     }
//!
//!     pub fn knight_moves(&self, rook_pos: Coord) -> Vec<&T> {
//!         let mut coords = Vec::new();
//!
//!         let mut move1 = Offset::from((2, 1));
//!         let mut move2 = Offset::from((1, 2));
//!
//!         for _ in 0..4 {
//!             if let Some(square_data) = self.board.rget(rook_pos, move1) {
//!                 coords.push(square_data);
//!             }
//!             if let Some(square_data) = self.board.rget(rook_pos, move2) {
//!                 coords.push(square_data);
//!             }
//!             Self::rotate(&mut move1);
//!             Self::rotate(&mut move2);
//!         }
//!
//!         coords
//!     }
//! }
//! ```
//!
//! It's also worth noting that Gridd's approach doesn't impose any order on
//! the final `knight_moves` method, offering greater flexibility.
//!
//! Implementations are provided for scalar multiplication, vector addition,
//! and vector subtraction.

use std::ops::{Add, Mul, Sub};

//////////////////////////////////////////////////////////////////////////////
// Type Aliases
//////////////////////////////////////////////////////////////////////////////

/// Coordinates of the form (column, row), where column >= 0 and row >= 0.
pub type Coord = (usize, usize);

//////////////////////////////////////////////////////////////////////////////
// Offset Vectors
//////////////////////////////////////////////////////////////////////////////

/// A two-dimensional offset vector used to relate grid elements spatially.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Offset {
    pub col_offset: i32,
    pub row_offset: i32,
}

impl Add for Offset {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            col_offset: self.col_offset + other.col_offset,
            row_offset: self.row_offset + other.row_offset,
        }
    }
}

impl Sub for Offset {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            col_offset: self.col_offset - other.col_offset,
            row_offset: self.row_offset - other.row_offset,
        }
    }
}

impl Mul<i32> for Offset {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self::Output {
        Self {
            col_offset: scalar * self.col_offset,
            row_offset: scalar * self.row_offset,
        }
    }
}

impl Mul<Offset> for i32 {
    type Output = Offset;

    fn mul(self, vec: Offset) -> Self::Output {
        vec * self
    }
}

impl From<(i32, i32)> for Offset {
    fn from((c, r): (i32, i32)) -> Self {
        Self {
            col_offset: c,
            row_offset: r,
        }
    }
}

impl Offset {
    //////////////////////////////////
    // Constants
    //////////////////////////////////

    /// Northern unit vector: (col: +0, row: -1).
    pub const NORTH: Offset = Offset {
        col_offset: 0,
        row_offset: -1,
    };

    /// Eastern unit vector: (col: +1, row: +0).
    pub const EAST: Offset = Offset {
        col_offset: 1,
        row_offset: 0,
    };

    /// Southern unit vector: (col: +0, row: +1).
    pub const SOUTH: Offset = Offset {
        col_offset: 0,
        row_offset: 1,
    };

    /// Western unit vector: (col: -1, row: +0).
    pub const WEST: Offset = Offset {
        col_offset: -1,
        row_offset: 0,
    };

    //////////////////////////////////
    // Operations
    //////////////////////////////////

    /// Create a new `Offset` from the sum of cardinal vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use gridd::Offset;
    ///
    /// assert_eq!(
    ///     Offset::cardinal_sum(1, 0, -1, 0),
    ///     Offset::NORTH - Offset::SOUTH
    /// );
    ///
    /// assert_eq!(
    ///     Offset::cardinal_sum(0, 2, 0, 3),
    ///     2 * Offset::EAST + 3 * Offset::WEST
    /// );
    /// ```
    pub fn cardinal_sum(n: i32, e: i32, s: i32, w: i32) -> Self {
        Offset::NORTH * n
        + Offset::EAST * e
        + Offset::SOUTH * s
        + Offset::WEST * w
    }

    /// Get the coordinate pointed to by an `Offset` from a given `Coord`.
    ///
    /// Returns `None` when either `Coord` component would be negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use gridd::{Coord, Offset};
    ///
    /// let coord: Coord = (3, 5);
    ///
    /// let v1 = Offset::from((-3, 2));
    /// assert_eq!(Some((0, 7)), v1.rcoord(coord));
    ///
    /// let v2 = Offset::from((-4, 5));
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
}

//////////////////////////////////////////////////////////////////////////////
// Fixed-Size 2D Grids
//////////////////////////////////////////////////////////////////////////////

/// Two-dimensional, non-resizeable, zero-indexed grid.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Grid<T> {
    col_count: usize,
    row_count: usize,
    data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Copy,
{
    //////////////////////////////////
    // Instantiation
    //////////////////////////////////

    /// Create a new `Grid` populated with a default value.
    pub fn new(col_count: usize, row_count: usize, default: T) -> Self
    {
        let capactiy = row_count * col_count;

        Self {
            col_count: col_count,
            row_count: row_count,
            data: vec![default; capactiy],
        }
    }

    /// Create a new `Grid` in a square shape, populated with a default
    /// value.
    pub fn square(side_len: usize, default: T) -> Self
    {
        Self::new(side_len, side_len, default)
    }

    //////////////////////////////////
    // Other Operations
    //////////////////////////////////

    /// Perform a transposition.
    pub fn transpose(&self) -> Self {
        if let Some(&val) = self.get((0, 0)) {
            let mut new_grid = Self::new(self.row_count, self.col_count, val);

            for src_row in 0..self.row_count {
                for src_col in 0..self.col_count {
                    if let Some(&val) = self.get((src_col, src_row)) {
                        new_grid.set((src_row, src_col), val)
                    }
                }
            }

            new_grid
        } else {
            Self {
                col_count: 0,
                row_count: 0,
                data: Vec::new(),
            }
        }
    }
}

impl<T> Grid<T> {
    //////////////////////////////////
    // Utilities
    //////////////////////////////////

    /// Get the flat-vector index from the column and row indices.
    fn flat_index(&self, (col, row): Coord) -> usize {
        col + self.col_count * row
    }

    //////////////////////////////////
    // Get & Set
    //////////////////////////////////

    /// Get a `Grid`'s column count.
    pub fn col_count(&self) -> usize {
        self.col_count
    }

    /// Get a `Grid`'s row count.
    pub fn row_count(&self) -> usize {
        self.row_count
    }

    /// Get an immutable reference to some cell.
    pub fn get(&self, coord: Coord) -> Option<&T>
    {
        if self.contains(coord) {
            let index = self.flat_index(coord);

            Some(&self.data[index])
        } else {
            None
        }
    }

    /// Get a mutable reference to some cell.
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        if self.contains(coord) {
            let index = self.flat_index(coord);

            Some(&mut self.data[index])
        } else {
            None
        }
    }

    /// Get an immutable reference to the cell with the given positional
    /// relationship to the provided coordinate.
    pub fn rget(&self, anchor: Coord, vec: Offset) -> Option<&T> {
        match vec.rcoord(anchor) {
            Some(coord) => self.get(coord),
            _ => None,
        }
    }

    /// Get a mutable reference to the cell with the given positional
    /// relationship to the provided coordinate.
    pub fn rget_mut(&mut self, anchor: Coord, vec: Offset) -> Option<&mut T> {
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

    /// Set the value of a cell with the given positional relationship to
    /// the provided coordinate.
    pub fn rset(&mut self, coord: Coord, vec: Offset, new_val: T) {
        if let Some(rcoord) = vec.rcoord(coord) {
            self.set(rcoord, new_val);
        }
    }

    //////////////////////////////////
    // Boolean Operations
    //////////////////////////////////

    /// Determine if a coordinate is within the grid
    pub fn contains(&self, (col, row): Coord) -> bool {
        col < self.col_count && row < self.row_count
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
        let mut grid = Grid::new(1, 1, 'a');
        let value = grid.get_mut((0, 0)).unwrap();

        assert_eq!(&'a', value);
        *value = 'b';
        assert_eq!(&'b', value);
    }

    #[test]
    fn test_get_set() {
        let mut grid = Grid::new(5, 5, 'a');

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
        let mut grid = Grid::new(5, 5, 'a');

        assert_eq!(Some(&'a'), grid.get((2, 3)));

        grid.set((2, 3), 'b');

        assert_eq!(Some(&'b'), grid.rget((2, 4), Offset::NORTH));
        assert_eq!(Some(&mut 'b'), grid.rget_mut((2, 4), Offset::NORTH));
    }

    #[test]
    fn test_rset() {
        let mut grid = Grid::new(5, 5, 'a');

        assert_eq!(Some(&'a'), grid.get((2, 3)));

        grid.rset((1, 1), Offset::from((1, 2)), 'b');

        assert_eq!(Some(&'b'), grid.rget((2, 4), Offset::NORTH));
        assert_eq!(Some(&mut 'b'), grid.rget_mut((2, 4), Offset::NORTH));
    }

    #[test]
    fn test_transpose() {
        let src_col = 3;
        let src_row = 4;

        let mut grid = Grid::new(src_col, src_row, (0, 0));

        for i in 0..src_col {
            for j in 0..src_row {
                grid.set((i, j), (i, j));
            }
        }

        let tgrid = grid.transpose();

        assert_eq!(4, tgrid.col_count());
        assert_eq!(3, tgrid.row_count());

        for i in 0..src_col {
            for j in 0..src_row {
                assert_eq!(tgrid.get((j, i)), grid.get((i, j)));
            }
        }
    }
}

//! ## Position
//!
//! This module contains the definition of the [`Pos`] type, used to represent positions in the rest of the library;

use std::ops::{Add, Mul, Sub};

/// A discrete position on a 2D-grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(isize, isize);

impl Pos {
    ///  Constructor.
    pub fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }

    /// Origin of the 2D-grid.
    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    /// Unit length on both axis of the 2D-grid.
    pub fn one() -> Self {
        Self::new(1, 1)
    }

    /// Constructor for positions of diagonal vectors.
    pub fn sized(scale: isize) -> Self {
        Self::one().scale(scale)
    }

    /// Scale the vector by an integer value.
    pub fn scale(self, factor: isize) -> Self {
        self * Self::sized(factor)
    }

    /// Accessor of the x-coordinate.
    pub fn x(self) -> isize {
        self.0
    }

    /// Accessor of the y-coordinate.
    pub fn y(self) -> isize {
        self.1
    }

    /// Decompose the position in both its x-part and y-part as a tuple.
    pub fn decompose(self) -> (isize, isize) {
        (self.x(), self.y())
    }
}

impl From<(isize, isize)> for Pos {
    fn from((x, y): (isize, isize)) -> Self {
        Self::new(x, y)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x() + rhs.x();
        let y = self.y() + rhs.y();
        Self::new(x, y)
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x() - rhs.x();
        let y = self.y() - rhs.y();
        Self::new(x, y)
    }
}

impl Mul for Pos {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x() * rhs.x();
        let y = self.y() * rhs.y();
        Self::new(x, y)
    }
}

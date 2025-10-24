use std::ops::{Add, Div, Mul, Neg, Sub};

use num::{cast, Float, NumCast};

/// Represents a 2D point (or any pair of objects).
#[allow(missing_docs)]
#[derive(
  Clone, Copy, Default, Debug, Display, PartialEq, Eq, Hash, PartialOrd,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[display("({}, {})", "x", "y")]
pub struct Point<T> {
  pub x: T,
  pub y: T,
}

impl<T> Point<T> {
  /// Create a new point
  pub const fn new(x: T, y: T) -> Point<T> {
    Point { x, y }
  }
}

impl<T: Div<Output = T>> Div for Point<T> {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x / rhs.x,
      y: self.y / rhs.y,
    }
  }
}

impl<T: Mul<Output = T>> Mul for Point<T> {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
    }
  }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Point<T> {
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self {
      x: self.x * rhs,
      y: self.y * rhs,
    }
  }
}

impl<T: Add<Output = T>> Add for Point<T> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

impl<T: Neg<Output = T>> Neg for Point<T> {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self {
      x: -self.x,
      y: -self.y,
    }
  }
}

impl<T: Copy + NumCast> Point<T> {
  /// Converts this point to a floating point point
  #[inline]
  pub fn to_float<U: Float>(&self) -> Option<Point<U>> {
    Some(Point::new(cast(self.x)?, cast(self.y)?))
  }
}

impl<T: Mul<Output = T> + Add<Output = T>> Point<T> {
  #[inline]
  /// Dot product of two points (element-wise multiplication)
  pub fn dot(self, other: Self) -> T {
    self.x * other.x + self.y * other.y
  }
}

impl<T: Float> Point<T> {
  /// Calculates the Euclidean distance between 2 points.
  #[inline]
  pub fn distance(&self, other: Point<T>) -> T {
    self.distance_squared(other).sqrt()
  }

  /// Calculates the Euclidean distance squared between 2 points. Faster
  #[inline]
  pub fn distance_squared(&self, other: Point<T>) -> T {
    let dx = other.x.sub(self.x);
    let dy = other.y.sub(self.y);
    dx * dx + dy * dy
  }

  /// Calculates the Euclidean distance between this point and origin.
  #[inline]
  pub fn length(&self) -> T {
    self.length_squared().sqrt()
  }

  /// Calculates the Euclidean distance squared between this point and origin. Faster
  #[inline]
  pub fn length_squared(&self) -> T {
    self.x * self.x + self.y * self.y
  }

  /// Calculates the magnitude of the vector.
  #[inline]
  pub fn magnitude(&self) -> T {
    (self.x * self.x + self.y * self.y).sqrt()
  }

  /// Calculates the norm of the vector.
  #[inline]
  pub fn norm(&self) -> Point<T> {
    let m = self.magnitude();
    Point::new(self.x / m, self.y / m)
  }
}

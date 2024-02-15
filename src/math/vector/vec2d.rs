//! 2-dimensional vectors
//!
//! This module provides [`Vec2D`].

use std::ops::{Add, Mul, Neg};

use num::{Num, Signed};

/// A two-dimensional vector $\left[\begin{matrix}x&y\end{matrix}\right]$ backed
/// by an integer type `I`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2D<I>(pub I, pub I);

impl<I> Vec2D<I> {
    /// Create a [`Vec2D`] $\left[\begin{matrix}x&y\end{matrix}\right]$ from a
    /// tuple $(x, y)$.
    ///
    /// Using the [`Vec2D`] constructor directly (e.g. `Vec2D(1, 2)`) is
    /// preferred, however if you already have a tuple, this is easier.
    ///
    /// ```
    /// use handyman::math::vector::Vec2D;
    /// assert_eq!(Vec2D::from_tuple(1, 3), Vec2D(1, 3));
    /// ```
    #[must_use]
    pub fn from_tuple((x, y): (I, I)) -> Self {
        Self(x, y)
    }

    /// Obtain the $x$ component from a vector
    /// $\left[\begin{matrix}x&y\end{matrix}\right]$
    ///
    /// ```
    /// use handyman::math::vector::Vec2D;
    /// assert_eq!(Vec2D(1, 2).x(), 1);
    /// ```
    #[must_use]
    pub fn x(self) -> I {
        self.0
    }

    /// Obtain the $y$ component from a vector
    /// $\left[\begin{matrix}x&y\end{matrix}\right]$
    ///
    /// ```
    /// use handyman::math::vector::Vec2D;
    /// assert_eq!(Vec2D(1, 2).y(), 2);
    /// ```
    #[must_use]
    pub fn y(self) -> I {
        self.1
    }

    /// Apply a function $f$ onto both components of this vector
    ///
    /// For a vector $\vec v=\left[\begin{matrix}v_x&v_y\end{matrix}\right]$,
    /// `v.apply(f)` will return
    /// $\vec{v'}=\left[\begin{matrix}f(v_x)&f(v_y)\end{matrix}\right]$.
    ///
    /// ```
    /// use handyman::math::vector::Vec2D;
    /// fn add_two(x: i32) -> i32 { x + 2 }
    /// assert_eq!(Vec2D(1, 2).apply(add_two), Vec2D(3, 4));
    /// ```
    pub fn apply<F, U>(self, mut f: F) -> Vec2D<U>
    where
        F: FnMut(I) -> U,
    {
        Vec2D(f(self.0), f(self.1))
    }

    /// Apply a function $f$ onto the corresponding components of two vectors
    ///
    /// For the vectors $\vec a=\left[\begin{matrix}a_x&x_y\end{matrix}\right]$
    /// and $\vec b=\left[\begin{matrix}b_x&b_y\end{matrix}\right]$ and a
    /// function $f(a, b)$, `a.zip_with(b, f)` will yield the vector $\vec
    /// c=\left[\begin{matrix}f(a_x, b_x)&f(a_y, b_y)\end{matrix}\right]$.
    ///
    /// Example (trivial implementation of [`Vec2D::add`]):
    /// ```
    /// use handyman::math::vector::Vec2D;
    /// assert_eq!(Vec2D(1, 2).zip_with(Vec2D(3, 4), |a, b| a + b), Vec2D(4, 6));
    /// ```
    pub fn zip_with<F, O, U>(self, other: Vec2D<O>, mut f: F) -> Vec2D<U>
    where
        F: FnMut(I, O) -> U,
    {
        Vec2D(f(self.0, other.0), f(self.1, other.1))
    }
}

impl<I: Num> Vec2D<I> {
    /// The additive identity vector
    /// $\left[\begin{matrix}0&0\end{matrix}\right]$
    #[must_use]
    pub fn zero() -> Self {
        Self(I::zero(), I::zero())
    }

    /// The multiplicative identity vector
    /// $\left[\begin{matrix}1&1\end{matrix}\right]$
    #[must_use]
    pub fn one() -> Self {
        Self(I::one(), I::one())
    }
}

impl<I: Num + Copy> Mul<I> for Vec2D<I> {
    type Output = Self;
    /// Multiply this vector $\vec v$ by a scalar $k$, yielding $k\vec v$.
    ///
    /// ```
    /// use handyman::math::vector::Vec2D;
    /// assert_eq!(Vec2D(1, 3) * 2, Vec2D(2, 6));
    /// ```
    fn mul(self, rhs: I) -> Self::Output {
        self.apply(|x| x * rhs)
    }
}
impl<I: Num + Copy + Signed> Neg for Vec2D<I> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * I::one().neg()
    }
}

impl<I: Num + Copy> Add for Vec2D<I> {
    type Output = Self;
    /// Add two vectors.
    ///
    /// Given the vectors $\vec
    /// a=\left[\begin{matrix}a_x&a_y\end{matrix}\right]$ and $\vec
    /// b=\left[\begin{matrix}b_x&b_y\end{matrix}\right]$, yielding a vector
    /// $\vec c=\left[\begin{matrix}a_x+b_x&a_y+b_y\end{matrix}\right]$.
    ///
    /// ```
    /// use handyman::math::vector::Vec2D;
    /// assert_eq!(Vec2D(1, 2) + Vec2D(3, 4), Vec2D(4, 6));
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        self.zip_with(rhs, I::add)
    }
}

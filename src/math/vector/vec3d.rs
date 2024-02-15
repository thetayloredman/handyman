//! 3-dimensional vectors
//!
//! This module provides [`Vec3D`].

use std::ops::{Add, Mul, Neg};

use num::{Num, Signed};

/// A three-dimensional vector $\left[\begin{matrix}x&y&z\end{matrix}\right]$
/// backed by an integer type `I`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3D<I>(pub I, pub I, pub I);

impl<I> Vec3D<I> {
    /// Create a [`Vec3D`] $\left[\begin{matrix}x&y&z\end{matrix}\right]$ from a
    /// tuple $(x, y, z)$.
    ///
    /// Using the [`Vec3D`] constructor directly (e.g. `Vec3D(1, 2, 3)`) is
    /// preferred, however if you already have a tuple, this is easier.
    ///
    /// ```
    /// use handyman::math::vector::Vec3D;
    /// assert_eq!(Vec3D::from_tuple(1, 3, 5), Vec3D(1, 3, 5));
    /// ```
    #[must_use]
    pub fn from_tuple((x, y, z): (I, I, I)) -> Self {
        Self(x, y, z)
    }

    /// Obtain the $x$ component from a vector
    /// $\left[\begin{matrix}x&y&z\end{matrix}\right]$
    #[must_use]
    pub fn x(self) -> I {
        self.0
    }

    /// Obtain the $y$ component from a vector
    /// $\left[\begin{matrix}x&y&z\end{matrix}\right]$
    #[must_use]
    pub fn y(self) -> I {
        self.1
    }

    // Obtain the $z$ component from a vector
    /// $\left[\begin{matrix}x&y&z\end{matrix}\right]$
    #[must_use]
    pub fn z(self) -> I {
        self.2
    }

    /// Apply a function $f$ onto both components of this vector
    ///
    /// For a vector $\vec
    /// v=\left[\begin{matrix}v_x&v_y&v_z\end{matrix}\right]$, `v.apply(f)`
    /// will return $\vec{v'}=\left[\begin{matrix}f(v_x)&f(v_y)&f(v_z)\
    /// end{matrix}\right]$.
    pub fn apply<F, U>(self, mut f: F) -> Vec3D<U>
    where
        F: FnMut(I) -> U,
    {
        Vec3D(f(self.0), f(self.1), f(self.2))
    }

    /// Apply a function $f$ onto the corresponding components of two vectors
    ///
    /// For the vectors $\vec
    /// a=\left[\begin{matrix}a_x&a_y&a_z\end{matrix}\right]$ and $\vec
    /// b=\left[\begin{matrix}b_x&b_y&b_z\end{matrix}\right]$ and a function
    /// $f(a, b)$, `a.zip_with(b, f)` will yield the vector $\vec
    /// c=\left[\begin{matrix}f(a_x, b_x)&f(a_y,
    /// b_y)&f(a_z,b_z)\end{matrix}\right]$.
    pub fn zip_with<F, O, U>(self, other: Vec3D<O>, mut f: F) -> Vec3D<U>
    where
        F: FnMut(I, O) -> U,
    {
        Vec3D(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2))
    }
}

impl<I: Num> Vec3D<I> {
    /// The additive identity vector
    /// $\left[\begin{matrix}0&0&0\end{matrix}\right]$
    #[must_use]
    pub fn zero() -> Self {
        Self(I::zero(), I::zero(), I::zero())
    }

    /// The multiplicative identity vector
    /// $\left[\begin{matrix}1&1&1\end{matrix}\right]$
    #[must_use]
    pub fn one() -> Self {
        Self(I::one(), I::one(), I::one())
    }
}

impl<I: Num + Copy> Mul<I> for Vec3D<I> {
    type Output = Self;
    /// Multiply this vector $\vec v$ by a scalar $k$, yielding $k\vec v$.
    fn mul(self, rhs: I) -> Self::Output {
        self.apply(|x| x * rhs)
    }
}
impl<I: Num + Copy + Signed> Neg for Vec3D<I> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * I::one().neg()
    }
}

impl<I: Num + Copy> Add for Vec3D<I> {
    type Output = Self;
    /// Add two vectors.
    ///
    /// Given the vectors $\vec
    /// a=\left[\begin{matrix}a_x&a_y&a_z\end{matrix}\right]$ and $\vec
    /// b=\left[\begin{matrix}b_x&b_y&a_z\end{matrix}\right]$, yielding a vector
    /// $\vec c=\left[\begin{matrix}a_x+b_x&a_y+b_y&a_z+b_z\end{matrix}\right]$.
    fn add(self, rhs: Self) -> Self::Output {
        self.zip_with(rhs, I::add)
    }
}

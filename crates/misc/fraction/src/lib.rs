use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign},
};

use numeric::{one::One, zero::Zero};

mod private {
    pub trait Sealed {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for i128 {}
}

pub trait FractionTrait:
    private::Sealed
    + Zero
    + One
    + Sized
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + Mul<Output = Self>
    + MulAssign
    + Rem<Output = Self>
    + Copy
    + DivAssign
{
}

impl FractionTrait for i32 {}
impl FractionTrait for i64 {}
impl FractionTrait for i128 {}

#[derive(Clone, Copy)]
pub struct Fraction<T: FractionTrait> {
    // 分子
    numerator: T,
    // 分母
    denominator: T,
}

impl<T: FractionTrait> Default for Fraction<T> {
    fn default() -> Self {
        Self {
            numerator: T::zero(),
            denominator: T::one(),
        }
    }
}

impl<T: FractionTrait> From<T> for Fraction<T> {
    fn from(value: T) -> Self {
        Self {
            numerator: value,
            denominator: T::one(),
        }
    }
}

impl<T: FractionTrait> Fraction<T> {
    pub fn new(numerator: T, denominator: T) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn gcd(a: T, b: T) -> T {
        let mut s = if a > T::zero() { a } else { -a };
        let mut t = if b > T::zero() { b } else { -b };
        while s % t != T::zero() {
            let u = s % t;
            s = t;
            t = u;
        }
        t
    }

    pub fn normalize(&mut self) {
        let g = Self::gcd(self.numerator, self.denominator);
        self.numerator /= g;
        self.denominator /= g;
        if self.denominator < T::zero() {
            self.numerator = -self.numerator;
            self.denominator = -self.denominator;
        }
    }
}

impl<T: FractionTrait> AddAssign for Fraction<T> {
    fn add_assign(&mut self, rhs: Self) {
        let new_numerator = self.numerator * rhs.denominator + self.denominator * rhs.numerator;
        let new_denominator = self.denominator * rhs.denominator;
        self.numerator = new_numerator;
        self.denominator = new_denominator;
        self.normalize();
    }
}

impl<T: FractionTrait> Add for Fraction<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut f = self;
        f += rhs;
        f
    }
}

impl<T: FractionTrait> SubAssign for Fraction<T> {
    fn sub_assign(&mut self, rhs: Self) {
        let new_numerator = self.numerator * rhs.denominator - self.denominator * rhs.numerator;
        let new_denominator = self.denominator * rhs.denominator;
        self.numerator = new_numerator;
        self.denominator = new_denominator;
        self.normalize();
    }
}

impl<T: FractionTrait> Sub for Fraction<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut f = self;
        f -= rhs;
        f
    }
}

impl<T: FractionTrait> MulAssign for Fraction<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.numerator *= rhs.numerator;
        self.denominator *= rhs.denominator;
        self.normalize();
    }
}

impl<T: FractionTrait> Mul for Fraction<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut f = self;
        f *= rhs;
        f
    }
}

impl<T: FractionTrait> DivAssign for Fraction<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.numerator *= rhs.denominator;
        self.denominator *= rhs.numerator;
        self.normalize();
    }
}

impl<T: FractionTrait> Div for Fraction<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut f = self;
        f /= rhs;
        f
    }
}

impl<T: FractionTrait> Neg for Fraction<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl<T: FractionTrait> PartialEq for Fraction<T> {
    fn eq(&self, other: &Self) -> bool {
        self.numerator * other.denominator == self.denominator * other.numerator
    }
}

impl<T: FractionTrait> Eq for Fraction<T> {}

impl<T: FractionTrait> Ord for Fraction<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.numerator * other.denominator < self.denominator * other.numerator {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl<T: FractionTrait> PartialOrd for Fraction<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::Fraction;
    use rand::Rng;

    #[test]
    fn fraction_test() {
        let mut rng = rand::rng();

        let mut generate_random_fraction = || {
            let numerator = rng.random_range(-100000..100000 + 1);
            let mut denominator = rng.random_range(-100000..100000 + 1);
            while denominator == 0 {
                denominator = rng.random_range(-100000..100000 + 1);
            }
            Fraction::new(numerator, denominator)
        };

        for _ in 0..100 {
            let f1 = generate_random_fraction();
            let f2 = generate_random_fraction();
            let mut f = f1 * f2 / f2;
            assert!(f == f1);
            f = f1 + f2 - f2;
            assert!(f == f1);
            f = f1 - f2 + f2;
            assert!(f == f1);
            f = f1 / f2 * f2;
            assert!(f == f1);
            f = f2 / f1 * f1;
            assert!(f != f1);
            f = f1 + f2 * Fraction::<i64>::from(2);
            assert!(f != f1);
        }
    }
}

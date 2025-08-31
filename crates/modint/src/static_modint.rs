use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use numeric::{one::One, zero::Zero};

use crate::modulus::Modulus;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StaticModInt<M>
where
    M: Modulus,
{
    value: u32,
    _phantom: PhantomData<fn() -> M>,
}

impl<M> StaticModInt<M>
where
    M: Modulus,
{
    #[inline]
    pub fn raw(value: u32) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn new(value: i64) -> Self {
        let value = if value >= 0 {
            (value % (M::MOD as i64)) as u32
        } else {
            (M::MOD - (((-value) % (M::MOD as i64)) as u32)) % M::MOD
        };
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn modulus() -> u32 {
        M::MOD
    }

    #[inline]
    pub fn val(self) -> u32 {
        self.value
    }

    #[inline]
    pub fn inv(self) -> Self {
        let mut a = self.value as i64;
        let mut b = M::MOD as i64;
        let mut u = 1_i64;
        let mut v = 0_i64;
        let mut t = 0_i64;
        while b > 0 {
            t = a / b;
            a -= t * b;
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut u, &mut v);
        }
        Self {
            value: u as u32,
            _phantom: PhantomData,
        }
    }

    pub fn pow(self, n: u64) -> Self {
        let mut ret = Self::raw(1);
        let mut mul = self;
        let mut n = n;
        while n > 0 {
            if (n & 1) == 1 {
                ret *= mul;
            }
            mul *= mul;
            n >>= 1;
        }
        ret
    }
}

impl<M> Add for StaticModInt<M>
where
    M: Modulus,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self.value + rhs.value;
        if res >= M::MOD {
            res -= M::MOD;
        }
        Self::raw(res)
    }
}

impl<M> Sub for StaticModInt<M>
where
    M: Modulus,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = self.value + M::MOD - rhs.value;
        if res >= M::MOD {
            res -= M::MOD;
        }
        Self::raw(res)
    }
}

impl<M> Mul for StaticModInt<M>
where
    M: Modulus,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let res = (self.value as u64) * (rhs.value as u64) % (M::MOD as u64);
        Self::raw(res as u32)
    }
}

impl<M> Div for StaticModInt<M>
where
    M: Modulus,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<M> Neg for StaticModInt<M>
where
    M: Modulus,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * Self::raw(M::MOD - 1)
    }
}

impl<M> AddAssign for StaticModInt<M>
where
    M: Modulus,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<M> SubAssign for StaticModInt<M>
where
    M: Modulus,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<M> MulAssign for StaticModInt<M>
where
    M: Modulus,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<M> DivAssign for StaticModInt<M>
where
    M: Modulus,
{
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<M> Display for StaticModInt<M>
where
    M: Modulus,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<M> Zero for StaticModInt<M>
where
    M: Modulus,
{
    fn zero() -> Self {
        Self::raw(0)
    }
}

impl<M> One for StaticModInt<M>
where
    M: Modulus,
{
    fn one() -> Self {
        Self::new(1)
    }
}

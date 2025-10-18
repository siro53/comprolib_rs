use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use crate::Monoid;
use numeric::{one::One, zero::Zero};

#[derive(Clone)]
pub struct Affine<T>(pub T, pub T);

impl<T> Affine<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T>,
{
    pub fn eval(self, x: T) -> T {
        self.0 * x + self.1
    }
}

pub struct AffineOperator<T, const REV: bool = false>(PhantomData<fn() -> T>);

impl<T, const REV: bool> Monoid for AffineOperator<T, REV>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Zero + One,
{
    type ValueType = Affine<T>;

    fn op(left_value: &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {
        if REV {
            return Affine(
                right_value.0 * left_value.0,
                right_value.0 * left_value.1 + right_value.1,
            );
        }
        Affine(
            left_value.0 * right_value.0,
            left_value.0 * right_value.1 + left_value.1,
        )
    }

    fn unit() -> Self::ValueType {
        Affine(T::one(), T::zero())
    }
}

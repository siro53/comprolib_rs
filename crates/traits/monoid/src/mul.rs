use std::{marker::PhantomData, ops::Mul};

use crate::Monoid;
use numeric::one::One;

pub struct Multiplicative<T>(PhantomData<fn() -> T>);

impl<T> Monoid for Multiplicative<T>
where
    T: Mul<Output = T> + Copy + One,
{
    type ValueType = T;

    fn op(left_value: &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {
        *left_value * *right_value
    }

    fn unit() -> Self::ValueType {
        T::one()
    }
}

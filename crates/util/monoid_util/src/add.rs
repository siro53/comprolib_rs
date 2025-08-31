use std::{marker::PhantomData, ops::Add};

use monoid::Monoid;
use numeric::zero::Zero;

pub struct Additive<T>(PhantomData<fn() -> T>);

impl<T> Monoid for Additive<T>
where
    T: Add<Output = T> + Copy + Zero,
{
    type ValueType = T;

    fn op(left_value: &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {
        *left_value + *right_value
    }

    fn unit() -> Self::ValueType {
        T::zero()
    }
}

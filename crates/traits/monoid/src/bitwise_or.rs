use std::{marker::PhantomData, ops::BitOr};

use crate::Monoid;
use numeric::zero::Zero;

pub struct BitwiseOr<T>(PhantomData<fn() -> T>);

impl<T> Monoid for BitwiseOr<T>
where
    T: Copy + BitOr<Output = T> + Zero,
{
    type ValueType = T;

    fn op(left_value: &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {
        *left_value | *right_value
    }

    fn unit() -> Self::ValueType {
        T::zero()
    }
}

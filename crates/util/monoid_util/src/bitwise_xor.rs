use std::{marker::PhantomData, ops::BitXor};

use monoid::Monoid;
use numeric::zero::Zero;

pub struct BitwiseXor<T>(PhantomData<fn() -> T>);

impl<T> Monoid for BitwiseXor<T>
where
    T: Copy + BitXor<Output = T> + Zero,
{
    type ValueType = T;

    fn op(left_value: &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {
        *left_value ^ *right_value
    }

    fn unit() -> Self::ValueType {
        T::zero()
    }
}

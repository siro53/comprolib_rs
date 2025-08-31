use std::{
    marker::PhantomData,
    ops::{BitAnd, Not},
};

use monoid::Monoid;
use numeric::zero::Zero;

pub struct BitwiseAnd<T>(PhantomData<fn() -> T>);

impl<T> Monoid for BitwiseAnd<T>
where
    T: Copy + BitAnd<Output = T> + Not<Output = T> + Zero,
{
    type ValueType = T;

    fn op(left_value: &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {
        *left_value & *right_value
    }

    fn unit() -> Self::ValueType {
        !T::zero()
    }
}

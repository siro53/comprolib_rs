use std::marker::PhantomData;

use crate::Monoid;
use numeric::bound::BoundedAbove;

pub struct Min<T>(PhantomData<fn() -> T>);

impl<T> Monoid for Min<T>
where
    T: Copy + Ord + BoundedAbove,
{
    type ValueType = T;

    fn op(left_value: &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {
        std::cmp::min(*left_value, *right_value)
    }

    fn unit() -> Self::ValueType {
        T::max_value()
    }
}

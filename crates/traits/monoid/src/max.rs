use std::marker::PhantomData;

use crate::Monoid;
use numeric::bound::BoundedBelow;

pub struct Max<T>(PhantomData<fn() -> T>);

impl<T> Monoid for Max<T>
where
    T: Copy + Ord + BoundedBelow,
{
    type ValueType = T;

    fn op(left_value: &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {
        std::cmp::max(*left_value, *right_value)
    }

    fn unit() -> Self::ValueType {
        T::min_value()
    }
}

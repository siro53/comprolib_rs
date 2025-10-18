use std::marker::PhantomData;

use crate::Monoid;
use numeric::bound::BoundedBelow;

#[derive(Clone, Copy)]
#[allow(unused)]
pub struct MaxWithIndex<T> {
    max_value: T,
    max_index: usize,
}

pub struct MaxWithIndexOperator<T>(PhantomData<fn() -> T>);

impl<T> Monoid for MaxWithIndexOperator<T>
where
    T: Copy + Ord + BoundedBelow,
{
    type ValueType = MaxWithIndex<T>;

    fn op(left_value: &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {
        if left_value.max_value < right_value.max_value {
            *right_value
        } else {
            *left_value
        }
    }

    fn unit() -> Self::ValueType {
        MaxWithIndex {
            max_value: T::min_value(),
            max_index: 0,
        }
    }
}

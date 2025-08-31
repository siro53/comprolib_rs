use std::marker::PhantomData;

use monoid::Monoid;
use numeric::bound::BoundedAbove;

#[derive(Clone, Copy)]
#[allow(unused)]
pub struct MinWithIndex<T> {
    min_value: T,
    min_index: usize,
}

pub struct MinWithIndexOperator<T>(PhantomData<fn() -> T>);

impl<T> Monoid for MinWithIndexOperator<T>
where
    T: Copy + Ord + BoundedAbove,
{
    type ValueType = MinWithIndex<T>;

    fn op(left_value: &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType {
        if left_value.min_value > right_value.min_value {
            *right_value
        } else {
            *left_value
        }
    }

    fn unit() -> Self::ValueType {
        MinWithIndex {
            min_value: T::max_value(),
            min_index: 0,
        }
    }
}

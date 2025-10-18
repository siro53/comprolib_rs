pub trait Monoid {
    type ValueType: Clone;
    fn op(left_value: &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType;
    fn unit() -> Self::ValueType;
}

pub mod add;
pub mod affine;
pub mod bitwise_and;
pub mod bitwise_or;
pub mod bitwise_xor;
pub mod max;
pub mod max_with_index;
pub mod min;
pub mod min_with_index;
pub mod mul;

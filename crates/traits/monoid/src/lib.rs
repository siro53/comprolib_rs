pub trait Monoid {
    type ValueType: Clone;
    fn op(left_value: &Self::ValueType, right_value: &Self::ValueType) -> Self::ValueType;
    fn unit() -> Self::ValueType;
}

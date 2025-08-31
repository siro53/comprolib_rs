pub trait BoundedBelow {
    fn min_value() -> Self;
}

pub trait BoundedAbove {
    fn max_value() -> Self;
}

macro_rules! impl_bound_integer {
    ($($ty:ty),*) => {
        $(
            impl BoundedBelow for $ty {
                #[inline]
                fn min_value() -> Self {
                    Self::MIN
                }
            }

            impl BoundedAbove for $ty {
                #[inline]
                fn max_value() -> Self {
                    Self::MAX
                }
            }
        )*
    };
}

impl_bound_integer!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_zero_integer {
    ($($ty:ty),*) => {
        $(
            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    0
                }
            }
        )*
    };
}

macro_rules! impl_zero_float {
    ($($ty:ty),*) => {
        $(
            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    0.
                }
            }
        )*
    };
}

impl_zero_integer!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

impl_zero_float!(f32, f64);

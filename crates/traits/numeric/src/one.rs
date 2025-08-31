pub trait One {
    fn one() -> Self;
}

macro_rules! impl_one_integer {
    ($($ty:ty),*) => {
        $(
            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1
                }
            }
        )*
    };
}

macro_rules! impl_one_float {
    ($($ty:ty),*) => {
        $(
            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1.
                }
            }
        )*
    };
}

impl_one_integer!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

impl_one_float!(f32, f64);

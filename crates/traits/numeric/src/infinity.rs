pub trait Infinity {
    fn infinity() -> Self;
}

pub trait NegInfinity {
    fn negative_infinity() -> Self;
}

macro_rules! impl_infinity {
    ($($t:ty, $infinity_value:expr,)*) => {
        $(
            impl Infinity for $t {
                fn infinity() -> Self {
                    $infinity_value
                }
            }
        )*
    }
}

macro_rules! impl_negative_infinity {
    ($($t:ty, $neg_infinity_value:expr,)*) => {
        $(
            impl NegInfinity for $t {
                fn negative_infinity() -> Self {
                    $neg_infinity_value
                }
            }
        )*
    }
}

impl_infinity! {
    i32, 1_i32 << 30,
    u32, 1_u32 << 30,
    f32, 1e10,
    i64, 1_i64 << 60,
    isize, 1_isize << 60,
    u64, 1_u64 << 60,
    usize, 1_usize << 60,
    f64, 1e20,
    i128, 1_i128 << 120,
    u128, 1_u128 << 120,
}

impl_negative_infinity! {
    i32, -1_i32 << 30,
    u32, 0,
    f32, -1e10,
    i64, -1_i64 << 60,
    isize, -1_isize << 60,
    u64, 0,
    usize, 0,
    f64, 1e20,
    i128, -1_i128 << 120,
    u128, 0,
}

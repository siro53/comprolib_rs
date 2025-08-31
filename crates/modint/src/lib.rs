use crate::{modulus::Modulus, static_modint::StaticModInt};

pub mod modulus;
pub mod static_modint;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mod998244353 {}

impl Modulus for Mod998244353 {
    const MOD: u32 = 998_244_353;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mod1000000007 {}

impl Modulus for Mod1000000007 {
    const MOD: u32 = 1_000_000_007;
}

pub type ModInt998244353 = StaticModInt<Mod998244353>;
pub type ModInt1000000007 = StaticModInt<Mod1000000007>;

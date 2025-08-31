---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/modint/src/lib.rs
    title: crates/modint/src/lib.rs
  - icon: ':warning:'
    path: crates/modint/src/modulus.rs
    title: crates/modint/src/modulus.rs
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/modint/src/lib.rs
    title: crates/modint/src/lib.rs
  - icon: ':warning:'
    path: crates/modint/src/modulus.rs
    title: crates/modint/src/modulus.rs
  - icon: ':warning:'
    path: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
    title: verify/library_checker/data_structure/point_set_range_composite/src/main.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/home/runner/.local/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    fmt::{Debug, Display},\n    marker::PhantomData,\n    ops::{Add,\
    \ AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},\n};\n\nuse\
    \ numeric::{one::One, zero::Zero};\n\nuse crate::modulus::Modulus;\n\n#[derive(Clone,\
    \ Copy, PartialEq, Eq, Debug)]\npub struct StaticModInt<M>\nwhere\n    M: Modulus,\n\
    {\n    value: u32,\n    _phantom: PhantomData<fn() -> M>,\n}\n\nimpl<M> StaticModInt<M>\n\
    where\n    M: Modulus,\n{\n    #[inline]\n    pub fn raw(value: u32) -> Self {\n\
    \        Self {\n            value,\n            _phantom: PhantomData,\n    \
    \    }\n    }\n\n    #[inline]\n    pub fn new(value: i64) -> Self {\n       \
    \ let value = if value >= 0 {\n            (value % (M::MOD as i64)) as u32\n\
    \        } else {\n            (M::MOD - (((-value) % (M::MOD as i64)) as u32))\
    \ % M::MOD\n        };\n        Self {\n            value,\n            _phantom:\
    \ PhantomData,\n        }\n    }\n\n    #[inline]\n    pub fn modulus() -> u32\
    \ {\n        M::MOD\n    }\n\n    #[inline]\n    pub fn val(self) -> u32 {\n \
    \       self.value\n    }\n\n    #[inline]\n    pub fn inv(self) -> Self {\n \
    \       let mut a = self.value as i64;\n        let mut b = M::MOD as i64;\n \
    \       let mut u = 1_i64;\n        let mut v = 0_i64;\n        let mut t = 0_i64;\n\
    \        while b > 0 {\n            t = a / b;\n            a -= t * b;\n    \
    \        std::mem::swap(&mut a, &mut b);\n            std::mem::swap(&mut u, &mut\
    \ v);\n        }\n        Self {\n            value: u as u32,\n            _phantom:\
    \ PhantomData,\n        }\n    }\n\n    pub fn pow(self, n: u64) -> Self {\n \
    \       let mut ret = Self::raw(1);\n        let mut mul = self;\n        let\
    \ mut n = n;\n        while n > 0 {\n            if (n & 1) == 1 {\n         \
    \       ret *= mul;\n            }\n            mul *= mul;\n            n >>=\
    \ 1;\n        }\n        ret\n    }\n}\n\nimpl<M> Add for StaticModInt<M>\nwhere\n\
    \    M: Modulus,\n{\n    type Output = Self;\n\n    fn add(self, rhs: Self) ->\
    \ Self::Output {\n        let mut res = self.value + rhs.value;\n        if res\
    \ >= M::MOD {\n            res -= M::MOD;\n        }\n        Self::raw(res)\n\
    \    }\n}\n\nimpl<M> Sub for StaticModInt<M>\nwhere\n    M: Modulus,\n{\n    type\
    \ Output = Self;\n\n    fn sub(self, rhs: Self) -> Self::Output {\n        let\
    \ mut res = self.value + M::MOD - rhs.value;\n        if res >= M::MOD {\n   \
    \         res -= M::MOD;\n        }\n        Self::raw(res)\n    }\n}\n\nimpl<M>\
    \ Mul for StaticModInt<M>\nwhere\n    M: Modulus,\n{\n    type Output = Self;\n\
    \n    fn mul(self, rhs: Self) -> Self::Output {\n        let res = (self.value\
    \ as u64) * (rhs.value as u64) % (M::MOD as u64);\n        Self::raw(res as u32)\n\
    \    }\n}\n\nimpl<M> Div for StaticModInt<M>\nwhere\n    M: Modulus,\n{\n    type\
    \ Output = Self;\n\n    fn div(self, rhs: Self) -> Self::Output {\n        self\
    \ * rhs.inv()\n    }\n}\n\nimpl<M> Neg for StaticModInt<M>\nwhere\n    M: Modulus,\n\
    {\n    type Output = Self;\n\n    fn neg(self) -> Self::Output {\n        self\
    \ * Self::raw(M::MOD - 1)\n    }\n}\n\nimpl<M> AddAssign for StaticModInt<M>\n\
    where\n    M: Modulus,\n{\n    fn add_assign(&mut self, rhs: Self) {\n       \
    \ *self = *self + rhs\n    }\n}\n\nimpl<M> SubAssign for StaticModInt<M>\nwhere\n\
    \    M: Modulus,\n{\n    fn sub_assign(&mut self, rhs: Self) {\n        *self\
    \ = *self - rhs;\n    }\n}\n\nimpl<M> MulAssign for StaticModInt<M>\nwhere\n \
    \   M: Modulus,\n{\n    fn mul_assign(&mut self, rhs: Self) {\n        *self =\
    \ *self * rhs;\n    }\n}\n\nimpl<M> DivAssign for StaticModInt<M>\nwhere\n   \
    \ M: Modulus,\n{\n    fn div_assign(&mut self, rhs: Self) {\n        *self = *self\
    \ / rhs;\n    }\n}\n\nimpl<M> Display for StaticModInt<M>\nwhere\n    M: Modulus,\n\
    {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n\
    \        write!(f, \"{}\", self.value)\n    }\n}\n\nimpl<M> Zero for StaticModInt<M>\n\
    where\n    M: Modulus,\n{\n    fn zero() -> Self {\n        Self::raw(0)\n   \
    \ }\n}\n\nimpl<M> One for StaticModInt<M>\nwhere\n    M: Modulus,\n{\n    fn one()\
    \ -> Self {\n        Self::new(1)\n    }\n}\n"
  dependsOn:
  - crates/modint/src/lib.rs
  - crates/modint/src/modulus.rs
  isVerificationFile: false
  path: crates/modint/src/static_modint.rs
  requiredBy:
  - verify/library_checker/data_structure/point_set_range_composite/src/main.rs
  - crates/modint/src/modulus.rs
  - crates/modint/src/lib.rs
  timestamp: '2025-08-31 17:55:35+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/modint/src/static_modint.rs
layout: document
redirect_from:
- /library/crates/modint/src/static_modint.rs
- /library/crates/modint/src/static_modint.rs.html
title: crates/modint/src/static_modint.rs
---

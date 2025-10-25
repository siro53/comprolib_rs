---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/traits/numeric/src/bound.rs
    title: crates/traits/numeric/src/bound.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/infinity.rs
    title: crates/traits/numeric/src/infinity.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/lib.rs
    title: crates/traits/numeric/src/lib.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/one.rs
    title: crates/traits/numeric/src/one.rs
  - icon: ':warning:'
    path: crates/traits/numeric/src/zero.rs
    title: crates/traits/numeric/src/zero.rs
  _extendedRequiredBy: []
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
  code: "use std::{\n    cmp::Ordering,\n    ops::{Add, AddAssign, Div, DivAssign,\
    \ Mul, MulAssign, Neg, Rem, Sub, SubAssign},\n};\n\nuse numeric::{one::One, zero::Zero};\n\
    \nmod private {\n    pub trait Sealed {}\n    impl Sealed for i32 {}\n    impl\
    \ Sealed for i64 {}\n    impl Sealed for i128 {}\n}\n\npub trait FractionTrait:\n\
    \    private::Sealed\n    + Zero\n    + One\n    + Sized\n    + PartialOrd\n \
    \   + Add<Output = Self>\n    + Sub<Output = Self>\n    + Neg<Output = Self>\n\
    \    + Mul<Output = Self>\n    + MulAssign\n    + Rem<Output = Self>\n    + Copy\n\
    \    + DivAssign\n{\n}\n\nimpl FractionTrait for i32 {}\nimpl FractionTrait for\
    \ i64 {}\nimpl FractionTrait for i128 {}\n\n#[derive(Clone, Copy)]\npub struct\
    \ Fraction<T: FractionTrait> {\n    // \u5206\u5B50\n    numerator: T,\n    //\
    \ \u5206\u6BCD\n    denominator: T,\n}\n\nimpl<T: FractionTrait> Default for Fraction<T>\
    \ {\n    fn default() -> Self {\n        Self {\n            numerator: T::zero(),\n\
    \            denominator: T::one(),\n        }\n    }\n}\n\nimpl<T: FractionTrait>\
    \ From<T> for Fraction<T> {\n    fn from(value: T) -> Self {\n        Self {\n\
    \            numerator: value,\n            denominator: T::one(),\n        }\n\
    \    }\n}\n\nimpl<T: FractionTrait> Fraction<T> {\n    pub fn new(numerator: T,\
    \ denominator: T) -> Self {\n        Self {\n            numerator,\n        \
    \    denominator,\n        }\n    }\n\n    pub fn gcd(a: T, b: T) -> T {\n   \
    \     let mut s = if a > T::zero() { a } else { -a };\n        let mut t = if\
    \ b > T::zero() { b } else { -b };\n        while s % t != T::zero() {\n     \
    \       let u = s % t;\n            s = t;\n            t = u;\n        }\n  \
    \      t\n    }\n\n    pub fn normalize(&mut self) {\n        let g = Self::gcd(self.numerator,\
    \ self.denominator);\n        self.numerator /= g;\n        self.denominator /=\
    \ g;\n        if self.denominator < T::zero() {\n            self.numerator =\
    \ -self.numerator;\n            self.denominator = -self.denominator;\n      \
    \  }\n    }\n}\n\nimpl<T: FractionTrait> AddAssign for Fraction<T> {\n    fn add_assign(&mut\
    \ self, rhs: Self) {\n        let new_numerator = self.numerator * rhs.denominator\
    \ + self.denominator * rhs.numerator;\n        let new_denominator = self.denominator\
    \ * rhs.denominator;\n        self.numerator = new_numerator;\n        self.denominator\
    \ = new_denominator;\n        self.normalize();\n    }\n}\n\nimpl<T: FractionTrait>\
    \ Add for Fraction<T> {\n    type Output = Self;\n    fn add(self, rhs: Self)\
    \ -> Self::Output {\n        let mut f = self;\n        f += rhs;\n        f\n\
    \    }\n}\n\nimpl<T: FractionTrait> SubAssign for Fraction<T> {\n    fn sub_assign(&mut\
    \ self, rhs: Self) {\n        let new_numerator = self.numerator * rhs.denominator\
    \ - self.denominator * rhs.numerator;\n        let new_denominator = self.denominator\
    \ * rhs.denominator;\n        self.numerator = new_numerator;\n        self.denominator\
    \ = new_denominator;\n        self.normalize();\n    }\n}\n\nimpl<T: FractionTrait>\
    \ Sub for Fraction<T> {\n    type Output = Self;\n    fn sub(self, rhs: Self)\
    \ -> Self::Output {\n        let mut f = self;\n        f -= rhs;\n        f\n\
    \    }\n}\n\nimpl<T: FractionTrait> MulAssign for Fraction<T> {\n    fn mul_assign(&mut\
    \ self, rhs: Self) {\n        self.numerator *= rhs.numerator;\n        self.denominator\
    \ *= rhs.denominator;\n        self.normalize();\n    }\n}\n\nimpl<T: FractionTrait>\
    \ Mul for Fraction<T> {\n    type Output = Self;\n    fn mul(self, rhs: Self)\
    \ -> Self::Output {\n        let mut f = self;\n        f *= rhs;\n        f\n\
    \    }\n}\n\nimpl<T: FractionTrait> DivAssign for Fraction<T> {\n    fn div_assign(&mut\
    \ self, rhs: Self) {\n        self.numerator *= rhs.denominator;\n        self.denominator\
    \ *= rhs.numerator;\n        self.normalize();\n    }\n}\n\nimpl<T: FractionTrait>\
    \ Div for Fraction<T> {\n    type Output = Self;\n    fn div(self, rhs: Self)\
    \ -> Self::Output {\n        let mut f = self;\n        f /= rhs;\n        f\n\
    \    }\n}\n\nimpl<T: FractionTrait> Neg for Fraction<T> {\n    type Output = Self;\n\
    \    fn neg(self) -> Self::Output {\n        Self {\n            numerator: -self.numerator,\n\
    \            denominator: self.denominator,\n        }\n    }\n}\n\nimpl<T: FractionTrait>\
    \ PartialEq for Fraction<T> {\n    fn eq(&self, other: &Self) -> bool {\n    \
    \    self.numerator * other.denominator == self.denominator * other.numerator\n\
    \    }\n}\n\nimpl<T: FractionTrait> Eq for Fraction<T> {}\n\nimpl<T: FractionTrait>\
    \ Ord for Fraction<T> {\n    fn cmp(&self, other: &Self) -> std::cmp::Ordering\
    \ {\n        if self.numerator * other.denominator < self.denominator * other.numerator\
    \ {\n            Ordering::Less\n        } else if self == other {\n         \
    \   Ordering::Equal\n        } else {\n            Ordering::Greater\n       \
    \ }\n    }\n}\n\nimpl<T: FractionTrait> PartialOrd for Fraction<T> {\n    fn partial_cmp(&self,\
    \ other: &Self) -> Option<Ordering> {\n        Some(self.cmp(other))\n    }\n\
    }\n\n#[cfg(test)]\nmod tests {\n    use crate::Fraction;\n    use rand::Rng;\n\
    \n    #[test]\n    fn fraction_test() {\n        let mut rng = rand::rng();\n\n\
    \        let mut generate_random_fraction = || {\n            let numerator =\
    \ rng.random_range(-100000..100000 + 1);\n            let mut denominator = rng.random_range(-100000..100000\
    \ + 1);\n            while denominator == 0 {\n                denominator = rng.random_range(-100000..100000\
    \ + 1);\n            }\n            Fraction::new(numerator, denominator)\n  \
    \      };\n\n        for _ in 0..100 {\n            let f1 = generate_random_fraction();\n\
    \            let f2 = generate_random_fraction();\n            let mut f = f1\
    \ * f2 / f2;\n            assert!(f == f1);\n            f = f1 + f2 - f2;\n \
    \           assert!(f == f1);\n            f = f1 - f2 + f2;\n            assert!(f\
    \ == f1);\n            f = f1 / f2 * f2;\n            assert!(f == f1);\n    \
    \        f = f2 / f1 * f1;\n            assert!(f != f1);\n            f = f1\
    \ + f2 * Fraction::<i64>::from(2);\n            assert!(f != f1);\n        }\n\
    \    }\n}\n"
  dependsOn:
  - crates/traits/numeric/src/bound.rs
  - crates/traits/numeric/src/infinity.rs
  - crates/traits/numeric/src/lib.rs
  - crates/traits/numeric/src/one.rs
  - crates/traits/numeric/src/zero.rs
  isVerificationFile: false
  path: crates/misc/fraction/src/lib.rs
  requiredBy: []
  timestamp: '2025-10-25 13:03:39+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/misc/fraction/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/fraction/src/lib.rs
- /library/crates/misc/fraction/src/lib.rs.html
title: crates/misc/fraction/src/lib.rs
---

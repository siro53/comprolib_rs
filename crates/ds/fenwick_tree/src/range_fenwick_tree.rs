use std::ops::{Add, AddAssign, Bound, Mul, Neg, RangeBounds, Sub};

use numeric::zero::Zero;

use crate::fenwick_tree::FenwickTree;

pub struct RangeFenwickTree<T>
where
    T: Clone
        + From<usize>
        + AddAssign
        + Sub<Output = T>
        + Neg<Output = T>
        + Add<Output = T>
        + Mul<Output = T>
        + Neg
        + Zero,
{
    n: usize,
    fwt1: FenwickTree<T>,
    fwt2: FenwickTree<T>,
}

impl<T> RangeFenwickTree<T>
where
    T: Clone
        + From<usize>
        + AddAssign
        + Sub<Output = T>
        + Neg<Output = T>
        + Add<Output = T>
        + Mul<Output = T>
        + Neg
        + Zero,
{
    pub fn new(n: usize) -> Self {
        let fwt1 = FenwickTree::<T>::new(n + 1);
        let fwt2 = FenwickTree::<T>::new(n + 1);
        Self { n, fwt1, fwt2 }
    }

    pub fn add<R>(&mut self, range: R, value: T)
    where
        R: RangeBounds<usize>,
        T: Neg<Output = T> + Mul<Output = T> + From<usize>,
    {
        let l = match range.start_bound() {
            Bound::Included(l) => *l,
            Bound::Excluded(l) => l + 1,
            Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            Bound::Included(r) => r + 1,
            Bound::Excluded(r) => *r,
            Bound::Unbounded => self.n,
        };
        assert!(l <= r);
        self.fwt1.add(l, -value.clone() * T::from(l));
        self.fwt1.add(r, value.clone() * T::from(r));
        self.fwt2.add(l, value.clone());
        self.fwt2.add(r, -value.clone());
    }

    fn accumulate(&self, index: usize) -> T {
        assert!(index <= self.n);
        self.fwt1.sum(..index) + self.fwt2.sum(..index) * T::from(index)
    }

    pub fn sum<R>(&self, range: R) -> T
    where
        R: RangeBounds<usize>,
    {
        let l = match range.start_bound() {
            Bound::Included(l) => *l,
            Bound::Excluded(l) => l + 1,
            Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            Bound::Included(r) => r + 1,
            Bound::Excluded(r) => *r,
            Bound::Unbounded => self.n,
        };
        assert!(l <= r);
        self.accumulate(r) - self.accumulate(l)
    }
}

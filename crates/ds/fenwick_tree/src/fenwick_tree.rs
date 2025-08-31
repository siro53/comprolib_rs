use std::ops::{AddAssign, Bound, RangeBounds, Sub, SubAssign};

use numeric::zero::Zero;

pub struct FenwickTree<T>
where
    T: Clone + AddAssign + Zero + Sub<Output = T>,
{
    n: usize,
    data: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Clone + AddAssign + Zero + Sub<Output = T>,
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![T::zero(); n + 1],
        }
    }

    pub fn add(&mut self, index: usize, value: T) {
        let mut i = index + 1;
        while i <= self.n {
            self.data[i] += value.clone();
            i += i & i.wrapping_neg();
        }
    }

    fn accumulate(&self, index: usize) -> T {
        assert!(index <= self.n);
        let mut res = T::zero();
        let mut r = index;
        while r >= 1 {
            res += self.data[r].clone();
            r -= r & r.wrapping_neg();
        }
        res
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

    pub fn lower_bound(&self, value: T) -> usize
    where
        T: SubAssign + PartialOrd,
    {
        let mut pos = 0_usize;
        let mut k = 1_usize;
        let mut value = value;
        while k * 2 <= self.n {
            k <<= 1;
        }
        while k >= 1 {
            if pos + k <= self.n && self.data[pos + k] < value {
                pos += k;
                value -= self.data[pos].clone();
            }
            k >>= 1;
        }
        pos + 1
    }
}

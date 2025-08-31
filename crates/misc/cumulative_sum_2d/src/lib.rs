use std::ops::{Add, AddAssign, Bound, RangeBounds, Sub};

use numeric::zero::Zero;

pub struct CumulativeSum2D<T> {
    sum: Vec<Vec<T>>,
    is_built: bool,
}

impl<T> CumulativeSum2D<T>
where
    T: Clone + Copy,
{
    pub fn new(height: usize, width: usize) -> Self
    where
        T: Zero,
    {
        Self {
            sum: vec![vec![T::zero(); width + 1]; height + 1],
            is_built: false,
        }
    }

    pub fn add(&mut self, row: usize, column: usize, value: T)
    where
        T: AddAssign,
    {
        assert!(row + 1 < self.sum.len());
        assert!(column + 1 < self.sum[0].len());
        self.sum[row + 1][column + 1] += value;
    }

    pub fn build(&mut self)
    where
        T: AddAssign + Add<Output = T> + Sub<Output = T>,
    {
        for i in 1..self.sum.len() {
            for j in 1..self.sum[i].len() {
                self.sum[i][j] = self.sum[i][j] + self.sum[i - 1][j] + self.sum[i][j - 1]
                    - self.sum[i - 1][j - 1];
            }
        }
        self.is_built = true;
    }

    pub fn sum<R>(&self, range_row: R, range_column: R) -> T
    where
        R: RangeBounds<usize>,
        T: Add<Output = T> + Sub<Output = T>,
    {
        let row_l = match range_row.start_bound() {
            Bound::Included(l) => *l,
            Bound::Excluded(l) => l + 1,
            Bound::Unbounded => 0,
        };
        let row_r = match range_row.end_bound() {
            Bound::Included(r) => r + 1,
            Bound::Excluded(r) => *r,
            Bound::Unbounded => self.sum.len(),
        };
        let column_l = match range_column.start_bound() {
            Bound::Included(l) => *l,
            Bound::Excluded(l) => l + 1,
            Bound::Unbounded => 0,
        };
        let column_r = match range_column.end_bound() {
            Bound::Included(r) => r + 1,
            Bound::Excluded(r) => *r,
            Bound::Unbounded => self.sum[0].len(),
        };
        assert!(row_l <= row_r && row_r < self.sum.len());
        assert!(column_l <= column_r && column_r < self.sum[0].len());
        assert!(self.is_built);
        self.sum[row_r][column_r] - self.sum[row_l][column_r] - self.sum[row_r][column_l]
            + self.sum[row_l][column_l]
    }
}

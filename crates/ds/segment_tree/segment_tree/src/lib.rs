use monoid::Monoid;

pub struct SegmentTree<M>
where
    M: Monoid,
{
    n: usize,
    size: usize,
    node: Vec<M::ValueType>,
}

impl<M> SegmentTree<M>
where
    M: Monoid,
{
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        let node = vec![M::unit(); size * 2];
        Self { n, size, node }
    }

    pub fn set(&mut self, pos: usize, value: M::ValueType) {
        assert!(pos < self.n);
        let mut pos = pos + self.size;
        self.node[pos] = value;
        while pos > 1 {
            pos >>= 1;
            self.node[pos] = M::op(&self.node[pos << 1], &self.node[pos << 1 | 1]);
        }
    }

    pub fn get(&self, pos: usize) -> M::ValueType {
        assert!(pos < self.n);
        self.node[pos + self.size].clone()
    }

    pub fn apply(&mut self, pos: usize, value: M::ValueType) {
        self.set(pos, M::op(&self.get(pos), &value));
    }

    pub fn prod(&self, l: usize, r: usize) -> M::ValueType {
        assert!(l <= r && r <= self.n);
        let mut value_left = M::unit();
        let mut value_right = M::unit();
        let mut l = l + self.size;
        let mut r = r + self.size;
        while l < r {
            if (l & 1) == 1 {
                l += 1;
                value_left = M::op(&value_left, &self.node[l]);
            }
            if (r & 1) == 1 {
                value_right = M::op(&self.node[r], &value_right);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(&value_left, &value_right)
    }

    pub fn all_prod(&self) -> M::ValueType {
        self.node[1].clone()
    }

    pub fn max_right<F>(&self, l: usize, mut f: F) -> usize
    where
        F: FnMut(&M::ValueType) -> bool,
    {
        assert!(l < self.n);
        assert!(f(&M::unit()));
        if l == self.n {
            return self.n;
        }
        let mut l = l + self.size;
        let mut value = M::unit();
        while {
            while (l & 1) == 0 {
                l >>= 1;
            }
            if !f(&M::op(&value, &self.node[l])) {
                while l < self.size {
                    l *= 2;
                    if f(&M::op(&value, &self.node[l])) {
                        value = M::op(&value, &self.node[l]);
                        l += 1;
                    }
                }
                return l - self.size;
            }
            value = M::op(&value, &self.node[l]);
            l += 1;
            (l & l.wrapping_neg()) != l
        } {}
        self.n
    }

    pub fn min_left<F>(&self, r: usize, mut f: F) -> usize
    where
        F: FnMut(&M::ValueType) -> bool,
    {
        assert!(r <= self.n);
        assert!(f(&M::unit()));
        if r == 0 {
            return 0;
        }
        let mut r = r + self.size;
        let mut value = M::unit();
        while {
            r -= 1;
            while r > 1 && (r & 1) == 1 {
                r >>= 1;
            }
            if !f(&M::op(&self.node[r], &value)) {
                while r < self.size {
                    r = r * 2 + 1;
                    if f(&M::op(&self.node[r], &value)) {
                        value = M::op(&self.node[r], &value);
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            value = M::op(&self.node[r], &value);
            (r & r.wrapping_neg()) != r
        } {}
        0
    }
}

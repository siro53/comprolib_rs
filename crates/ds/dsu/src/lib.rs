use std::collections::{BTreeMap, btree_map};
use std::mem::swap;

pub struct Dsu {
    size: usize,
    parent_or_size: Vec<i32>,
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            parent_or_size: vec![-1; size],
        }
    }

    pub fn leader(&mut self, u: usize) -> usize {
        assert!(u < self.size);
        if self.parent_or_size[u] < 0 {
            return u;
        }
        self.parent_or_size[u] = self.leader(self.parent_or_size[u] as usize) as i32;
        self.parent_or_size[u] as usize
    }

    pub fn same(&mut self, u: usize, v: usize) -> bool {
        assert!(u < self.size);
        assert!(v < self.size);
        self.leader(u) == self.leader(v)
    }

    pub fn merge(&mut self, u: usize, v: usize) -> usize {
        assert!(u < self.size);
        assert!(v < self.size);
        let mut lu = self.leader(u);
        let mut lv = self.leader(v);
        if lu == lv {
            return lu;
        }
        if -self.parent_or_size[lu] < -self.parent_or_size[lv] {
            swap(&mut lu, &mut lv);
        }
        self.parent_or_size[lu] += self.parent_or_size[lv];
        self.parent_or_size[lv] = lu as i32;
        lu
    }

    pub fn size(&mut self, u: usize) -> usize {
        let lu = self.leader(u);
        -self.parent_or_size[lu] as usize
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leaders: Vec<usize> = vec![0; self.size];
        let mut group_size: Vec<usize> = vec![0; self.size];
        for i in 0..self.size {
            leaders[i] = self.leader(i);
            group_size[i] += 1;
        }
        let mut groups: Vec<Vec<usize>> = vec![Vec::new(); self.size];
        for i in 0..self.size {
            groups[i].reserve(group_size[i]);
        }
        for i in 0..self.size {
            groups[leaders[i]].push(i);
        }
        groups
            .into_iter()
            .filter(|group| !group.is_empty())
            .collect()
    }

    pub fn groups_with_btreemap(&mut self) -> BTreeMap<usize, Vec<usize>> {
        let mut groups: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for i in 0..self.size {
            let l = self.leader(i);
            if let btree_map::Entry::Vacant(e) = groups.entry(l) {
                e.insert(vec![i]);
            } else {
                let group = groups.get_mut(&l).unwrap();
                group.push(i);
            }
        }
        groups
    }
}

#[cfg(test)]
mod tests {
    use crate::Dsu;

    #[test]
    fn dsu_test() {
        let mut d = Dsu::new(4);
        d.merge(0, 1);
        assert!(d.same(0, 1));
        d.merge(1, 2);
        assert!(d.same(0, 2));
        assert_eq!(d.size(0), 3);
        assert!(!d.same(0, 3));
        assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3]]);
        assert_eq!(*d.groups_with_btreemap().get(&0).unwrap(), vec![0, 1, 2]);
        assert_eq!(*d.groups_with_btreemap().get(&3).unwrap(), vec![3]);
    }
}

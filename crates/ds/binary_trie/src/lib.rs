use std::{
    ops::{AddAssign, BitAnd, BitXorAssign, Shl, Shr},
    usize,
};

use numeric::{one::One, zero::Zero};

#[derive(Clone, Default)]
pub struct Node {
    count: usize,
    children_node: [Option<Box<Node>>; 2],
}

impl Node {
    pub fn new() -> Self {
        Self::default()
    }
}

mod private {
    pub trait Sealed {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}

pub trait BinaryTrieTrait:
    private::Sealed
    + Zero
    + One
    + Copy
    + PartialEq
    + Shr<usize, Output = Self>
    + Shl<usize, Output = Self>
    + BitAnd<Output = Self>
    + AddAssign
    + BitXorAssign
{
    const BIT_LENGTH: usize;

    fn get_dir<T: BinaryTrieTrait>(value: T, i: usize) -> usize {
        if (value >> i & T::one()) == T::one() {
            1
        } else {
            0
        }
    }
}

impl BinaryTrieTrait for u32 {
    const BIT_LENGTH: usize = 32;
}

impl BinaryTrieTrait for u64 {
    const BIT_LENGTH: usize = 64;
}

pub struct BinaryTrie<T: BinaryTrieTrait> {
    root: Option<Box<Node>>,
    all_xor_value: T,
}

impl<T: BinaryTrieTrait> Default for BinaryTrie<T> {
    fn default() -> Self {
        Self {
            root: None,
            all_xor_value: T::zero(),
        }
    }
}

impl<T: BinaryTrieTrait> BinaryTrie<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(&self, value: T) -> usize {
        if self.root.is_none() {
            return 0;
        }
        let mut now = self.root.as_ref();
        for i in (0..T::BIT_LENGTH).rev() {
            let dir = T::get_dir(value, i);
            now = now.unwrap().children_node[dir].as_ref();
            if now.is_none() || now.unwrap().count == 0 {
                return 0;
            }
        }
        now.unwrap().count
    }

    pub fn insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new()));
        }
        let mut now = self.root.as_mut().unwrap();
        now.count += 1;
        for i in (0..T::BIT_LENGTH).rev() {
            let dir: usize = T::get_dir(value, i);
            let nxt = &mut now.children_node[dir];
            if nxt.is_none() {
                *nxt = Some(Box::new(Node::new()));
            }
            now = nxt.as_mut().unwrap();
            now.count += 1;
        }
    }

    pub fn erase(&mut self, value: T) -> bool {
        if self.count(value) == 0 {
            return false;
        }
        let mut now = self.root.as_mut().unwrap();
        for i in (0..T::BIT_LENGTH).rev() {
            now.count -= 1;
            let dir = T::get_dir(value, i);
            now = now.children_node[dir].as_mut().unwrap();
            assert!(now.count > 0);
        }
        now.count -= 1;
        true
    }

    pub fn min_element(&self) -> T {
        assert!(self.root.is_some());
        let mut res = T::zero();
        let mut now = self.root.as_ref().unwrap();
        for i in (0..T::BIT_LENGTH).rev() {
            let dir = T::get_dir(self.all_xor_value, i);
            let mut nxt = now.children_node[dir].as_ref();
            if nxt.is_none() || nxt.unwrap().count == 0 {
                res += T::one() << i;
                nxt = now.children_node[dir ^ 1].as_ref();
            }
            now = nxt.unwrap();
        }
        res
    }

    pub fn max_element(&self) -> T {
        assert!(self.root.is_some());
        let mut res = T::zero();
        let mut now = self.root.as_ref().unwrap();
        for i in (0..T::BIT_LENGTH).rev() {
            let dir = T::get_dir(self.all_xor_value, i) ^ 1;
            let mut nxt = now.children_node[dir].as_ref();
            if nxt.is_none() || nxt.unwrap().count == 0 {
                nxt = now.children_node[dir ^ 1].as_ref();
            } else {
                res += T::one() << i;
            }
            now = nxt.unwrap();
        }
        res
    }

    pub fn get_kth_min_element(&self, k: usize) -> T {
        assert!(k < self.size());
        let mut now = self.root.as_ref().unwrap();
        let mut res = T::zero();
        let mut k = k;
        for i in (0..T::BIT_LENGTH).rev() {
            let dir = T::get_dir(self.all_xor_value, i);
            let small_node = now.children_node[dir].as_ref();
            let big_node = now.children_node[dir ^ 1].as_ref();
            let small_count = if let Some(n) = small_node { n.count } else { 0 };
            let big_count = if let Some(n) = big_node { n.count } else { 0 };
            assert!(0 < small_count + big_count);
            assert!(k < small_count + big_count);
            if k >= small_count {
                res += T::one() << i;
                now = big_node.unwrap();
                k -= small_count;
            } else {
                now = small_node.unwrap();
            }
        }
        res
    }

    pub fn lower_bound(&self, value: T) -> usize {
        assert!(self.root.is_some());
        let mut res: usize = 0;
        let mut now = self.root.as_ref().unwrap();
        for i in (0..T::BIT_LENGTH).rev() {
            let dir = T::get_dir(self.all_xor_value, i);
            let small_node = now.children_node[dir].as_ref();
            let big_node = now.children_node[dir ^ 1].as_ref();
            let small_count = if let Some(n) = small_node { n.count } else { 0 };
            let big_count = if let Some(n) = big_node { n.count } else { 0 };
            if small_count == 0 {
                assert!(big_count > 0);
                now = big_node.unwrap();
            } else if big_count == 0 {
                assert!(small_count > 0);
                now = small_node.unwrap();
            } else if dir == T::get_dir(value, i) {
                now = small_node.unwrap();
            } else {
                res += small_count;
                now = big_node.unwrap();
            }
        }
        res
    }

    pub fn upper_bound(&self, value: T) -> usize {
        let idx = self.lower_bound(value);
        let kth_min_element = self.get_kth_min_element(idx);
        if kth_min_element != value {
            return idx;
        }
        let count = self.count(kth_min_element);
        idx + count
    }

    pub fn size(&self) -> usize {
        if self.root.is_none() {
            0
        } else {
            self.root.as_ref().unwrap().count
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none() || self.root.as_ref().unwrap().count == 0
    }

    pub fn all_xor(&mut self, value: T) {
        self.all_xor_value ^= value;
    }
}

#[cfg(test)]
mod tests {
    use crate::BinaryTrie;
    use rand::Rng;
    use superslice::Ext;

    #[test]
    fn binary_trie_test() {
        const TESTCASE_NUM: usize = 50;
        const QUERY_NUM: usize = 10000;
        for testcase_id in 0..TESTCASE_NUM {
            let mut a = Vec::<u32>::new();
            let mut bt = BinaryTrie::<u32>::new();
            let mut rng = rand::rng();
            for _ in 0..QUERY_NUM {
                let t: usize = if a.is_empty() {
                    0
                } else {
                    rng.random_range(0..7)
                };
                match t {
                    0 => {
                        // insert
                        let x: u32 = rng.random();
                        bt.insert(x);
                        a.push(x);
                    }
                    1 => {
                        // erase
                        let a_len = a.len();
                        let idx = rng.random_range(0..a_len);
                        a.swap(idx, a_len - 1);
                        bt.erase(*a.last().unwrap());
                        a.pop();
                    }
                    2 => {
                        // min_element
                        assert!(*a.iter().reduce(std::cmp::min).unwrap() == bt.min_element());
                    }
                    3 => {
                        // max_element
                        assert!(*a.iter().reduce(std::cmp::max).unwrap() == bt.max_element());
                    }
                    4 => {
                        // kth_element
                        a.sort();
                        let a_len = a.len();
                        let k = rng.random_range(0..a_len);
                        assert!(a[k] == bt.get_kth_min_element(k));
                    }
                    5 => {
                        // lower_bound
                        a.sort();
                        let a_len = a.len();
                        let k = rng.random_range(0..a_len);
                        let index1 = bt.lower_bound(a[k]);
                        let index2 = a.lower_bound(&a[k]);
                        assert!(index1 == index2);
                    }
                    6 => {}
                    _ => unreachable!(),
                }
            }
            eprintln!("passed Test {} / {}", testcase_id + 1, TESTCASE_NUM);
        }
    }
}

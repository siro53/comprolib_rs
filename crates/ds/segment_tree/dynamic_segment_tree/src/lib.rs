use std::ops::{Bound, RangeBounds};

use monoid::Monoid;

pub struct Node<M>
where
    M: Monoid,
{
    value: M::ValueType,
    children_node: [Option<Box<Node<M>>>; 2],
}

impl<M> Node<M>
where
    M: Monoid,
{
    pub fn new(value: M::ValueType) -> Self {
        Self {
            value,
            children_node: [None, None],
        }
    }
}

pub struct DynamicSegmentTree<M, const MIN_INDEX: i64, const MAX_INDEX: i64>
where
    M: Monoid,
{
    root: Option<Box<Node<M>>>,
}

impl<M, const MIN_INDEX: i64, const MAX_INDEX: i64> Default
    for DynamicSegmentTree<M, MIN_INDEX, MAX_INDEX>
where
    M: Monoid,
{
    fn default() -> Self {
        Self { root: None }
    }
}

impl<M, const MIN_INDEX: i64, const MAX_INDEX: i64> DynamicSegmentTree<M, MIN_INDEX, MAX_INDEX>
where
    M: Monoid,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, pos: i64, value: M::ValueType) {
        assert!(MIN_INDEX <= pos);
        assert!(pos < MAX_INDEX);
        Self::_set(&mut self.root, MIN_INDEX, MAX_INDEX, pos, value);
    }

    pub fn get(&self, pos: i64) -> M::ValueType {
        assert!(MIN_INDEX <= pos);
        assert!(pos < MAX_INDEX);
        Self::_get(&self.root, MIN_INDEX, MAX_INDEX, pos)
    }

    pub fn prod<R: RangeBounds<i64>>(&self, range: R) -> M::ValueType {
        let l = match range.start_bound() {
            Bound::Included(l) => *l,
            Bound::Excluded(l) => l + 1,
            Bound::Unbounded => MIN_INDEX,
        };
        let r = match range.end_bound() {
            Bound::Included(r) => r + 1,
            Bound::Excluded(r) => *r,
            Bound::Unbounded => MAX_INDEX,
        };
        assert!(MIN_INDEX <= l);
        assert!(l <= r);
        assert!(r <= MAX_INDEX);
        Self::_prod(&self.root, MIN_INDEX, MAX_INDEX, l, r)
    }

    fn _set(node: &mut Option<Box<Node<M>>>, l: i64, r: i64, pos: i64, value: M::ValueType) {
        if node.is_none() {
            *node = Some(Box::new(Node::<M>::new(M::unit())));
        }
        let ptr = node.as_mut().unwrap();
        if r - l == 1 {
            ptr.value = value;
            return;
        }
        let mid = (l + r) / 2;
        if pos < mid {
            Self::_set(&mut ptr.children_node[0], l, mid, pos, value);
        } else {
            Self::_set(&mut ptr.children_node[1], mid, r, pos, value);
        }
        let left_value = if ptr.children_node[0].is_none() {
            M::unit()
        } else {
            ptr.children_node[0].as_mut().unwrap().value.clone()
        };
        let right_value = if ptr.children_node[1].is_none() {
            M::unit()
        } else {
            ptr.children_node[1].as_mut().unwrap().value.clone()
        };
        ptr.value = M::op(&left_value, &right_value);
    }

    fn _get(node: &Option<Box<Node<M>>>, l: i64, r: i64, pos: i64) -> M::ValueType {
        if node.is_none() {
            return M::unit();
        }
        let ptr = node.as_ref().unwrap();
        if r - l == 1 {
            return ptr.value.clone();
        }
        let mid = (l + r) / 2;
        if pos < mid {
            Self::_get(&ptr.children_node[0], l, mid, pos)
        } else {
            Self::_get(&ptr.children_node[1], mid, r, pos)
        }
    }

    fn _prod(node: &Option<Box<Node<M>>>, l: i64, r: i64, ql: i64, qr: i64) -> M::ValueType {
        if node.is_none() || (qr <= l || r <= ql) {
            return M::unit();
        }
        let ptr = node.as_ref().unwrap();
        if ql <= l && r <= qr {
            return ptr.value.clone();
        }
        let mid = (l + r) / 2;
        M::op(
            &Self::_prod(&ptr.children_node[0], l, mid, ql, qr),
            &Self::_prod(&ptr.children_node[1], mid, r, ql, qr),
        )
    }
}

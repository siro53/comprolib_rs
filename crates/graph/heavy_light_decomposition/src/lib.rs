use std::{mem::swap, ops::Index};

use graph::graph::Tree;

pub struct HeavyLightDecomposition<Cost>
where
    Cost: Clone,
{
    tree: Tree<Cost>,
    input: Vec<usize>,
    output: Vec<usize>,
    size: Vec<usize>,
    head: Vec<usize>,
    parent: Vec<Option<usize>>,
    depth: Vec<usize>,
    rev_input: Vec<usize>,
    is_built: bool,
}

impl<Cost> HeavyLightDecomposition<Cost>
where
    Cost: Clone + Copy,
{
    fn _dfs1(&mut self, u: usize, p: Option<usize>, d: usize) {
        self.depth[u] = d;
        self.size[u] = 1;
        if !self.tree[u].is_empty() && Some(self.tree[u].first().unwrap().to()) == p {
            let sz = self.tree[u].len();
            self.tree[u].swap(0, sz - 1);
        }
        for i in 0..self.tree[u].len() {
            let v = self.tree[u][i].to();
            if Some(v) == p {
                continue;
            }
            self._dfs1(v, Some(u), d + 1);
            self.size[u] += self.size[v];
            let h = self.tree[u][0].to();
            if self.size[v] > self.size[h] {
                self.tree[u].swap(0, i);
            }
        }
    }

    fn _dfs2(&mut self, u: usize, p: Option<usize>, index: &mut usize) {
        self.parent[u] = p;
        self.input[u] = *index;
        *index += 1;
        self.rev_input[self.input[u]] = u;
        for i in 0..self.tree[u].len() {
            let v = self.tree[u][i].to();
            if Some(v) == p {
                continue;
            }
            let h = self.tree[u][0].to();
            self.head[v] = if v == h { self.head[u] } else { v };
            self._dfs2(v, Some(u), index);
        }
        self.output[u] = *index;
    }

    pub fn new(tree: &Tree<Cost>) -> Self {
        Self {
            tree: tree.clone(),
            input: vec![0; tree.size()],
            output: vec![0; tree.size()],
            size: vec![0; tree.size()],
            head: vec![0; tree.size()],
            parent: vec![None; tree.size()],
            depth: vec![0; tree.size()],
            rev_input: vec![0; tree.size()],
            is_built: false,
        }
    }

    pub fn build(&mut self, root: usize) {
        self._dfs1(root, None, 0);
        self.head[root] = root;
        let mut index = 0;
        self._dfs2(root, None, &mut index);
        self.is_built = true;
    }

    pub fn lca(&self, u: usize, v: usize) -> usize {
        assert!(self.is_built);
        assert!(u < self.tree.size());
        assert!(v < self.tree.size());
        let mut u = u;
        let mut v = v;
        loop {
            if self.input[u] > self.input[v] {
                swap(&mut u, &mut v);
            }
            if self.head[u] == self.head[v] {
                return u;
            }
            v = self.parent[self.head[v]].unwrap();
        }
    }

    pub fn dist(&self, u: usize, v: usize) -> usize {
        assert!(self.is_built);
        assert!(u < self.tree.size());
        assert!(v < self.tree.size());
        self.depth[u] + self.depth[v] - self.depth[self.lca(u, v)] * 2
    }

    pub fn la(&self, u: usize, k: usize) -> Option<usize> {
        assert!(self.is_built);
        assert!(u < self.tree.size());
        if self.depth[u] < k {
            return None;
        }
        let mut u = u;
        let mut k = k;
        loop {
            if self.input[u] - k >= self.input[self.head[u]] {
                return Some(self.rev_input[self.input[u] - k]);
            }
            k -= self.input[u] - self.input[self.head[u]] + 1;
            u = self.parent[self.head[u]].unwrap();
        }
    }

    fn _path_query<F>(&self, u: usize, v: usize, f: &mut F, is_edge: bool)
    where
        F: FnMut(usize, usize),
    {
        assert!(self.is_built);
        assert!(u < self.tree.size());
        assert!(v < self.tree.size());
        let mut u = u;
        let mut v = v;
        loop {
            if self.input[u] > self.input[v] {
                swap(&mut u, &mut v);
            }
            if self.head[u] == self.head[v] {
                f(self.input[u] + (is_edge as usize), self.input[v] + 1);
                break;
            } else {
                f(self.input[self.head[v]], self.input[v] + 1);
            }
            v = self.parent[self.head[v]].unwrap();
        }
    }

    pub fn path_query_commutative<F>(&self, u: usize, v: usize, f: &mut F, is_edge: bool)
    where
        F: FnMut(usize, usize),
    {
        assert!(self.is_built);
        assert!(u < self.tree.size());
        assert!(v < self.tree.size());
        self._path_query(u, v, f, is_edge);
    }

    pub fn path_query_not_commutative<F, InverseF>(
        &self,
        u: usize,
        v: usize,
        f: &mut F,
        inv_f: &mut InverseF,
        is_edge: bool,
    ) where
        F: FnMut(usize, usize),
        InverseF: FnMut(usize, usize),
    {
        assert!(self.is_built);
        assert!(u < self.tree.size());
        assert!(v < self.tree.size());
        let lca = self.lca(u, v);
        self._path_query(u, lca, f, is_edge);
        self._path_query(lca, v, inv_f, true);
    }

    pub fn subtree_query<F>(&self, u: usize, f: &mut F)
    where
        F: FnMut(usize, usize),
    {
        assert!(self.is_built);
        assert!(u < self.tree.size());
        f(self.input[u], self.output[u]);
    }
}

impl<Cost> Index<usize> for HeavyLightDecomposition<Cost>
where
    Cost: Clone + Copy,
{
    type Output = usize;

    fn index(&self, u: usize) -> &Self::Output {
        assert!(self.is_built);
        assert!(u < self.tree.size());
        &self.input[u]
    }
}

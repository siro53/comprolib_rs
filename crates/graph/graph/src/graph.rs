use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use crate::edge::Edge;

pub struct GraphBase<Cost: Clone, Marker> {
    n: usize,
    m: usize,
    g: Vec<Vec<Edge<Cost>>>,
    _marker: PhantomData<fn() -> Marker>,
}

impl<Cost, Marker> GraphBase<Cost, Marker>
where
    Cost: Clone + Copy,
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            m: 0,
            g: vec![Vec::<Edge<Cost>>::new(); n],
            _marker: PhantomData,
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }
}

impl<Cost, Marker> Index<usize> for GraphBase<Cost, Marker>
where
    Cost: Clone,
{
    type Output = Vec<Edge<Cost>>;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.n);
        &self.g[index]
    }
}

impl<Cost, Marker> IndexMut<usize> for GraphBase<Cost, Marker>
where
    Cost: Clone,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.n);
        &mut self.g[index]
    }
}

pub enum UndirectedGraphMarker {}
pub enum DirectedGraphMarker {}
pub enum TreeMarker {}

pub type UndirectedGraph<Cost> = GraphBase<Cost, UndirectedGraphMarker>;
pub type DirectedGraph<Cost> = GraphBase<Cost, DirectedGraphMarker>;
pub type Tree<Cost> = GraphBase<Cost, TreeMarker>;

impl<Cost> DirectedGraph<Cost>
where
    Cost: Clone + Copy,
{
    pub fn add_directed_edge(&mut self, from: usize, to: usize, cost: Cost) {
        self.g[from].push(Edge::new(from, to, cost, self.m));
        self.m += 1;
    }
}

impl<Cost> UndirectedGraph<Cost>
where
    Cost: Clone + Copy,
{
    pub fn add_undirected_edge(&mut self, from: usize, to: usize, cost: Cost) {
        self.g[from].push(Edge::new(from, to, cost, self.m));
        self.m += 1;
        self.g[to].push(Edge::new(to, from, cost, self.m));
        self.m += 1;
    }
}

impl<Cost> Tree<Cost>
where
    Cost: Clone + Copy,
{
    pub fn add_edge(&mut self, from: usize, to: usize, cost: Cost) {
        self.g[from].push(Edge::new(from, to, cost, self.m));
        self.m += 1;
        self.g[to].push(Edge::new(to, from, cost, self.m));
        self.m += 1;
    }
}

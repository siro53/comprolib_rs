use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    ops::Add,
};

use numeric::one::One;

use crate::graph::GraphBase;

impl<Cost, Marker> GraphBase<Cost, Marker>
where
    Cost: Clone + Copy,
{
    pub fn bfs(&self, start: usize, iv: Cost, inf: Cost) -> (Vec<Cost>, Vec<Option<usize>>)
    where
        Cost: One + Add<Output = Cost> + PartialOrd,
    {
        let mut dist = vec![inf; self.size()];
        let mut prev: Vec<Option<usize>> = vec![None; self.size()];
        let mut que = VecDeque::<usize>::new();
        que.push_back(start);
        dist[start] = iv;
        while let Some(u) = que.pop_front() {
            self[u].iter().for_each(|e| {
                if dist[e.to()] > dist[u] + Cost::one() {
                    dist[e.to()] = dist[u] + Cost::one();
                    prev[e.to()] = Some(u);
                    que.push_back(e.to());
                }
            });
        }
        (dist, prev)
    }

    pub fn dijkstra(&self, start: usize, iv: Cost, inf: Cost) -> (Vec<Cost>, Vec<Option<usize>>)
    where
        Cost: Add<Output = Cost> + PartialOrd + Ord,
    {
        let mut dist = vec![inf; self.size()];
        let mut prev: Vec<Option<usize>> = vec![None; self.size()];
        let mut que = BinaryHeap::new();
        que.push(Reverse((iv, start)));
        dist[start] = iv;
        while let Some(Reverse((d, u))) = que.pop() {
            if d > dist[u] {
                continue;
            }
            self[u].iter().for_each(|e| {
                if dist[e.to()] > dist[u] + e.cost() {
                    dist[e.to()] = dist[u] + e.cost();
                    prev[e.to()] = Some(u);
                    que.push(Reverse((dist[e.to()], e.to())));
                }
            });
        }
        (dist, prev)
    }
}

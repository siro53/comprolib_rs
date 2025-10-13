use std::ops::Add;

use numeric::zero::Zero;

use crate::graph::Tree;

impl<Cost> Tree<Cost>
where
    Cost: Clone + Copy + Zero + PartialOrd + Add<Output = Cost>,
{
    fn _diameter_dfs(
        &self,
        u: usize,
        p: Option<usize>,
        d: Cost,
        depth: &mut Vec<Cost>,
        parent: &mut Vec<Option<usize>>,
    ) {
        depth[u] = d;
        parent[u] = p;
        self[u].iter().for_each(|e| {
            if p.is_some() && e.to() == p.unwrap() {
                return;
            }
            self._diameter_dfs(e.to(), Some(u), d + e.cost(), depth, parent);
        });
    }

    pub fn diameter(&self) -> (Cost, Vec<usize>) {
        let mut depth: Vec<Cost> = vec![Cost::zero(); self.size()];
        let mut parent: Vec<Option<usize>> = vec![None; self.size()];
        self._diameter_dfs(0, None, Cost::zero(), &mut depth, &mut parent);
        let from = depth
            .iter()
            .enumerate()
            .reduce(|acc, elem| {
                if acc.1 < elem.1 {
                    return elem;
                }
                acc
            })
            .unwrap()
            .0;
        self._diameter_dfs(from, None, Cost::zero(), &mut depth, &mut parent);
        let to = depth
            .iter()
            .enumerate()
            .reduce(|acc, elem| {
                if acc.1 < elem.1 {
                    return elem;
                }
                acc
            })
            .unwrap()
            .0;
        let mut path = vec![to];
        loop {
            let nxt = parent[*path.last().unwrap()];
            if nxt.is_none() {
                break;
            }
            path.push(nxt.unwrap());
        }
        (depth[to], path)
    }
}

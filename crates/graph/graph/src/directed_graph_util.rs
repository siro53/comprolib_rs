use crate::graph::DirectedGraph;

impl<Cost> DirectedGraph<Cost>
where
    Cost: Clone + Copy,
{
    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        let n = self.size();
        let mut deg = vec![0_usize; n];
        for u in 0..n {
            self[u].iter().for_each(|e| {
                deg[e.to()] += 1;
            });
        }
        let mut stack = Vec::<usize>::new();
        let mut ret = Vec::<usize>::new();
        deg.iter().enumerate().for_each(|(i, &d)| {
            if d == 0 {
                stack.push(i);
            }
        });
        while let Some(u) = stack.pop() {
            ret.push(u);
            self[u].iter().for_each(|e| {
                let v = e.to();
                deg[v] -= 1;
                if deg[v] == 0 {
                    stack.push(v);
                }
            });
        }

        if ret.len() == n { Some(ret) } else { None }
    }
}

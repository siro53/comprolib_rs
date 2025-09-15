pub trait Mo {
    type Output: Default;

    fn add(&mut self, _idx: usize) {
        unimplemented!();
    }

    fn del(&mut self, _idx: usize) {
        unimplemented!();
    }

    fn add_left(&mut self, idx: usize) {
        self.add(idx);
    }

    fn add_right(&mut self, idx: usize) {
        self.add(idx);
    }

    fn del_left(&mut self, idx: usize) {
        self.del(idx);
    }

    fn del_right(&mut self, idx: usize) {
        self.del(idx);
    }

    fn query(&self) -> Self::Output;

    fn run(&mut self, n: usize, queries: &[(usize, usize)]) -> Vec<Self::Output> {
        let q = queries.len();
        let bucket_size = (((n as f64) / (q as f64).sqrt()) as usize).max(1);

        let mut ord = (0..q).collect::<Vec<usize>>();
        ord.sort_by(|&a, &b| {
            if queries[a].0 / bucket_size != queries[b].0 / bucket_size {
                return queries[a].0.cmp(&queries[b].0);
            }
            queries[a].1.cmp(&queries[b].1)
        });

        let mut l: usize = 0;
        let mut r: usize = 0;
        let mut res = Vec::new();
        res.resize_with(q, Self::Output::default);
        ord.into_iter().for_each(|qid| {
            while l > queries[qid].0 {
                l -= 1;
                self.add_left(l);
            }
            while l < queries[qid].0 {
                self.del_left(l);
                l += 1;
            }
            while r < queries[qid].1 {
                self.add_right(r);
                r += 1;
            }
            while r > queries[qid].1 {
                r -= 1;
                self.del_right(r);
            }
            res[qid] = self.query();
        });

        res
    }
}

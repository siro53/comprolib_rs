#[derive(Clone)]
pub struct Edge<Cost: Clone> {
    from: usize,
    to: usize,
    cost: Cost,
    id: usize,
}

impl<Cost: Clone + Copy> Edge<Cost> {
    pub fn new(from: usize, to: usize, cost: Cost, id: usize) -> Self {
        Self { from, to, cost, id }
    }

    pub fn to(&self) -> usize {
        self.to
    }

    pub fn cost(&self) -> Cost {
        self.cost
    }
}

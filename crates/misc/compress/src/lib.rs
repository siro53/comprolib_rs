use std::ops::Index;

use superslice::Ext;

#[derive(Default)]
pub struct Compress<T> {
    data: Vec<T>,
    is_built: bool,
}

impl<T> Compress<T>
where
    T: Clone + PartialOrd + Ord,
{
    pub fn new() -> Self {
        Self {
            data: Vec::<T>::new(),
            is_built: false,
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.is_built = false;
    }

    pub fn build(&mut self) {
        self.data.sort();
        self.data.dedup();
        self.is_built = true;
    }

    pub fn get(&self, value: T) -> usize {
        assert!(self.is_built);
        self.data.lower_bound(&value)
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T> Index<usize> for Compress<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(self.is_built);
        &self.data[index]
    }
}

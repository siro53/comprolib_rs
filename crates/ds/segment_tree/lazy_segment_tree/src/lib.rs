use act::Act;
use monoid::Monoid;

pub struct LazySegmentTree<A: Act> {
    n: usize,
    size: usize,
    node: Vec<<A::S as Monoid>::ValueType>,
    lazy: Vec<<A::F as Monoid>::ValueType>,
}

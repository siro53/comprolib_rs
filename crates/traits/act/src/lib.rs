use monoid::Monoid;

pub trait Act {
    type S: Monoid;
    type F: Monoid;
    fn act(
        x: &<Self::S as Monoid>::ValueType,
        f: &<Self::F as Monoid>::ValueType,
    ) -> <Self::S as Monoid>::ValueType;
}

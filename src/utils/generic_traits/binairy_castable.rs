use crate::numeric_types;


pub trait BinairyCastable {}

macro_rules! binary_converable_numeric_impl {
    ($($t:ty)*) => ($(
        impl BinairyCastable for $t {}
    )*);
}
numeric_types!(binary_converable_numeric_impl);
binary_converable_numeric_impl!(bool);
binary_converable_numeric_impl!(char);

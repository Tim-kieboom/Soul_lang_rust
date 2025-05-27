use core::fmt;
use std::ops::AddAssign;
use num_traits::{Bounded, Num};

use super::binairy_castable::BinairyCastable;
#[macro_export]
macro_rules! numeric_types {
    ($macro:ident) => {
        $macro!(i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64);
    };
}

/// base trait for numeric types, 
/// this is a trait derived from num_traits::Num with some added basic trait aften used for numeric types
/// (you also have to_f64() to cast number to f64 if not impl trait is needed)
#[allow(dead_code)]
pub trait Number<T: PartialOrd>:
    Num + 
    Copy + 
    PartialEq +
    AddAssign + 
    PartialOrd + 
    fmt::Debug + 
    Bounded +
    Tof64 + 
    MinMax<T> + 
    BinairyCastable +
    Sized
{
}
macro_rules! number_impl {
    ($($t:ty)*) => ($(
        impl Number<$t> for $t {}
    )*);
}
numeric_types!(number_impl);

#[allow(dead_code)]
pub trait Tof64 {
    fn to_f64(&self) -> f64;
}


macro_rules! Tof64_impl {
    ($($t:ty)*) => ($(
        impl Tof64 for $t {
            fn to_f64(&self) -> f64 {
                self.clone() as f64 
            }
        }
    )*);
}
numeric_types!(Tof64_impl);

#[allow(dead_code)]
pub trait MinMax<T: PartialOrd> {
    fn min(&self, other: T) -> T;
    fn max(&self, other: T) -> T;
}
macro_rules! minmax_impl {
    ($($t:ty)*) => ($(
        impl MinMax<$t> for $t {
            fn min(&self, other: $t) -> $t {
                if *self < other {
                    self.clone()
                } else {
                    other
                }
            }

            fn max(&self, other: $t) -> $t {
                if *self > other {
                    self.clone()
                } else {
                    other
                }
            }
        }
    )*);
}
numeric_types!(minmax_impl);

#[allow(dead_code)]
trait ConstSizeOf {
    fn size_of() -> usize;
}
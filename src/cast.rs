use super::Complex;
use traits::{Num, ToPrimitive};

macro_rules! impl_to_primitive { ($ty:ty, $to:ident) => {
#[inline]
fn $to(&self) -> Option<$ty> {
    if self.im == T::zero() { self.re.$to() } else { None }
}
}} // impl_to_primitive

// Returns None if Complex part is non-zero
impl<T: ToPrimitive + Num> ToPrimitive for Complex<T> {
    impl_to_primitive!(usize, to_usize);
    impl_to_primitive!(isize, to_isize);
    impl_to_primitive!(u8, to_u8);
    impl_to_primitive!(u16, to_u16);
    impl_to_primitive!(u32, to_u32);
    impl_to_primitive!(u64, to_u64);
    impl_to_primitive!(i8, to_i8);
    impl_to_primitive!(i16, to_i16);
    impl_to_primitive!(i32, to_i32);
    impl_to_primitive!(i64, to_i64);
    #[cfg(has_i128)]
    impl_to_primitive!(u128, to_u128);
    #[cfg(has_i128)]
    impl_to_primitive!(i128, to_i128);
    impl_to_primitive!(f32, to_f32);
    impl_to_primitive!(f64, to_f64);
}

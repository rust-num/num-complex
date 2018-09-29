use super::Complex;
use traits::{FromPrimitive, Num, ToPrimitive, Zero};

macro_rules! impl_to_primitive {
    ($ty:ty, $to:ident) => {
        #[inline]
        fn $to(&self) -> Option<$ty> {
            if self.im == T::zero() { self.re.$to() } else { None }
        }
    }
} // impl_to_primitive

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

macro_rules! impl_from_primitive {
    ($ty:ty, $from_xx:ident) => {
        #[inline]
        fn $from_xx(n: $ty) -> Option<Self> {
            Some(Self {
                re: T::$from_xx(n)?,
                im: T::zero(),
            })
        }
    };
} // impl_from_primitive

impl<T: FromPrimitive + Zero> FromPrimitive for Complex<T> {
    impl_from_primitive!(usize, from_usize);
    impl_from_primitive!(isize, from_isize);
    impl_from_primitive!(u8, from_u8);
    impl_from_primitive!(u16, from_u16);
    impl_from_primitive!(u32, from_u32);
    impl_from_primitive!(u64, from_u64);
    impl_from_primitive!(i8, from_i8);
    impl_from_primitive!(i16, from_i16);
    impl_from_primitive!(i32, from_i32);
    impl_from_primitive!(i64, from_i64);
    #[cfg(has_i128)]
    impl_from_primitive!(u128, from_u128);
    #[cfg(has_i128)]
    impl_from_primitive!(i128, from_i128);
    impl_from_primitive!(f32, from_f32);
    impl_from_primitive!(f64, from_f64);
}

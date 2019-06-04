use super::Complex;

use std::ops::Neg;
use traits::{Float, Num, One, Pow};

macro_rules! pow_impl {
    ($U:ty, $S:ty) => {
        impl<'a, T: Clone + Num> Pow<$U> for &'a Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn pow(self, mut exp: $U) -> Self::Output {
                if exp == 0 {
                    return Complex::one();
                }
                let mut base = self.clone();

                while exp & 1 == 0 {
                    base = base.clone() * base;
                    exp >>= 1;
                }

                if exp == 1 {
                    return base;
                }

                let mut acc = base.clone();
                while exp > 1 {
                    exp >>= 1;
                    base = base.clone() * base;
                    if exp & 1 == 1 {
                        acc = acc * base.clone();
                    }
                }
                acc
            }
        }

        impl<'a, 'b, T: Clone + Num> Pow<&'b $U> for &'a Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn pow(self, exp: &$U) -> Self::Output {
                self.pow(*exp)
            }
        }

        impl<'a, T: Clone + Num + Neg<Output = T>> Pow<$S> for &'a Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn pow(self, exp: $S) -> Self::Output {
                if exp < 0 {
                    Pow::pow(&self.inv(), exp.wrapping_neg() as $U)
                } else {
                    Pow::pow(self, exp as $U)
                }
            }
        }

        impl<'a, 'b, T: Clone + Num + Neg<Output = T>> Pow<&'b $S> for &'a Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn pow(self, exp: &$S) -> Self::Output {
                self.pow(*exp)
            }
        }
    };
}

pow_impl!(u8, i8);
pow_impl!(u16, i16);
pow_impl!(u32, i32);
pow_impl!(u64, i64);
pow_impl!(usize, isize);
#[cfg(has_i128)]
pow_impl!(u128, i128);

// Note: the impls above are for `&Complex<T>`, while those below are for `Complex<T>`.  This is
// fine since `Float: Copy` anyway, but it's also necessary to avoid conflicting implementations.
// Otherwise rustc would insist that those `Pow<{integer}>` impls could overlap with `Pow<T>` if
// integers ever implement `Float`, though of course we know they won't...

impl<T: Float> Pow<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn pow(self, exp: T) -> Self::Output {
        self.powf(exp)
    }
}

impl<T: Float> Pow<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn pow(self, exp: Complex<T>) -> Self::Output {
        self.powc(exp)
    }
}

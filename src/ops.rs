use super::Complex;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use traits::{Num, Zero};

macro_rules! forward_ref_ref_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, 'b, T: Clone + Num> $imp<&'b Complex<T>> for &'a Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, other: &Complex<T>) -> Complex<T> {
                self.clone().$method(other.clone())
            }
        }
    }
}

macro_rules! forward_ref_val_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T: Clone + Num> $imp<Complex<T>> for &'a Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, other: Complex<T>) -> Complex<T> {
                self.clone().$method(other)
            }
        }
    }
}

macro_rules! forward_val_ref_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, T: Clone + Num> $imp<&'a Complex<T>> for Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, other: &Complex<T>) -> Complex<T> {
                self.$method(other.clone())
            }
        }
    }
}

macro_rules! forward_all_binop {
    (impl $imp:ident, $method:ident) => {
        forward_ref_ref_binop!(impl $imp, $method);
        forward_ref_val_binop!(impl $imp, $method);
        forward_val_ref_binop!(impl $imp, $method);
    };
}

/* arithmetic */
forward_all_binop!(impl Add, add);

// (a + i b) + (c + i d) == (a + c) + i (b + d)
impl<T: Clone + Num> Add<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn add(self, other: Complex<T>) -> Complex<T> {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

forward_all_binop!(impl Sub, sub);

// (a + i b) - (c + i d) == (a - c) + i (b - d)
impl<T: Clone + Num> Sub<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn sub(self, other: Complex<T>) -> Complex<T> {
        Complex::new(self.re - other.re, self.im - other.im)
    }
}

forward_all_binop!(impl Mul, mul);

// (a + i b) * (c + i d) == (a*c - b*d) + i (a*d + b*c)
impl<T: Clone + Num> Mul<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn mul(self, other: Complex<T>) -> Complex<T> {
        let re = self.re.clone() * other.re.clone() - self.im.clone() * other.im.clone();
        let im = self.re * other.im + self.im * other.re;
        Complex::new(re, im)
    }
}

forward_all_binop!(impl Div, div);

// (a + i b) / (c + i d) == [(a + i b) * (c - i d)] / (c*c + d*d)
//   == [(a*c + b*d) / (c*c + d*d)] + i [(b*c - a*d) / (c*c + d*d)]
impl<T: Clone + Num> Div<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn div(self, other: Complex<T>) -> Complex<T> {
        let norm_sqr = other.norm_sqr();
        let re = self.re.clone() * other.re.clone() + self.im.clone() * other.im.clone();
        let im = self.im * other.re - self.re * other.im;
        Complex::new(re / norm_sqr.clone(), im / norm_sqr)
    }
}

forward_all_binop!(impl Rem, rem);

// Attempts to identify the gaussian integer whose product with `modulus`
// is closest to `self`.
impl<T: Clone + Num> Rem<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn rem(self, modulus: Complex<T>) -> Self {
        let Complex { re, im } = self.clone() / modulus.clone();
        // This is the gaussian integer corresponding to the true ratio
        // rounded towards zero.
        let (re0, im0) = (re.clone() - re % T::one(), im.clone() - im % T::one());
        self - modulus * Complex::new(re0, im0)
    }
}

// Op Assign

mod opassign {
    use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

    use traits::NumAssign;

    use Complex;

    impl<T: Clone + NumAssign> AddAssign for Complex<T> {
        fn add_assign(&mut self, other: Complex<T>) {
            self.re += other.re;
            self.im += other.im;
        }
    }

    impl<T: Clone + NumAssign> SubAssign for Complex<T> {
        fn sub_assign(&mut self, other: Complex<T>) {
            self.re -= other.re;
            self.im -= other.im;
        }
    }

    impl<T: Clone + NumAssign> MulAssign for Complex<T> {
        fn mul_assign(&mut self, other: Complex<T>) {
            *self = self.clone() * other;
        }
    }

    impl<T: Clone + NumAssign> DivAssign for Complex<T> {
        fn div_assign(&mut self, other: Complex<T>) {
            *self = self.clone() / other;
        }
    }

    impl<T: Clone + NumAssign> RemAssign for Complex<T> {
        fn rem_assign(&mut self, other: Complex<T>) {
            *self = self.clone() % other;
        }
    }

    impl<T: Clone + NumAssign> AddAssign<T> for Complex<T> {
        fn add_assign(&mut self, other: T) {
            self.re += other;
        }
    }

    impl<T: Clone + NumAssign> SubAssign<T> for Complex<T> {
        fn sub_assign(&mut self, other: T) {
            self.re -= other;
        }
    }

    impl<T: Clone + NumAssign> MulAssign<T> for Complex<T> {
        fn mul_assign(&mut self, other: T) {
            self.re *= other.clone();
            self.im *= other;
        }
    }

    impl<T: Clone + NumAssign> DivAssign<T> for Complex<T> {
        fn div_assign(&mut self, other: T) {
            self.re /= other.clone();
            self.im /= other;
        }
    }

    impl<T: Clone + NumAssign> RemAssign<T> for Complex<T> {
        fn rem_assign(&mut self, other: T) {
            *self = self.clone() % other;
        }
    }

    macro_rules! forward_op_assign {
        (impl $imp:ident, $method:ident) => {
            impl<'a, T: Clone + NumAssign> $imp<&'a Complex<T>> for Complex<T> {
                #[inline]
                fn $method(&mut self, other: &Complex<T>) {
                    self.$method(other.clone())
                }
            }
            impl<'a, T: Clone + NumAssign> $imp<&'a T> for Complex<T> {
                #[inline]
                fn $method(&mut self, other: &T) {
                    self.$method(other.clone())
                }
            }
        }
    }

    forward_op_assign!(impl AddAssign, add_assign);
    forward_op_assign!(impl SubAssign, sub_assign);
    forward_op_assign!(impl MulAssign, mul_assign);
    forward_op_assign!(impl DivAssign, div_assign);

    impl<'a, T: Clone + NumAssign> RemAssign<&'a Complex<T>> for Complex<T> {
        #[inline]
        fn rem_assign(&mut self, other: &Complex<T>) {
            self.rem_assign(other.clone())
        }
    }
    impl<'a, T: Clone + NumAssign> RemAssign<&'a T> for Complex<T> {
        #[inline]
        fn rem_assign(&mut self, other: &T) {
            self.rem_assign(other.clone())
        }
    }
}

impl<T: Clone + Num + Neg<Output = T>> Neg for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn neg(self) -> Complex<T> {
        Complex::new(-self.re, -self.im)
    }
}

impl<'a, T: Clone + Num + Neg<Output = T>> Neg for &'a Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn neg(self) -> Complex<T> {
        -self.clone()
    }
}

macro_rules! real_arithmetic {
    (@forward $imp:ident::$method:ident for $($real:ident),*) => (
        impl<'a, T: Clone + Num> $imp<&'a T> for Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, other: &T) -> Complex<T> {
                self.$method(other.clone())
            }
        }
        impl<'a, T: Clone + Num> $imp<T> for &'a Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, other: T) -> Complex<T> {
                self.clone().$method(other)
            }
        }
        impl<'a, 'b, T: Clone + Num> $imp<&'a T> for &'b Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, other: &T) -> Complex<T> {
                self.clone().$method(other.clone())
            }
        }
        $(
            impl<'a> $imp<&'a Complex<$real>> for $real {
                type Output = Complex<$real>;

                #[inline]
                fn $method(self, other: &Complex<$real>) -> Complex<$real> {
                    self.$method(other.clone())
                }
            }
            impl<'a> $imp<Complex<$real>> for &'a $real {
                type Output = Complex<$real>;

                #[inline]
                fn $method(self, other: Complex<$real>) -> Complex<$real> {
                    self.clone().$method(other)
                }
            }
            impl<'a, 'b> $imp<&'a Complex<$real>> for &'b $real {
                type Output = Complex<$real>;

                #[inline]
                fn $method(self, other: &Complex<$real>) -> Complex<$real> {
                    self.clone().$method(other.clone())
                }
            }
        )*
    );
    ($($real:ident),*) => (
        real_arithmetic!(@forward Add::add for $($real),*);
        real_arithmetic!(@forward Sub::sub for $($real),*);
        real_arithmetic!(@forward Mul::mul for $($real),*);
        real_arithmetic!(@forward Div::div for $($real),*);
        real_arithmetic!(@forward Rem::rem for $($real),*);

        $(
            impl Add<Complex<$real>> for $real {
                type Output = Complex<$real>;

                #[inline]
                fn add(self, other: Complex<$real>) -> Complex<$real> {
                    Complex::new(self + other.re, other.im)
                }
            }

            impl Sub<Complex<$real>> for $real {
                type Output = Complex<$real>;

                #[inline]
                fn sub(self, other: Complex<$real>) -> Complex<$real> {
                    Complex::new(self - other.re, $real::zero() - other.im)
                }
            }

            impl Mul<Complex<$real>> for $real {
                type Output = Complex<$real>;

                #[inline]
                fn mul(self, other: Complex<$real>) -> Complex<$real> {
                    Complex::new(self * other.re, self * other.im)
                }
            }

            impl Div<Complex<$real>> for $real {
                type Output = Complex<$real>;

                #[inline]
                fn div(self, other: Complex<$real>) -> Complex<$real> {
                    // a / (c + i d) == [a * (c - i d)] / (c*c + d*d)
                    let norm_sqr = other.norm_sqr();
                    Complex::new(self * other.re / norm_sqr.clone(),
                                 $real::zero() - self * other.im / norm_sqr)
                }
            }

            impl Rem<Complex<$real>> for $real {
                type Output = Complex<$real>;

                #[inline]
                fn rem(self, other: Complex<$real>) -> Complex<$real> {
                    Complex::new(self, Self::zero()) % other
                }
            }
        )*
    );
}

impl<T: Clone + Num> Add<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn add(self, other: T) -> Complex<T> {
        Complex::new(self.re + other, self.im)
    }
}

impl<T: Clone + Num> Sub<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn sub(self, other: T) -> Complex<T> {
        Complex::new(self.re - other, self.im)
    }
}

impl<T: Clone + Num> Mul<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn mul(self, other: T) -> Complex<T> {
        Complex::new(self.re * other.clone(), self.im * other)
    }
}

impl<T: Clone + Num> Div<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn div(self, other: T) -> Complex<T> {
        Complex::new(self.re / other.clone(), self.im / other)
    }
}

impl<T: Clone + Num> Rem<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn rem(self, other: T) -> Complex<T> {
        self % Complex::new(other, T::zero())
    }
}

real_arithmetic!(usize, u8, u16, u32, u64, isize, i8, i16, i32, i64, f32, f64);

#[cfg(test)]
mod tests {
    #![allow(non_upper_case_globals)]

    use super::super::{Complex, Complex64};

    const _0_0i: Complex64 = Complex { re: 0.0, im: 0.0 };
    const _1_0i: Complex64 = Complex { re: 1.0, im: 0.0 };
    const _1_1i: Complex64 = Complex { re: 1.0, im: 1.0 };
    const _0_1i: Complex64 = Complex { re: 0.0, im: 1.0 };
    const _neg1_1i: Complex64 = Complex { re: -1.0, im: 1.0 };
    const _05_05i: Complex64 = Complex { re: 0.5, im: 0.5 };
    const all_consts: [Complex64; 5] = [_0_0i, _1_0i, _1_1i, _neg1_1i, _05_05i];
    const _4_2i: Complex64 = Complex { re: 4.0, im: 2.0 };

    // Test both a + b and a += b
    macro_rules! test_a_op_b {
        ($a:ident + $b:expr, $answer:expr) => {
            assert_eq!($a + $b, $answer);
            assert_eq!({ let mut x = $a; x += $b; x}, $answer);
        };
        ($a:ident - $b:expr, $answer:expr) => {
            assert_eq!($a - $b, $answer);
            assert_eq!({ let mut x = $a; x -= $b; x}, $answer);
        };
        ($a:ident * $b:expr, $answer:expr) => {
            assert_eq!($a * $b, $answer);
            assert_eq!({ let mut x = $a; x *= $b; x}, $answer);
        };
        ($a:ident / $b:expr, $answer:expr) => {
            assert_eq!($a / $b, $answer);
            assert_eq!({ let mut x = $a; x /= $b; x}, $answer);
        };
        ($a:ident % $b:expr, $answer:expr) => {
            assert_eq!($a % $b, $answer);
            assert_eq!({ let mut x = $a; x %= $b; x}, $answer);
        }
    }

    // Test both a + b and a + &b
    macro_rules! test_op {
        ($a:ident $op:tt $b:expr, $answer:expr) => {
            test_a_op_b!($a $op $b, $answer);
            test_a_op_b!($a $op &$b, $answer);
        }
    }

    mod complex_arithmetic {
        use super::{all_consts, _05_05i, _0_0i, _0_1i, _1_0i, _1_1i, _4_2i, _neg1_1i};
        use traits::Zero;

        #[test]
        fn test_add() {
            test_op!(_05_05i + _05_05i, _1_1i);
            test_op!(_0_1i + _1_0i, _1_1i);
            test_op!(_1_0i + _neg1_1i, _0_1i);

            for &c in all_consts.iter() {
                test_op!(_0_0i + c, c);
                test_op!(c + _0_0i, c);
            }
        }

        #[test]
        fn test_sub() {
            test_op!(_05_05i - _05_05i, _0_0i);
            test_op!(_0_1i - _1_0i, _neg1_1i);
            test_op!(_0_1i - _neg1_1i, _1_0i);

            for &c in all_consts.iter() {
                test_op!(c - _0_0i, c);
                test_op!(c - c, _0_0i);
            }
        }

        #[test]
        fn test_mul() {
            test_op!(_05_05i * _05_05i, _0_1i.unscale(2.0));
            test_op!(_1_1i * _0_1i, _neg1_1i);

            // i^2 & i^4
            test_op!(_0_1i * _0_1i, -_1_0i);
            assert_eq!(_0_1i * _0_1i * _0_1i * _0_1i, _1_0i);

            for &c in all_consts.iter() {
                test_op!(c * _1_0i, c);
                test_op!(_1_0i * c, c);
            }
        }

        #[test]
        fn test_div() {
            test_op!(_neg1_1i / _0_1i, _1_1i);
            for &c in all_consts.iter() {
                if c != Zero::zero() {
                    test_op!(c / c, _1_0i);
                }
            }
        }

        #[test]
        fn test_rem() {
            test_op!(_neg1_1i % _0_1i, _0_0i);
            test_op!(_4_2i % _0_1i, _0_0i);
            test_op!(_05_05i % _0_1i, _05_05i);
            test_op!(_05_05i % _1_1i, _05_05i);
            assert_eq!((_4_2i + _05_05i) % _0_1i, _05_05i);
            assert_eq!((_4_2i + _05_05i) % _1_1i, _05_05i);
        }

        #[test]
        fn test_neg() {
            assert_eq!(-_1_0i + _0_1i, _neg1_1i);
            assert_eq!((-_0_1i) * _0_1i, _1_0i);
            for &c in all_consts.iter() {
                assert_eq!(-(-c), c);
            }
        }
    }

    mod real_arithmetic {
        use super::super::Complex;
        use super::{_4_2i, _neg1_1i};

        #[test]
        fn test_add() {
            test_op!(_4_2i + 0.5, Complex::new(4.5, 2.0));
            assert_eq!(0.5 + _4_2i, Complex::new(4.5, 2.0));
        }

        #[test]
        fn test_sub() {
            test_op!(_4_2i - 0.5, Complex::new(3.5, 2.0));
            assert_eq!(0.5 - _4_2i, Complex::new(-3.5, -2.0));
        }

        #[test]
        fn test_mul() {
            assert_eq!(_4_2i * 0.5, Complex::new(2.0, 1.0));
            assert_eq!(0.5 * _4_2i, Complex::new(2.0, 1.0));
        }

        #[test]
        fn test_div() {
            assert_eq!(_4_2i / 0.5, Complex::new(8.0, 4.0));
            assert_eq!(0.5 / _4_2i, Complex::new(0.1, -0.05));
        }

        #[test]
        fn test_rem() {
            assert_eq!(_4_2i % 2.0, Complex::new(0.0, 0.0));
            assert_eq!(_4_2i % 3.0, Complex::new(1.0, 2.0));
            assert_eq!(3.0 % _4_2i, Complex::new(3.0, 0.0));
            assert_eq!(_neg1_1i % 2.0, _neg1_1i);
            assert_eq!(-_4_2i % 3.0, Complex::new(-1.0, -2.0));
        }
    }
}

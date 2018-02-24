// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Complex numbers.
//!
//! ## Compatibility
//!
//! The `num-complex` crate is tested for rustc 1.15 and greater.

#![doc(html_root_url = "https://docs.rs/num-complex/0.1")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

extern crate num_traits as traits;

#[cfg(feature = "rustc-serialize")]
extern crate rustc_serialize;

#[cfg(feature = "serde")]
extern crate serde;

mod ops;
#[cfg(feature = "std")]
mod from_str;
#[cfg(feature = "std")]
mod fmt;
#[cfg(feature = "std")]
mod math;

use core::ops::Neg;
use traits::{Num, One, Zero};

#[cfg(not(feature = "std"))]
use traits::float::FloatCore as Float;
#[cfg(feature = "std")]
use traits::Float;

// FIXME #1284: handle complex NaN & infinity etc. This
// probably doesn't map to C's _Complex correctly.

/// A complex number in Cartesian form.
///
/// ## Representation and Foreign Function Interface Compatibility
///
/// `Complex<T>` is memory layout compatible with an array `[T; 2]`.
///
/// Note that `Complex<F>` where F is a floating point type is **only** memory
/// layout compatible with C's complex types, **not** necessarily calling
/// convention compatible.  This means that for FFI you can only pass
/// `Complex<F>` behind a pointer, not as a value.
///
/// ## Examples
///
/// Example of extern function declaration.
///
/// ```
/// use num_complex::Complex;
/// use std::os::raw::c_int;
///
/// extern "C" {
///     fn zaxpy_(n: *const c_int, alpha: *const Complex<f64>,
///               x: *const Complex<f64>, incx: *const c_int,
///               y: *mut Complex<f64>, incy: *const c_int);
/// }
/// ```
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
#[repr(C)]
pub struct Complex<T> {
    /// Real portion of the complex number
    pub re: T,
    /// Imaginary portion of the complex number
    pub im: T,
}

pub type Complex32 = Complex<f32>;
pub type Complex64 = Complex<f64>;

impl<T: Clone + Num> Complex<T> {
    /// Create a new Complex
    #[inline]
    pub fn new(re: T, im: T) -> Complex<T> {
        Complex { re: re, im: im }
    }

    /// Returns imaginary unit
    #[inline]
    pub fn i() -> Complex<T> {
        Self::new(T::zero(), T::one())
    }

    /// Returns the square of the norm (since `T` doesn't necessarily
    /// have a sqrt function), i.e. `re^2 + im^2`.
    #[inline]
    pub fn norm_sqr(&self) -> T {
        self.re.clone() * self.re.clone() + self.im.clone() * self.im.clone()
    }

    /// Multiplies `self` by the scalar `t`.
    #[inline]
    pub fn scale(&self, t: T) -> Complex<T> {
        Complex::new(self.re.clone() * t.clone(), self.im.clone() * t)
    }

    /// Divides `self` by the scalar `t`.
    #[inline]
    pub fn unscale(&self, t: T) -> Complex<T> {
        Complex::new(self.re.clone() / t.clone(), self.im.clone() / t)
    }
}

impl<T: Clone + Num + Neg<Output = T>> Complex<T> {
    /// Returns the complex conjugate. i.e. `re - i im`
    #[inline]
    pub fn conj(&self) -> Complex<T> {
        Complex::new(self.re.clone(), -self.im.clone())
    }

    /// Returns `1/self`
    #[inline]
    pub fn inv(&self) -> Complex<T> {
        let norm_sqr = self.norm_sqr();
        Complex::new(
            self.re.clone() / norm_sqr.clone(),
            -self.im.clone() / norm_sqr,
        )
    }
}

impl<T: Float> Complex<T> {
    /// Checks if the given complex number is NaN
    #[inline]
    pub fn is_nan(self) -> bool {
        self.re.is_nan() || self.im.is_nan()
    }

    /// Checks if the given complex number is infinite
    #[inline]
    pub fn is_infinite(self) -> bool {
        !self.is_nan() && (self.re.is_infinite() || self.im.is_infinite())
    }

    /// Checks if the given complex number is finite
    #[inline]
    pub fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }

    /// Checks if the given complex number is normal
    #[inline]
    pub fn is_normal(self) -> bool {
        self.re.is_normal() && self.im.is_normal()
    }
}

impl<T: Clone + Num> From<T> for Complex<T> {
    #[inline]
    fn from(re: T) -> Complex<T> {
        Complex {
            re: re,
            im: T::zero(),
        }
    }
}

impl<'a, T: Clone + Num> From<&'a T> for Complex<T> {
    #[inline]
    fn from(re: &T) -> Complex<T> {
        From::from(re.clone())
    }
}

/* constants */
impl<T: Clone + Num> Zero for Complex<T> {
    #[inline]
    fn zero() -> Complex<T> {
        Complex::new(Zero::zero(), Zero::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.re.is_zero() && self.im.is_zero()
    }
}

impl<T: Clone + Num> One for Complex<T> {
    #[inline]
    fn one() -> Complex<T> {
        Complex::new(One::one(), Zero::zero())
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for Complex<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: serde::Serializer,
    {
        (&self.re, &self.im).serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Deserialize for Complex<T>
where
    T: serde::Deserialize + Num + Clone,
{
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer,
    {
        let (re, im) = try!(serde::Deserialize::deserialize(deserializer));
        Ok(Complex::new(re, im))
    }
}

#[cfg(test)]
mod test {
    #![allow(non_upper_case_globals)]

    use super::{Complex, Complex64};
    use core::f64;
    use traits::{One, Zero};

    const _0_0i: Complex64 = Complex { re: 0.0, im: 0.0 };
    const _1_0i: Complex64 = Complex { re: 1.0, im: 0.0 };
    const _1_1i: Complex64 = Complex { re: 1.0, im: 1.0 };
    const _0_1i: Complex64 = Complex { re: 0.0, im: 1.0 };
    const _neg1_1i: Complex64 = Complex { re: -1.0, im: 1.0 };
    const _05_05i: Complex64 = Complex { re: 0.5, im: 0.5 };
    const all_consts: [Complex64; 5] = [_0_0i, _1_0i, _1_1i, _neg1_1i, _05_05i];
    const _4_2i: Complex64 = Complex { re: 4.0, im: 2.0 };

    #[test]
    fn test_consts() {
        // check our constants are what Complex::new creates
        fn test(c: Complex64, r: f64, i: f64) {
            assert_eq!(c, Complex::new(r, i));
        }
        test(_0_0i, 0.0, 0.0);
        test(_1_0i, 1.0, 0.0);
        test(_1_1i, 1.0, 1.0);
        test(_neg1_1i, -1.0, 1.0);
        test(_05_05i, 0.5, 0.5);

        assert_eq!(_0_0i, Zero::zero());
        assert_eq!(_1_0i, One::one());
    }

    #[test]
    fn test_scale_unscale() {
        assert_eq!(_05_05i.scale(2.0), _1_1i);
        assert_eq!(_1_1i.unscale(2.0), _05_05i);
        for &c in all_consts.iter() {
            assert_eq!(c.scale(2.0).unscale(2.0), c);
        }
    }

    #[test]
    fn test_conj() {
        for &c in all_consts.iter() {
            assert_eq!(c.conj(), Complex::new(c.re, -c.im));
            assert_eq!(c.conj().conj(), c);
        }
    }

    #[test]
    fn test_inv() {
        assert_eq!(_1_1i.inv(), _05_05i.conj());
        assert_eq!(_1_0i.inv(), _1_0i.inv());
    }

    #[test]
    #[should_panic]
    fn test_divide_by_zero_natural() {
        let n = Complex::new(2, 3);
        let d = Complex::new(0, 0);
        let _x = n / d;
    }

    #[test]
    fn test_inv_zero() {
        // FIXME #20: should this really fail, or just NaN?
        assert!(_0_0i.inv().is_nan());
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_hash() {
        let a = Complex::new(0i32, 0i32);
        let b = Complex::new(1i32, 0i32);
        let c = Complex::new(0i32, 1i32);
        assert!(hash(&a) != hash(&b));
        assert!(hash(&b) != hash(&c));
        assert!(hash(&c) != hash(&a));

        fn hash<T: ::std::hash::Hash>(x: &T) -> u64 {
            use std::hash::{BuildHasher, Hasher};
            use std::collections::hash_map::RandomState;
            let mut hasher = <RandomState as BuildHasher>::Hasher::new();
            x.hash(&mut hasher);
            hasher.finish()
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_hashset() {
        use std::collections::HashSet;
        let a = Complex::new(0i32, 0i32);
        let b = Complex::new(1i32, 0i32);
        let c = Complex::new(0i32, 1i32);

        let set: HashSet<_> = [a, b, c].iter().cloned().collect();
        assert!(set.contains(&a));
        assert!(set.contains(&b));
        assert!(set.contains(&c));
        assert!(!set.contains(&(a + b + c)));
    }

    #[test]
    fn test_is_nan() {
        assert!(!_1_1i.is_nan());
        let a = Complex::new(f64::NAN, f64::NAN);
        assert!(a.is_nan());
    }

    #[test]
    fn test_is_nan_special_cases() {
        let a = Complex::new(0f64, f64::NAN);
        let b = Complex::new(f64::NAN, 0f64);
        assert!(a.is_nan());
        assert!(b.is_nan());
    }

    #[test]
    fn test_is_infinite() {
        let a = Complex::new(2f64, f64::INFINITY);
        assert!(a.is_infinite());
    }

    #[test]
    fn test_is_finite() {
        assert!(_1_1i.is_finite())
    }

    #[test]
    fn test_is_normal() {
        let a = Complex::new(0f64, f64::NAN);
        let b = Complex::new(2f64, f64::INFINITY);
        assert!(!a.is_normal());
        assert!(!b.is_normal());
        assert!(_1_1i.is_normal());
    }
}

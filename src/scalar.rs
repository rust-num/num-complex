
use traits::{Num, FromPrimitive, Float};
use std::ops::Neg;

use Complex;

pub trait Scalar: Num + Copy + Neg<Output = Self>
where
    Complex<Self::Real>: Scalar<
        Real = Self::Real,
        Complex = Complex<Self::Real>,
    >,
{
    /// Associated Real type
    type Real: Scalar<Real = Self::Real, Complex = Complex<Self::Real>> + Float + FromPrimitive;

    /// Associated Complex type
    type Complex: Scalar<Real = Self::Real, Complex = Complex<Self::Real>>;

    /// Take the square root of a number.
    fn sqrt(&self) -> Self;

    /// Returns `e^(self)`, (the exponential function).
    fn exp(&self) -> Self;

    /// Returns the natural logarithm of the number.
    fn ln(&self) -> Self;

    /// Returns the square of the absolute value of the number
    fn abs_sqr(&self) -> Self::Real;

    /// Returns the absolute value of the number
    fn abs(&self) -> Self::Real;

    /// Raise a number to an integer power.
    fn powi(&self, exp: i32) -> Self;

    /// Raise a number to a floating point power.
    fn powf(&self, exp: Self::Real) -> Self;

    /// Raise a number to a complex power.
    fn powc(&self, exp: Self::Complex) -> Self::Complex;

    /// Returns complex-conjugate number
    fn conj(&self) -> Self;

    /// Computes the sine of a number
    fn sin(&self) -> Self;

    /// Computes the cosine of a number
    fn cos(&self) -> Self;

    /// Computes the tangent of a number
    fn tan(&self) -> Self;

    /// Computes the arcsine of a number
    fn asin(&self) -> Self;

    /// Computes the arccosine of a number
    fn acos(&self) -> Self;

    /// Computes the arctangent of a number
    fn atan(&self) -> Self;

    /// Computes the hyperbolic-sine of a number
    fn sinh(&self) -> Self;

    /// Computes the hyperbolic-cosine of a number
    fn cosh(&self) -> Self;

    /// Computes the hyperbolic-tangent of a number
    fn tanh(&self) -> Self;

    /// Computes the hyperbolic-arcsine of a number
    fn asinh(&self) -> Self;

    /// Computes the hyperbolic-arccosine of a number
    fn acosh(&self) -> Self;

    /// Computes the hyperbolic-arctangent of a number
    fn atanh(&self) -> Self;

    /// Checks if the given (real or imaginary part of complex) number is NaN
    fn is_nan(self) -> bool;

    /// Checks if the given (real or imaginary part of complex) number is infinite
    fn is_infinite(self) -> bool;

    /// Checks if the given number is finite
    fn is_finite(self) -> bool;

    /// Checks if the given number is normal
    fn is_normal(self) -> bool;
}

macro_rules! impl_complex_unary {
    ($($unary:ident),*) => {
        $(
        fn $unary(&self) -> Self {
            Complex::$unary(self)
        }
        )*
    }
}

macro_rules! impl_float_unary {
    ($($unary:ident),*) => {
        $(
        fn $unary(&self) -> Self {
            Float::$unary(*self)
        }
        )*
    }
}

macro_rules! impl_is_any {
    ($($is_any:ident),*) => {
        $(
        fn $is_any(self) -> bool {
            self.$is_any()
        }
        )*
    }
}

impl<T: Float + FromPrimitive> Scalar for Complex<T> {
    type Real = T;
    type Complex = Self;

    fn abs_sqr(&self) -> Self::Real {
        Complex::norm_sqr(self)
    }

    fn abs(&self) -> Self::Real {
        Complex::norm(self)
    }

    fn powi(&self, exp: i32) -> Self {
        let exp = T::from_i32(exp).unwrap();
        Complex::powf(self, exp)
    }

    fn powf(&self, exp: Self::Real) -> Self {
        Complex::powf(self, exp)
    }

    fn powc(&self, exp: Self::Complex) -> Self::Complex {
        Complex::powc(self, exp)
    }

    impl_complex_unary!(
        sqrt, exp, ln, conj,
        sin, cos, tan,
        asin, acos, atan,
        sinh, cosh, tanh,
        asinh, acosh, atanh
    );
    impl_is_any!(is_nan, is_infinite, is_finite, is_normal);
}

impl<T: Float + FromPrimitive> Scalar for T {
    type Real = T;
    type Complex = Complex<T>;

    fn abs_sqr(&self) -> Self::Real {
        *self * *self
    }

    fn powi(&self, exp: i32) -> Self {
        Float::powi(*self, exp)
    }

    fn powf(&self, exp: Self::Real) -> Self {
        Float::powf(*self, exp)
    }

    fn powc(&self, exp: Self::Complex) -> Self::Complex {
        exp.expf(*self)
    }

    fn conj(&self) -> Self {
        *self
    }

    impl_float_unary!(
        sqrt, exp, ln, abs,
        sin, cos, tan,
        asin, acos, atan,
        sinh, cosh, tanh,
        asinh, acosh, atanh
    );
    impl_is_any!(is_nan, is_infinite, is_finite, is_normal);
}

#[cfg(test)]
mod test {
    use super::{Scalar, Complex};

    #[test]
    fn scalar_sqrt() {
        fn call_sqrt<T: Scalar>(x: T) -> T {
            x.sqrt()
        }
        call_sqrt(2.0_f64);
        call_sqrt(Complex::from(2.0_f64));
    }
}

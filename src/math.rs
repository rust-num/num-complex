use super::Complex;
use traits::{Float, One};

impl<T: Clone + Float> Complex<T> {
    /// Calculate |self|
    #[inline]
    pub fn norm(&self) -> T {
        self.re.hypot(self.im)
    }
    /// Calculate the principal Arg of self.
    #[inline]
    pub fn arg(&self) -> T {
        self.im.atan2(self.re)
    }
    /// Convert to polar form (r, theta), such that
    /// `self = r * exp(i * theta)`
    #[inline]
    pub fn to_polar(&self) -> (T, T) {
        (self.norm(), self.arg())
    }
    /// Convert a polar representation into a complex number.
    #[inline]
    pub fn from_polar(r: &T, theta: &T) -> Complex<T> {
        Complex::new(*r * theta.cos(), *r * theta.sin())
    }

    /// Computes `e^(self)`, where `e` is the base of the natural logarithm.
    #[inline]
    pub fn exp(&self) -> Complex<T> {
        // formula: e^(a + bi) = e^a (cos(b) + i*sin(b))
        // = from_polar(e^a, b)
        Complex::from_polar(&self.re.exp(), &self.im)
    }

    /// Computes the principal value of natural logarithm of `self`.
    ///
    /// This function has one branch cut:
    ///
    /// * `(-∞, 0]`, continuous from above.
    ///
    /// The branch satisfies `-π ≤ arg(ln(z)) ≤ π`.
    #[inline]
    pub fn ln(&self) -> Complex<T> {
        // formula: ln(z) = ln|z| + i*arg(z)
        let (r, theta) = self.to_polar();
        Complex::new(r.ln(), theta)
    }

    /// Computes the principal value of the square root of `self`.
    ///
    /// This function has one branch cut:
    ///
    /// * `(-∞, 0)`, continuous from above.
    ///
    /// The branch satisfies `-π/2 ≤ arg(sqrt(z)) ≤ π/2`.
    #[inline]
    pub fn sqrt(&self) -> Complex<T> {
        // formula: sqrt(r e^(it)) = sqrt(r) e^(it/2)
        let two = T::one() + T::one();
        let (r, theta) = self.to_polar();
        Complex::from_polar(&(r.sqrt()), &(theta / two))
    }

    /// Raises `self` to a floating point power.
    #[inline]
    pub fn powf(&self, exp: T) -> Complex<T> {
        // formula: x^y = (ρ e^(i θ))^y = ρ^y e^(i θ y)
        // = from_polar(ρ^y, θ y)
        let (r, theta) = self.to_polar();
        Complex::from_polar(&r.powf(exp), &(theta * exp))
    }

    /// Returns the logarithm of `self` with respect to an arbitrary base.
    #[inline]
    pub fn log(&self, base: T) -> Complex<T> {
        // formula: log_y(x) = log_y(ρ e^(i θ))
        // = log_y(ρ) + log_y(e^(i θ)) = log_y(ρ) + ln(e^(i θ)) / ln(y)
        // = log_y(ρ) + i θ / ln(y)
        let (r, theta) = self.to_polar();
        Complex::new(r.log(base), theta / base.ln())
    }

    /// Raises `self` to a complex power.
    #[inline]
    pub fn powc(&self, exp: Complex<T>) -> Complex<T> {
        // formula: x^y = (a + i b)^(c + i d)
        // = (ρ e^(i θ))^c (ρ e^(i θ))^(i d)
        //    where ρ=|x| and θ=arg(x)
        // = ρ^c e^(−d θ) e^(i c θ) ρ^(i d)
        // = p^c e^(−d θ) (cos(c θ)
        //   + i sin(c θ)) (cos(d ln(ρ)) + i sin(d ln(ρ)))
        // = p^c e^(−d θ) (
        //   cos(c θ) cos(d ln(ρ)) − sin(c θ) sin(d ln(ρ))
        //   + i(cos(c θ) sin(d ln(ρ)) + sin(c θ) cos(d ln(ρ))))
        // = p^c e^(−d θ) (cos(c θ + d ln(ρ)) + i sin(c θ + d ln(ρ)))
        // = from_polar(p^c e^(−d θ), c θ + d ln(ρ))
        let (r, theta) = self.to_polar();
        Complex::from_polar(
            &(r.powf(exp.re) * (-exp.im * theta).exp()),
            &(exp.re * theta + exp.im * r.ln()),
        )
    }

    /// Raises a floating point number to the complex power `self`.
    #[inline]
    pub fn expf(&self, base: T) -> Complex<T> {
        // formula: x^(a+bi) = x^a x^bi = x^a e^(b ln(x) i)
        // = from_polar(x^a, b ln(x))
        Complex::from_polar(&base.powf(self.re), &(self.im * base.ln()))
    }

    /// Computes the sine of `self`.
    #[inline]
    pub fn sin(&self) -> Complex<T> {
        // formula: sin(a + bi) = sin(a)cosh(b) + i*cos(a)sinh(b)
        Complex::new(
            self.re.sin() * self.im.cosh(),
            self.re.cos() * self.im.sinh(),
        )
    }

    /// Computes the cosine of `self`.
    #[inline]
    pub fn cos(&self) -> Complex<T> {
        // formula: cos(a + bi) = cos(a)cosh(b) - i*sin(a)sinh(b)
        Complex::new(
            self.re.cos() * self.im.cosh(),
            -self.re.sin() * self.im.sinh(),
        )
    }

    /// Computes the tangent of `self`.
    #[inline]
    pub fn tan(&self) -> Complex<T> {
        // formula: tan(a + bi) = (sin(2a) + i*sinh(2b))/(cos(2a) + cosh(2b))
        let (two_re, two_im) = (self.re + self.re, self.im + self.im);
        Complex::new(two_re.sin(), two_im.sinh()).unscale(two_re.cos() + two_im.cosh())
    }

    /// Computes the principal value of the inverse sine of `self`.
    ///
    /// This function has two branch cuts:
    ///
    /// * `(-∞, -1)`, continuous from above.
    /// * `(1, ∞)`, continuous from below.
    ///
    /// The branch satisfies `-π/2 ≤ Re(asin(z)) ≤ π/2`.
    #[inline]
    pub fn asin(&self) -> Complex<T> {
        // formula: arcsin(z) = -i ln(sqrt(1-z^2) + iz)
        let i = Complex::<T>::i();
        -i * ((Complex::<T>::one() - self * self).sqrt() + i * self).ln()
    }

    /// Computes the principal value of the inverse cosine of `self`.
    ///
    /// This function has two branch cuts:
    ///
    /// * `(-∞, -1)`, continuous from above.
    /// * `(1, ∞)`, continuous from below.
    ///
    /// The branch satisfies `0 ≤ Re(acos(z)) ≤ π`.
    #[inline]
    pub fn acos(&self) -> Complex<T> {
        // formula: arccos(z) = -i ln(i sqrt(1-z^2) + z)
        let i = Complex::<T>::i();
        -i * (i * (Complex::<T>::one() - self * self).sqrt() + self).ln()
    }

    /// Computes the principal value of the inverse tangent of `self`.
    ///
    /// This function has two branch cuts:
    ///
    /// * `(-∞i, -i]`, continuous from the left.
    /// * `[i, ∞i)`, continuous from the right.
    ///
    /// The branch satisfies `-π/2 ≤ Re(atan(z)) ≤ π/2`.
    #[inline]
    pub fn atan(&self) -> Complex<T> {
        // formula: arctan(z) = (ln(1+iz) - ln(1-iz))/(2i)
        let i = Complex::<T>::i();
        let one = Complex::<T>::one();
        let two = one + one;
        if *self == i {
            return Complex::new(T::zero(), T::infinity());
        } else if *self == -i {
            return Complex::new(T::zero(), -T::infinity());
        }
        ((one + i * self).ln() - (one - i * self).ln()) / (two * i)
    }

    /// Computes the hyperbolic sine of `self`.
    #[inline]
    pub fn sinh(&self) -> Complex<T> {
        // formula: sinh(a + bi) = sinh(a)cos(b) + i*cosh(a)sin(b)
        Complex::new(
            self.re.sinh() * self.im.cos(),
            self.re.cosh() * self.im.sin(),
        )
    }

    /// Computes the hyperbolic cosine of `self`.
    #[inline]
    pub fn cosh(&self) -> Complex<T> {
        // formula: cosh(a + bi) = cosh(a)cos(b) + i*sinh(a)sin(b)
        Complex::new(
            self.re.cosh() * self.im.cos(),
            self.re.sinh() * self.im.sin(),
        )
    }

    /// Computes the hyperbolic tangent of `self`.
    #[inline]
    pub fn tanh(&self) -> Complex<T> {
        // formula: tanh(a + bi) = (sinh(2a) + i*sin(2b))/(cosh(2a) + cos(2b))
        let (two_re, two_im) = (self.re + self.re, self.im + self.im);
        Complex::new(two_re.sinh(), two_im.sin()).unscale(two_re.cosh() + two_im.cos())
    }

    /// Computes the principal value of inverse hyperbolic sine of `self`.
    ///
    /// This function has two branch cuts:
    ///
    /// * `(-∞i, -i)`, continuous from the left.
    /// * `(i, ∞i)`, continuous from the right.
    ///
    /// The branch satisfies `-π/2 ≤ Im(asinh(z)) ≤ π/2`.
    #[inline]
    pub fn asinh(&self) -> Complex<T> {
        // formula: arcsinh(z) = ln(z + sqrt(1+z^2))
        let one = Complex::<T>::one();
        (self + (one + self * self).sqrt()).ln()
    }

    /// Computes the principal value of inverse hyperbolic cosine of `self`.
    ///
    /// This function has one branch cut:
    ///
    /// * `(-∞, 1)`, continuous from above.
    ///
    /// The branch satisfies `-π ≤ Im(acosh(z)) ≤ π` and `0 ≤ Re(acosh(z)) < ∞`.
    #[inline]
    pub fn acosh(&self) -> Complex<T> {
        // formula: arccosh(z) = 2 ln(sqrt((z+1)/2) + sqrt((z-1)/2))
        let one = Complex::one();
        let two = one + one;
        two * (((self + one) / two).sqrt() + ((self - one) / two).sqrt()).ln()
    }

    /// Computes the principal value of inverse hyperbolic tangent of `self`.
    ///
    /// This function has two branch cuts:
    ///
    /// * `(-∞, -1]`, continuous from above.
    /// * `[1, ∞)`, continuous from below.
    ///
    /// The branch satisfies `-π/2 ≤ Im(atanh(z)) ≤ π/2`.
    #[inline]
    pub fn atanh(&self) -> Complex<T> {
        // formula: arctanh(z) = (ln(1+z) - ln(1-z))/2
        let one = Complex::one();
        let two = one + one;
        if *self == one {
            return Complex::new(T::infinity(), T::zero());
        } else if *self == -one {
            return Complex::new(-T::infinity(), T::zero());
        }
        ((one + self).ln() - (one - self).ln()) / two
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_upper_case_globals)]

    use super::super::{Complex, Complex64};
    use std::f64;

    use traits::Float;

    pub const _0_0i: Complex64 = Complex { re: 0.0, im: 0.0 };
    pub const _1_0i: Complex64 = Complex { re: 1.0, im: 0.0 };
    pub const _1_1i: Complex64 = Complex { re: 1.0, im: 1.0 };
    pub const _0_1i: Complex64 = Complex { re: 0.0, im: 1.0 };
    pub const _neg1_1i: Complex64 = Complex { re: -1.0, im: 1.0 };
    pub const _05_05i: Complex64 = Complex { re: 0.5, im: 0.5 };
    pub const all_consts: [Complex64; 5] = [_0_0i, _1_0i, _1_1i, _neg1_1i, _05_05i];
    pub const _4_2i: Complex64 = Complex { re: 4.0, im: 2.0 };

    #[test]
    fn test_arg() {
        fn test(c: Complex64, arg: f64) {
            assert!((c.arg() - arg).abs() < 1.0e-6)
        }
        test(_1_0i, 0.0);
        test(_1_1i, 0.25 * f64::consts::PI);
        test(_neg1_1i, 0.75 * f64::consts::PI);
        test(_05_05i, 0.25 * f64::consts::PI);
    }

    #[test]
    fn test_polar_conv() {
        fn test(c: Complex64) {
            let (r, theta) = c.to_polar();
            assert!((c - Complex::from_polar(&r, &theta)).norm() < 1e-6);
        }
        for &c in all_consts.iter() {
            test(c);
        }
    }

    fn close(a: Complex64, b: Complex64) -> bool {
        close_to_tol(a, b, 1e-10)
    }

    fn close_to_tol(a: Complex64, b: Complex64, tol: f64) -> bool {
        // returns true if a and b are reasonably close
        (a == b) || (a - b).norm() < tol
    }

    #[test]
    fn test_exp() {
        assert!(close(_1_0i.exp(), _1_0i.scale(f64::consts::E)));
        assert!(close(_0_0i.exp(), _1_0i));
        assert!(close(_0_1i.exp(), Complex::new(1.0.cos(), 1.0.sin())));
        assert!(close(_05_05i.exp() * _05_05i.exp(), _1_1i.exp()));
        assert!(close(
            _0_1i.scale(-f64::consts::PI).exp(),
            _1_0i.scale(-1.0)
        ));
        for &c in all_consts.iter() {
            // e^conj(z) = conj(e^z)
            assert!(close(c.conj().exp(), c.exp().conj()));
            // e^(z + 2 pi i) = e^z
            assert!(close(
                c.exp(),
                (c + _0_1i.scale(f64::consts::PI * 2.0)).exp()
            ));
        }
    }

    #[test]
    fn test_ln() {
        assert!(close(_1_0i.ln(), _0_0i));
        assert!(close(_0_1i.ln(), _0_1i.scale(f64::consts::PI / 2.0)));
        assert!(close(_0_0i.ln(), Complex::new(f64::neg_infinity(), 0.0)));
        assert!(close(
            (_neg1_1i * _05_05i).ln(),
            _neg1_1i.ln() + _05_05i.ln()
        ));
        for &c in all_consts.iter() {
            // ln(conj(z() = conj(ln(z))
            assert!(close(c.conj().ln(), c.ln().conj()));
            // for this branch, -pi <= arg(ln(z)) <= pi
            assert!(-f64::consts::PI <= c.ln().arg() && c.ln().arg() <= f64::consts::PI);
        }
    }

    #[test]
    fn test_powc() {
        let a = Complex::new(2.0, -3.0);
        let b = Complex::new(3.0, 0.0);
        assert!(close(a.powc(b), a.powf(b.re)));
        assert!(close(b.powc(a), a.expf(b.re)));
        let c = Complex::new(1.0 / 3.0, 0.1);
        assert!(close_to_tol(
            a.powc(c),
            Complex::new(1.65826, -0.33502),
            1e-5
        ));
    }

    #[test]
    fn test_powf() {
        let c = Complex::new(2.0, -1.0);
        let r = c.powf(3.5);
        assert!(close_to_tol(r, Complex::new(-0.8684746, -16.695934), 1e-5));
    }

    #[test]
    fn test_log() {
        let c = Complex::new(2.0, -1.0);
        let r = c.log(10.0);
        assert!(close_to_tol(r, Complex::new(0.349485, -0.20135958), 1e-5));
    }

    #[test]
    fn test_some_expf_cases() {
        let c = Complex::new(2.0, -1.0);
        let r = c.expf(10.0);
        assert!(close_to_tol(r, Complex::new(-66.82015, -74.39803), 1e-5));

        let c = Complex::new(5.0, -2.0);
        let r = c.expf(3.4);
        assert!(close_to_tol(r, Complex::new(-349.25, -290.63), 1e-2));

        let c = Complex::new(-1.5, 2.0 / 3.0);
        let r = c.expf(1.0 / 3.0);
        assert!(close_to_tol(r, Complex::new(3.8637, -3.4745), 1e-2));
    }

    #[test]
    fn test_sqrt() {
        assert!(close(_0_0i.sqrt(), _0_0i));
        assert!(close(_1_0i.sqrt(), _1_0i));
        assert!(close(Complex::new(-1.0, 0.0).sqrt(), _0_1i));
        assert!(close(Complex::new(-1.0, -0.0).sqrt(), _0_1i.scale(-1.0)));
        assert!(close(_0_1i.sqrt(), _05_05i.scale(2.0.sqrt())));
        for &c in all_consts.iter() {
            // sqrt(conj(z() = conj(sqrt(z))
            assert!(close(c.conj().sqrt(), c.sqrt().conj()));
            // for this branch, -pi/2 <= arg(sqrt(z)) <= pi/2
            assert!(
                -f64::consts::PI / 2.0 <= c.sqrt().arg() && c.sqrt().arg() <= f64::consts::PI / 2.0
            );
            // sqrt(z) * sqrt(z) = z
            assert!(close(c.sqrt() * c.sqrt(), c));
        }
    }

    #[test]
    fn test_sin() {
        assert!(close(_0_0i.sin(), _0_0i));
        assert!(close(_1_0i.scale(f64::consts::PI * 2.0).sin(), _0_0i));
        assert!(close(_0_1i.sin(), _0_1i.scale(1.0.sinh())));
        for &c in all_consts.iter() {
            // sin(conj(z)) = conj(sin(z))
            assert!(close(c.conj().sin(), c.sin().conj()));
            // sin(-z) = -sin(z)
            assert!(close(c.scale(-1.0).sin(), c.sin().scale(-1.0)));
        }
    }

    #[test]
    fn test_cos() {
        assert!(close(_0_0i.cos(), _1_0i));
        assert!(close(_1_0i.scale(f64::consts::PI * 2.0).cos(), _1_0i));
        assert!(close(_0_1i.cos(), _1_0i.scale(1.0.cosh())));
        for &c in all_consts.iter() {
            // cos(conj(z)) = conj(cos(z))
            assert!(close(c.conj().cos(), c.cos().conj()));
            // cos(-z) = cos(z)
            assert!(close(c.scale(-1.0).cos(), c.cos()));
        }
    }

    #[test]
    fn test_tan() {
        assert!(close(_0_0i.tan(), _0_0i));
        assert!(close(_1_0i.scale(f64::consts::PI / 4.0).tan(), _1_0i));
        assert!(close(_1_0i.scale(f64::consts::PI).tan(), _0_0i));
        for &c in all_consts.iter() {
            // tan(conj(z)) = conj(tan(z))
            assert!(close(c.conj().tan(), c.tan().conj()));
            // tan(-z) = -tan(z)
            assert!(close(c.scale(-1.0).tan(), c.tan().scale(-1.0)));
        }
    }

    #[test]
    fn test_asin() {
        assert!(close(_0_0i.asin(), _0_0i));
        assert!(close(_1_0i.asin(), _1_0i.scale(f64::consts::PI / 2.0)));
        assert!(close(
            _1_0i.scale(-1.0).asin(),
            _1_0i.scale(-f64::consts::PI / 2.0)
        ));
        assert!(close(_0_1i.asin(), _0_1i.scale((1.0 + 2.0.sqrt()).ln())));
        for &c in all_consts.iter() {
            // asin(conj(z)) = conj(asin(z))
            assert!(close(c.conj().asin(), c.asin().conj()));
            // asin(-z) = -asin(z)
            assert!(close(c.scale(-1.0).asin(), c.asin().scale(-1.0)));
            // for this branch, -pi/2 <= asin(z).re <= pi/2
            assert!(-f64::consts::PI / 2.0 <= c.asin().re && c.asin().re <= f64::consts::PI / 2.0);
        }
    }

    #[test]
    fn test_acos() {
        assert!(close(_0_0i.acos(), _1_0i.scale(f64::consts::PI / 2.0)));
        assert!(close(_1_0i.acos(), _0_0i));
        assert!(close(
            _1_0i.scale(-1.0).acos(),
            _1_0i.scale(f64::consts::PI)
        ));
        assert!(close(
            _0_1i.acos(),
            Complex::new(f64::consts::PI / 2.0, (2.0.sqrt() - 1.0).ln())
        ));
        for &c in all_consts.iter() {
            // acos(conj(z)) = conj(acos(z))
            assert!(close(c.conj().acos(), c.acos().conj()));
            // for this branch, 0 <= acos(z).re <= pi
            assert!(0.0 <= c.acos().re && c.acos().re <= f64::consts::PI);
        }
    }

    #[test]
    fn test_atan() {
        assert!(close(_0_0i.atan(), _0_0i));
        assert!(close(_1_0i.atan(), _1_0i.scale(f64::consts::PI / 4.0)));
        assert!(close(
            _1_0i.scale(-1.0).atan(),
            _1_0i.scale(-f64::consts::PI / 4.0)
        ));
        assert!(close(_0_1i.atan(), Complex::new(0.0, f64::infinity())));
        for &c in all_consts.iter() {
            // atan(conj(z)) = conj(atan(z))
            assert!(close(c.conj().atan(), c.atan().conj()));
            // atan(-z) = -atan(z)
            assert!(close(c.scale(-1.0).atan(), c.atan().scale(-1.0)));
            // for this branch, -pi/2 <= atan(z).re <= pi/2
            assert!(-f64::consts::PI / 2.0 <= c.atan().re && c.atan().re <= f64::consts::PI / 2.0);
        }
    }

    #[test]
    fn test_sinh() {
        assert!(close(_0_0i.sinh(), _0_0i));
        assert!(close(
            _1_0i.sinh(),
            _1_0i.scale((f64::consts::E - 1.0 / f64::consts::E) / 2.0)
        ));
        assert!(close(_0_1i.sinh(), _0_1i.scale(1.0.sin())));
        for &c in all_consts.iter() {
            // sinh(conj(z)) = conj(sinh(z))
            assert!(close(c.conj().sinh(), c.sinh().conj()));
            // sinh(-z) = -sinh(z)
            assert!(close(c.scale(-1.0).sinh(), c.sinh().scale(-1.0)));
        }
    }

    #[test]
    fn test_cosh() {
        assert!(close(_0_0i.cosh(), _1_0i));
        assert!(close(
            _1_0i.cosh(),
            _1_0i.scale((f64::consts::E + 1.0 / f64::consts::E) / 2.0)
        ));
        assert!(close(_0_1i.cosh(), _1_0i.scale(1.0.cos())));
        for &c in all_consts.iter() {
            // cosh(conj(z)) = conj(cosh(z))
            assert!(close(c.conj().cosh(), c.cosh().conj()));
            // cosh(-z) = cosh(z)
            assert!(close(c.scale(-1.0).cosh(), c.cosh()));
        }
    }

    #[test]
    fn test_tanh() {
        assert!(close(_0_0i.tanh(), _0_0i));
        assert!(close(
            _1_0i.tanh(),
            _1_0i.scale((f64::consts::E.powi(2) - 1.0) / (f64::consts::E.powi(2) + 1.0))
        ));
        assert!(close(_0_1i.tanh(), _0_1i.scale(1.0.tan())));
        for &c in all_consts.iter() {
            // tanh(conj(z)) = conj(tanh(z))
            assert!(close(c.conj().tanh(), c.conj().tanh()));
            // tanh(-z) = -tanh(z)
            assert!(close(c.scale(-1.0).tanh(), c.tanh().scale(-1.0)));
        }
    }

    #[test]
    fn test_asinh() {
        assert!(close(_0_0i.asinh(), _0_0i));
        assert!(close(_1_0i.asinh(), _1_0i.scale(1.0 + 2.0.sqrt()).ln()));
        assert!(close(_0_1i.asinh(), _0_1i.scale(f64::consts::PI / 2.0)));
        assert!(close(
            _0_1i.asinh().scale(-1.0),
            _0_1i.scale(-f64::consts::PI / 2.0)
        ));
        for &c in all_consts.iter() {
            // asinh(conj(z)) = conj(asinh(z))
            assert!(close(c.conj().asinh(), c.conj().asinh()));
            // asinh(-z) = -asinh(z)
            assert!(close(c.scale(-1.0).asinh(), c.asinh().scale(-1.0)));
            // for this branch, -pi/2 <= asinh(z).im <= pi/2
            assert!(
                -f64::consts::PI / 2.0 <= c.asinh().im && c.asinh().im <= f64::consts::PI / 2.0
            );
        }
    }

    #[test]
    fn test_acosh() {
        assert!(close(_0_0i.acosh(), _0_1i.scale(f64::consts::PI / 2.0)));
        assert!(close(_1_0i.acosh(), _0_0i));
        assert!(close(
            _1_0i.scale(-1.0).acosh(),
            _0_1i.scale(f64::consts::PI)
        ));
        for &c in all_consts.iter() {
            // acosh(conj(z)) = conj(acosh(z))
            assert!(close(c.conj().acosh(), c.conj().acosh()));
            // for this branch, -pi <= acosh(z).im <= pi and 0 <= acosh(z).re
            assert!(
                -f64::consts::PI <= c.acosh().im && c.acosh().im <= f64::consts::PI
                    && 0.0 <= c.cosh().re
            );
        }
    }

    #[test]
    fn test_atanh() {
        assert!(close(_0_0i.atanh(), _0_0i));
        assert!(close(_0_1i.atanh(), _0_1i.scale(f64::consts::PI / 4.0)));
        assert!(close(_1_0i.atanh(), Complex::new(f64::infinity(), 0.0)));
        for &c in all_consts.iter() {
            // atanh(conj(z)) = conj(atanh(z))
            assert!(close(c.conj().atanh(), c.conj().atanh()));
            // atanh(-z) = -atanh(z)
            assert!(close(c.scale(-1.0).atanh(), c.atanh().scale(-1.0)));
            // for this branch, -pi/2 <= atanh(z).im <= pi/2
            assert!(
                -f64::consts::PI / 2.0 <= c.atanh().im && c.atanh().im <= f64::consts::PI / 2.0
            );
        }
    }

    #[test]
    fn test_exp_ln() {
        for &c in all_consts.iter() {
            // e^ln(z) = z
            assert!(close(c.ln().exp(), c));
        }
    }

    #[test]
    fn test_trig_to_hyperbolic() {
        for &c in all_consts.iter() {
            // sin(iz) = i sinh(z)
            assert!(close((_0_1i * c).sin(), _0_1i * c.sinh()));
            // cos(iz) = cosh(z)
            assert!(close((_0_1i * c).cos(), c.cosh()));
            // tan(iz) = i tanh(z)
            assert!(close((_0_1i * c).tan(), _0_1i * c.tanh()));
        }
    }

    #[test]
    fn test_trig_identities() {
        for &c in all_consts.iter() {
            // tan(z) = sin(z)/cos(z)
            assert!(close(c.tan(), c.sin() / c.cos()));
            // sin(z)^2 + cos(z)^2 = 1
            assert!(close(c.sin() * c.sin() + c.cos() * c.cos(), _1_0i));

            // sin(asin(z)) = z
            assert!(close(c.asin().sin(), c));
            // cos(acos(z)) = z
            assert!(close(c.acos().cos(), c));
            // tan(atan(z)) = z
            // i and -i are branch points
            if c != _0_1i && c != _0_1i.scale(-1.0) {
                assert!(close(c.atan().tan(), c));
            }

            // sin(z) = (e^(iz) - e^(-iz))/(2i)
            assert!(close(
                ((_0_1i * c).exp() - (_0_1i * c).exp().inv()) / _0_1i.scale(2.0),
                c.sin()
            ));
            // cos(z) = (e^(iz) + e^(-iz))/2
            assert!(close(
                ((_0_1i * c).exp() + (_0_1i * c).exp().inv()).unscale(2.0),
                c.cos()
            ));
            // tan(z) = i (1 - e^(2iz))/(1 + e^(2iz))
            assert!(close(
                _0_1i * (_1_0i - (_0_1i * c).scale(2.0).exp())
                    / (_1_0i + (_0_1i * c).scale(2.0).exp()),
                c.tan()
            ));
        }
    }

    #[test]
    fn test_hyperbolic_identites() {
        for &c in all_consts.iter() {
            // tanh(z) = sinh(z)/cosh(z)
            assert!(close(c.tanh(), c.sinh() / c.cosh()));
            // cosh(z)^2 - sinh(z)^2 = 1
            assert!(close(c.cosh() * c.cosh() - c.sinh() * c.sinh(), _1_0i));

            // sinh(asinh(z)) = z
            assert!(close(c.asinh().sinh(), c));
            // cosh(acosh(z)) = z
            assert!(close(c.acosh().cosh(), c));
            // tanh(atanh(z)) = z
            // 1 and -1 are branch points
            if c != _1_0i && c != _1_0i.scale(-1.0) {
                assert!(close(c.atanh().tanh(), c));
            }

            // sinh(z) = (e^z - e^(-z))/2
            assert!(close((c.exp() - c.exp().inv()).unscale(2.0), c.sinh()));
            // cosh(z) = (e^z + e^(-z))/2
            assert!(close((c.exp() + c.exp().inv()).unscale(2.0), c.cosh()));
            // tanh(z) = ( e^(2z) - 1)/(e^(2z) + 1)
            assert!(close(
                (c.scale(2.0).exp() - _1_0i) / (c.scale(2.0).exp() + _1_0i),
                c.tanh()
            ));
        }
    }

}

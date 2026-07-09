//! Trait-based `cis` for both real and complex inputs.
//!
//! This provides a `cis()` method for both real floats and `Complex<T>` so callers can
//! uniformly write `x.cis()` whether `x` is a `T` (float) or a `Complex<T>`.

/// Compute cis(x) = exp(i*x). Implemented for both real floats and Complex<T>.
///
/// - For real `T`, `x.cis()` returns `Complex::new(x.cos(), x.sin())`.
/// - For `Complex<T>`, `z.cis()` returns `exp(i*z) = exp(-Im(z)) * cis(Re(z))`.
pub trait Cis {
    type Output;
    fn cis(self) -> Self::Output;
}

#[cfg(any(feature = "std", feature = "libm"))]
mod imp {
    use super::Cis;
    use crate::Complex;
    use num_traits::Float;

    impl<T: Float> Cis for T {
        type Output = Complex<T>;

        fn cis(self) -> Complex<T> {
            // cis(x) = cos(x) + i*sin(x)
            Complex::new(self.cos(), self.sin())
        }
    }

    impl<T: Float> Cis for Complex<T> {
        type Output = Complex<T>;

        fn cis(self) -> Complex<T> {
            // cis(a+ib) = exp(-b) * cis(a)
            let scale = (-self.im).exp();
            self.re.cis() * scale
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::Cis;
        use crate::Complex;

        #[test]
        fn cis_real() {
            // Compare a hard-coded example to a reference result via WolframAlpha:
            // https://www.wolframalpha.com/input?i=cis%281.2345%29

            let theta = 1.2345;
            let expected = Complex::new(0.3299931576785677, 0.9439833239445111);
            assert!((expected - theta.cis()).norm_sqr() < 1e-10);
        }

        #[test]
        fn cis_complex() {
            // Compare a hard-coded example to a reference result via WolframAlpha:
            // https://www.wolframalpha.com/input?i=cis%28-6.789%2Bi1.2345%29

            let z = Complex::new(-6.7890, 1.2345);
            let expected = Complex::new(0.254543682056692, -0.1409858152159941);
            assert!((expected - z.cis()).norm_sqr() < 1e-10);
        }
    }
}

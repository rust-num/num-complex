//! Rand implementations for complex numbers

use crate::Complex;
use num_traits::Num;
use rand::distr::StandardUniform;
use rand::prelude::*;

impl<T> Distribution<Complex<T>> for StandardUniform
where
    T: Num + Clone,
    StandardUniform: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Complex<T> {
        Complex::new(self.sample(rng), self.sample(rng))
    }
}

/// A generic random value distribution for complex numbers.
#[derive(Clone, Copy, Debug)]
pub struct ComplexDistribution<Re, Im = Re> {
    re: Re,
    im: Im,
}

impl<Re, Im> ComplexDistribution<Re, Im> {
    /// Creates a complex distribution from independent
    /// distributions of the real and imaginary parts.
    pub fn new(re: Re, im: Im) -> Self {
        ComplexDistribution { re, im }
    }
}

impl<T, Re, Im> Distribution<Complex<T>> for ComplexDistribution<Re, Im>
where
    T: Num + Clone,
    Re: Distribution<T>,
    Im: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Complex<T> {
        Complex::new(self.re.sample(rng), self.im.sample(rng))
    }
}

#[cfg(test)]
fn test_rng() -> impl Rng {
    /// Simple `Rng` for testing without additional dependencies
    struct XorShiftStar {
        a: u64,
    }

    impl rand::TryRng for XorShiftStar {
        type Error = core::convert::Infallible;

        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Ok(self.next_u64() as u32)
        }

        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            // https://en.wikipedia.org/wiki/Xorshift#xorshift*
            self.a ^= self.a >> 12;
            self.a ^= self.a << 25;
            self.a ^= self.a >> 27;
            Ok(self.a.wrapping_mul(0x2545_F491_4F6C_DD1D))
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
            rand::rand_core::utils::fill_bytes_via_next_word(dest, || self.try_next_u64())
        }
    }

    XorShiftStar {
        a: 0x0123_4567_89AB_CDEF,
    }
}

#[test]
fn standard_f64() {
    let mut rng = test_rng();
    for _ in 0..100 {
        let c: Complex<f64> = rng.random();
        assert!(c.re >= 0.0 && c.re < 1.0);
        assert!(c.im >= 0.0 && c.im < 1.0);
    }
}

#[test]
fn generic_standard_f64() {
    let mut rng = test_rng();
    let dist = ComplexDistribution::new(StandardUniform, StandardUniform);
    for _ in 0..100 {
        let c: Complex<f64> = rng.sample(dist);
        assert!(c.re >= 0.0 && c.re < 1.0);
        assert!(c.im >= 0.0 && c.im < 1.0);
    }
}

#[test]
fn generic_uniform_f64() {
    use rand::distr::Uniform;

    let mut rng = test_rng();
    let re = Uniform::new(-100.0, 0.0).unwrap();
    let im = Uniform::new(0.0, 100.0).unwrap();
    let dist = ComplexDistribution::new(re, im);
    for _ in 0..100 {
        // no type annotation required, since `Uniform` only produces one type.
        let c = rng.sample(dist);
        assert!(c.re >= -100.0 && c.re < 0.0);
        assert!(c.im >= 0.0 && c.im < 100.0);
    }
}

#[test]
fn generic_mixed_f64() {
    use rand::distr::Uniform;

    let mut rng = test_rng();
    let re = Uniform::new(-100.0, 0.0).unwrap();
    let dist = ComplexDistribution::new(re, StandardUniform);
    for _ in 0..100 {
        // no type annotation required, since `Uniform` only produces one type.
        let c = rng.sample(dist);
        assert!(c.re >= -100.0 && c.re < 0.0);
        assert!(c.im >= 0.0 && c.im < 1.0);
    }
}

#[test]
fn generic_uniform_i32() {
    use rand::distr::Uniform;

    let mut rng = test_rng();
    let re = Uniform::new(-100, 0).unwrap();
    let im = Uniform::new(0, 100).unwrap();
    let dist = ComplexDistribution::new(re, im);
    for _ in 0..100 {
        // no type annotation required, since `Uniform` only produces one type.
        let c = rng.sample(dist);
        assert!(c.re >= -100 && c.re < 0);
        assert!(c.im >= 0 && c.im < 100);
    }
}

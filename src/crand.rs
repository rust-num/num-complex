//! Rand implementations for complex numbers

use Complex;
use rand::distributions::Standard;
use rand::prelude::*;
use traits::Num;

impl<T> Distribution<Complex<T>> for Standard
where
    T: Num + Clone,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Complex<T> {
        Complex::new(self.sample(rng), self.sample(rng))
    }
}

#[test]
fn standard_f64() {
    let mut rng = SmallRng::from_seed([42; 16]);
    for _ in 0..100 {
        let c: Complex<f64> = rng.gen();
        assert!(c.re >= 0.0 && c.re < 1.0);
        assert!(c.im >= 0.0 && c.im < 1.0);
    }
}

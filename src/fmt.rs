use super::Complex;
use core::fmt;
use traits::{Num, Zero};

macro_rules! write_complex {
    ($f:ident, $t:expr, $prefix:expr, $re:expr, $im:expr, $T:ident) => {{
        let abs_re = if $re < Zero::zero() { $T::zero() - $re.clone() } else { $re.clone() };
        let abs_im = if $im < Zero::zero() { $T::zero() - $im.clone() } else { $im.clone() };

        let real: String;
        let imag: String;

        if let Some(prec) = $f.precision() {
            real = format!(concat!("{:.1$", $t, "}"), abs_re, prec);
            imag = format!(concat!("{:.1$", $t, "}"), abs_im, prec);
        }
        else {
            real = format!(concat!("{:", $t, "}"), abs_re);
            imag = format!(concat!("{:", $t, "}"), abs_im);
        }

        let prefix = if $f.alternate() { $prefix } else { "" };
        let sign = if $re < Zero::zero() {
            "-"
        } else if $f.sign_plus() {
            "+"
        } else {
            ""
        };

        let complex = if $im < Zero::zero() {
            format!("{}{pre}{re}-{pre}{im}i", sign, re=real, im=imag, pre=prefix)
        }
        else {
            format!("{}{pre}{re}+{pre}{im}i", sign, re=real, im=imag, pre=prefix)
        };

        if let Some(width) = $f.width() {
            write!($f, "{0: >1$}", complex, width)
        }
        else {
            write!($f, "{}", complex)
        }
    }}
}

/* string conversions */
impl<T> fmt::Display for Complex<T>
where
    T: fmt::Display + Num + PartialOrd + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_complex!(f, "", "", self.re, self.im, T)
    }
}

impl<T> fmt::LowerExp for Complex<T>
where
    T: fmt::LowerExp + Num + PartialOrd + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_complex!(f, "e", "", self.re, self.im, T)
    }
}

impl<T> fmt::UpperExp for Complex<T>
where
    T: fmt::UpperExp + Num + PartialOrd + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_complex!(f, "E", "", self.re, self.im, T)
    }
}

impl<T> fmt::LowerHex for Complex<T>
where
    T: fmt::LowerHex + Num + PartialOrd + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_complex!(f, "x", "0x", self.re, self.im, T)
    }
}

impl<T> fmt::UpperHex for Complex<T>
where
    T: fmt::UpperHex + Num + PartialOrd + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_complex!(f, "X", "0x", self.re, self.im, T)
    }
}

impl<T> fmt::Octal for Complex<T>
where
    T: fmt::Octal + Num + PartialOrd + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_complex!(f, "o", "0o", self.re, self.im, T)
    }
}

impl<T> fmt::Binary for Complex<T>
where
    T: fmt::Binary + Num + PartialOrd + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_complex!(f, "b", "0b", self.re, self.im, T)
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_upper_case_globals)]

    use super::super::{Complex, Complex64};

    pub const _0_0i: Complex64 = Complex { re: 0.0, im: 0.0 };
    pub const _1_0i: Complex64 = Complex { re: 1.0, im: 0.0 };
    pub const _1_1i: Complex64 = Complex { re: 1.0, im: 1.0 };
    pub const _0_1i: Complex64 = Complex { re: 0.0, im: 1.0 };
    pub const _neg1_1i: Complex64 = Complex { re: -1.0, im: 1.0 };
    pub const _05_05i: Complex64 = Complex { re: 0.5, im: 0.5 };
    pub const _4_2i: Complex64 = Complex { re: 4.0, im: 2.0 };

    #[test]
    fn test_to_string() {
        fn test(c: Complex64, s: String) {
            assert_eq!(c.to_string(), s);
        }
        test(_0_0i, "0+0i".to_string());
        test(_1_0i, "1+0i".to_string());
        test(_0_1i, "0+1i".to_string());
        test(_1_1i, "1+1i".to_string());
        test(_neg1_1i, "-1+1i".to_string());
        test(-_neg1_1i, "1-1i".to_string());
        test(_05_05i, "0.5+0.5i".to_string());
    }

    #[test]
    fn test_string_formatting() {
        let a = Complex::new(1.23456, 123.456);
        assert_eq!(format!("{}", a), "1.23456+123.456i");
        assert_eq!(format!("{:.2}", a), "1.23+123.46i");
        assert_eq!(format!("{:.2e}", a), "1.23e0+1.23e2i");
        assert_eq!(format!("{:+20.2E}", a), "     +1.23E0+1.23E2i");

        let b = Complex::new(0x80, 0xff);
        assert_eq!(format!("{:X}", b), "80+FFi");
        assert_eq!(format!("{:#x}", b), "0x80+0xffi");
        assert_eq!(format!("{:+#b}", b), "+0b10000000+0b11111111i");
        assert_eq!(format!("{:+#16o}", b), "   +0o200+0o377i");

        let c = Complex::new(-10, -10000);
        assert_eq!(format!("{}", c), "-10-10000i");
        assert_eq!(format!("{:16}", c), "      -10-10000i");
    }
}

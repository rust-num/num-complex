use super::Complex;
use std::fmt;
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

use super::Complex;

use std::error::Error;
use std::str::FromStr;

use traits::Num;

pub fn from_str_generic<T, E, F>(s: &str, from: F) -> Result<Complex<T>, ParseComplexError<E>>
where
    F: Fn(&str) -> Result<T, E>,
    T: Clone + Num,
{
    let imag = match s.rfind('j') {
        None => 'i',
        _ => 'j',
    };

    let mut b = String::with_capacity(s.len());
    let mut first = true;

    let char_indices = s.char_indices();
    let mut pc = ' ';
    let mut split_index = s.len();

    for (i, cc) in char_indices {
        if cc == '+' && pc != 'e' && pc != 'E' && i > 0 {
            // ignore '+' if part of an exponent
            if first {
                split_index = i;
                first = false;
            }
            // don't carry '+' over into b
            pc = ' ';
            continue;
        } else if cc == '-' && pc != 'e' && pc != 'E' && i > 0 {
            // ignore '-' if part of an exponent or begins the string
            if first {
                split_index = i;
                first = false;
            }
            // DO carry '-' over into b
        }

        if pc == '-' && cc == ' ' && !first {
            // ignore whitespace between minus sign and next number
            continue;
        }

        if !first {
            b.push(cc);
        }
        pc = cc;
    }

    // split off real and imaginary parts, trim whitespace
    let (a, _) = s.split_at(split_index);
    let a = a.trim_right();
    let mut b = b.trim_left();
    // input was either pure real or pure imaginary
    if b.is_empty() {
        b = match a.ends_with(imag) {
            false => "0i",
            true => "0",
        };
    }

    let re;
    let im;
    if a.ends_with(imag) {
        im = a;
        re = b;
    } else if b.ends_with(imag) {
        re = a;
        im = b;
    } else {
        return Err(ParseComplexError::new());
    }

    // parse re
    let re = try!(from(re).map_err(ParseComplexError::from_error));

    // pop imaginary unit off
    let mut im = &im[..im.len() - 1];
    // handle im == "i" or im == "-i"
    if im.is_empty() || im == "+" {
        im = "1";
    } else if im == "-" {
        im = "-1";
    }

    // parse im
    let im = try!(from(im).map_err(ParseComplexError::from_error));

    Ok(Complex::new(re, im))
}

impl<T> FromStr for Complex<T>
where
    T: FromStr + Num + Clone,
{
    type Err = ParseComplexError<T::Err>;

    /// Parses `a +/- bi`; `ai +/- b`; `a`; or `bi` where `a` and `b` are of type `T`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_str_generic(s, T::from_str)
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseComplexError<E> {
    kind: ComplexErrorKind<E>,
}

#[derive(Debug, PartialEq)]
enum ComplexErrorKind<E> {
    ParseError(E),
    ExprError,
}

impl<E> ParseComplexError<E> {
    fn new() -> Self {
        ParseComplexError {
            kind: ComplexErrorKind::ExprError,
        }
    }

    fn from_error(error: E) -> Self {
        ParseComplexError {
            kind: ComplexErrorKind::ParseError(error),
        }
    }
}

impl<E: Error> Error for ParseComplexError<E> {
    fn description(&self) -> &str {
        match self.kind {
            ComplexErrorKind::ParseError(ref e) => e.description(),
            ComplexErrorKind::ExprError => "invalid or unsupported complex expression",
        }
    }
}

impl<E: Error> ::std::fmt::Display for ParseComplexError<E> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.description().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_upper_case_globals)]

    use super::super::{Complex, Complex64};
    use std::str::FromStr;

    use traits::Num;

    pub const _0_0i: Complex64 = Complex { re: 0.0, im: 0.0 };
    pub const _1_0i: Complex64 = Complex { re: 1.0, im: 0.0 };
    pub const _1_1i: Complex64 = Complex { re: 1.0, im: 1.0 };
    pub const _0_1i: Complex64 = Complex { re: 0.0, im: 1.0 };
    pub const _neg1_1i: Complex64 = Complex { re: -1.0, im: 1.0 };
    pub const _05_05i: Complex64 = Complex { re: 0.5, im: 0.5 };
    pub const _4_2i: Complex64 = Complex { re: 4.0, im: 2.0 };

    #[test]
    fn test_from_str() {
        fn test(z: Complex64, s: &str) {
            assert_eq!(FromStr::from_str(s), Ok(z));
        }
        test(_0_0i, "0 + 0i");
        test(_0_0i, "0+0j");
        test(_0_0i, "0 - 0j");
        test(_0_0i, "0-0i");
        test(_0_0i, "0i + 0");
        test(_0_0i, "0");
        test(_0_0i, "-0");
        test(_0_0i, "0i");
        test(_0_0i, "0j");
        test(_0_0i, "+0j");
        test(_0_0i, "-0i");

        test(_1_0i, "1 + 0i");
        test(_1_0i, "1+0j");
        test(_1_0i, "1 - 0j");
        test(_1_0i, "+1-0i");
        test(_1_0i, "-0j+1");
        test(_1_0i, "1");

        test(_1_1i, "1 + i");
        test(_1_1i, "1+j");
        test(_1_1i, "1 + 1j");
        test(_1_1i, "1+1i");
        test(_1_1i, "i + 1");
        test(_1_1i, "1i+1");
        test(_1_1i, "+j+1");

        test(_0_1i, "0 + i");
        test(_0_1i, "0+j");
        test(_0_1i, "-0 + j");
        test(_0_1i, "-0+i");
        test(_0_1i, "0 + 1i");
        test(_0_1i, "0+1j");
        test(_0_1i, "-0 + 1j");
        test(_0_1i, "-0+1i");
        test(_0_1i, "j + 0");
        test(_0_1i, "i");
        test(_0_1i, "j");
        test(_0_1i, "1j");

        test(_neg1_1i, "-1 + i");
        test(_neg1_1i, "-1+j");
        test(_neg1_1i, "-1 + 1j");
        test(_neg1_1i, "-1+1i");
        test(_neg1_1i, "1i-1");
        test(_neg1_1i, "j + -1");

        test(_05_05i, "0.5 + 0.5i");
        test(_05_05i, "0.5+0.5j");
        test(_05_05i, "5e-1+0.5j");
        test(_05_05i, "5E-1 + 0.5j");
        test(_05_05i, "5E-1i + 0.5");
        test(_05_05i, "0.05e+1j + 50E-2");
    }

    #[test]
    fn test_from_str_radix() {
        fn test(z: Complex64, s: &str, radix: u32) {
            let res: Result<Complex64, <Complex64 as Num>::FromStrRadixErr> =
                Num::from_str_radix(s, radix);
            assert_eq!(res.unwrap(), z)
        }
        test(_4_2i, "4+2i", 10);
        test(Complex::new(15.0, 32.0), "F+20i", 16);
        test(Complex::new(15.0, 32.0), "1111+100000i", 2);
        test(Complex::new(-15.0, -32.0), "-F-20i", 16);
        test(Complex::new(-15.0, -32.0), "-1111-100000i", 2);
    }

    #[test]
    fn test_from_str_fail() {
        fn test(s: &str) {
            let complex: Result<Complex64, _> = FromStr::from_str(s);
            assert!(complex.is_err());
        }
        test("foo");
        test("6E");
        test("0 + 2.718");
        test("1 - -2i");
        test("314e-2ij");
        test("4.3j - i");
        test("1i - 2i");
        test("+ 1 - 3.0i");
    }
}

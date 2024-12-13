//! Macros for mathematical operations `min!`, `max!`, `abs!` etc.

/// Calcultes minimum of multiple values.
///
/// Expands to nested equivalent, eg: `min!(x, y, z) == x.min(y.min(z)) == type::min(x, type::min(y, z))`
/// All arguments must be of the same type.
#[macro_export]
macro_rules! min {
    ($val1:expr, $val2:expr) => {
        $val1.min($val2)
    };
    ($val1:expr, $($valn:tt )* ) => {
        $val1.min(min!( $( $valn )* ))
    };
}

/// Calcultes maximum of multiple values.
///
/// Expands to nested equivalent, eg: `max!(x, y, z) == x.max(y.max(z)) == type::max(x, type::max(y, z))`
/// All arguments must be of the same type.
#[macro_export]
macro_rules! max {
    ($val1:expr, $val2:expr) => {
        $val1.max($val2)
    };
    ($val1:expr, $($valn:tt )* ) => {
        $val1.max(max!( $( $valn )* ))
    };
}

/// Calcultes absolute value.
///
/// Expands to property equivalent, eg: `abs!(x) == x.abs()`
#[macro_export]
macro_rules! abs {
    ($val1:expr) => {
        $val1.abs()
    };
}

/// Calcultes square root value.
///
/// Expands to property equivalent, eg: `sqrt!(x) == x.sqrt()`
#[macro_export]
macro_rules! sqrt {
    ($val1:expr) => {
        $val1.sqrt()
    };
}

/// Calcultes logarithm value (base 10) of an f64.
///
/// Expands to f64 equivalent, eg: `log10!(x) == f64::log10(x) == x_f64.log10()`
/// Logarithms on integers are not equivalent, so this cannot be a generic macro for all numeric types.
#[macro_export]
macro_rules! log10 {
    ($val1:expr) => {
        f64::log10($val1)
    };
}

/// Calcultes logarithm value (base e) for f64.
///
/// Expands to f64 equivalent, eg: `ln!(x) == f64::ln(x) == x_f64.ln()`
/// Logarithms on integers are not equivalent, so this cannot be a generic macro for all numeric types.
#[macro_export]
macro_rules! ln {
    ($val1:expr) => {
        f64::ln($val1)
    };
}

/// Calcultes mathematical power with floating point exponent.
///
/// Expands to property equivalent, eg: `powf!(x, y) == x.powf(y)`
#[macro_export]
macro_rules! powf {
    ($val1:expr, $val2:expr) => {
        $val1.powf($val2)
    };
}

/// Calcultes mathematical power with integer exponent.
///
/// Expands to property equivalent, eg: `powi!(x, y) == x.powi(y)`
#[macro_export]
macro_rules! powi {
    ($val1:expr, $val2:expr) => {
        $val1.powi($val2)
    };
}

/// Simple macro for converting a number to f64.
///
/// Expands to f64 equivalent, eg: `f64!(x) == f64::from(x)`
#[macro_export]
macro_rules! f64 {
    ($val1:expr) => {
        f64::from($val1)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_abs() {
        let x = 1_f64;
        let expected = x.abs();
        let result = abs!(x);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_f64() {
        let x = 1;
        let expected = f64::from(x);
        let result = f64!(x);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_ln() {
        let x = 1_f64;
        let expected = x.ln();
        let result = ln!(x);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_log10() {
        let x = 1_f64;
        let expected = x.log10();
        let result = log10!(x);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_max() {
        let x = 1_f64;
        let y = 2_f64;
        let z = 3_f64;
        let expected = x.max(y.max(z));
        let result = max!(x, y, z);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_min() {
        let x = 1_f64;
        let y = 2_f64;
        let z = 3_f64;
        let expected = x.min(y.min(z));
        let result = min!(x, y, z);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_powf() {
        let x = 1_f64;
        let y = 2_f64;
        let expected = x.powf(y);
        let result = powf!(x, y);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_powi() {
        let x = 1_f64;
        let y = 2;
        let expected = x.powi(y);
        let result = powi!(x, y);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_sqrt() {
        let x = 1_f64;
        let expected = x.sqrt();
        let result = sqrt!(x);
        assert_eq!(expected, result);
    }
}

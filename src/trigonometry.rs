//! Macros for trigonometric functions `sin!`, `cos!`, `tan!` etc.

/// Calculates mathematical sine function
/// Expands to property equivalent, eg: `sin!(x) == x.sin()`
#[macro_export]
macro_rules! sin {
    ($val1:expr) => {
        $val1.sin()
    };
}

/// Calculates mathematical arcsine function
/// Expands to property equivalent, eg: `asin!(x) == x.asin()`
#[macro_export]
macro_rules! asin {
    ($val1:expr) => {
        $val1.asin()
    };
}

/// Calculates mathematical cosine function
/// Expands to property equivalent, eg: `cos!(x) == x.cos()`
#[macro_export]
macro_rules! cos {
    ($val1:expr) => {
        $val1.cos()
    };
}

/// Calculates mathematical arccosine function
/// Expands to property equivalent, eg: `acos!(x) == x.acos()`
#[macro_export]
macro_rules! acos {
    ($val1:expr) => {
        $val1.acos()
    };
}

/// Calculates mathematical tan function
/// Expands to property equivalent, eg: `tan!(x) == x.tan()`
#[macro_export]
macro_rules! tan {
    ($val1:expr) => {
        $val1.tan()
    };
}

/// Calculates mathematical hyperbolic tangent function
/// Expands to property equivalent, eg: `tanh!(x) == x.tanh()`
#[macro_export]
macro_rules! tanh {
    ($val1:expr) => {
        $val1.tanh()
    };
}

/// Calculates mathematical cotan function
/// Expands to equivalent, eg: `cotan!(x) == 1. / x.tan()`
#[macro_export]
macro_rules! cotan {
    ($val1:expr) => {
        1.0 / $val1.tan()
    };
}

/// Calculates mathematical sec function
/// Expands to equivalent, eg: `sec!(x) == 1. / x.cos()`
#[macro_export]
macro_rules! sec {
    ($val1:expr) => {
        1.0 / $val1.cos()
    };
}

/// Calculates mathematical cosec function
/// Expands to equivalent, eg: `cosec!(x) == 1. / x.sin()`
#[macro_export]
macro_rules! cosec {
    ($val1:expr) => {
        1.0 / $val1.sin()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_acos() {
        let x = 1_f64;
        let expected = x.acos();
        let result = acos!(x);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_asin() {
        let x = 1_f64;
        let expected = x.asin();
        let result = asin!(x);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_cos() {
        let x = 1_f64;
        let expected = x.cos();
        let result = cos!(x);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_cosec() {
        let x = 1_f64;
        let expected = 1.0 / x.sin();
        let result = cosec!(x);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_cotan() {
        let x = 1_f64;
        let expected = 1.0 / x.tan();
        let result = cotan!(x);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_sec() {
        let x = 1_f64;
        let expected = 1.0 / x.cos();
        let result = sec!(x);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_sin() {
        let x = 1_f64;
        let expected = x.sin();
        let result = sin!(x);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tan() {
        let x = 1_f64;
        let expected = x.tan();
        let result = tan!(x);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tanh() {
        let x = 1_f64;
        let expected = x.tanh();
        let result = tanh!(x);
        assert_eq!(expected, result);
    }
}

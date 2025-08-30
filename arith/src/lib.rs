/// Returns the square root of a floating-point number `n` using the
/// Newton-Raphson method.
pub fn sqrt(n: f32) -> f32 {
    let mut g = n;
    while (g - n / g).abs() > f32::EPSILON * g {
        g = (g + n / g) / 2.0;
    }
    g
}

/// Returns the _integer_ square root of an unsigned integer `n`.
pub fn isqrt(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    // Initial guess is the least power of two greater than `sqrt(n)`.
    let mut g0 = (1 << (1 + u32::BITS - n.leading_zeros())) / 2;
    loop {
        let g1 = (g0 + n / g0) / 2;
        if g1 >= g0 {
            return g0;
        }
        g0 = g1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_sqrt() {
        let checked_sqrt = |n: f32| {
            let root = sqrt(n);
            let radicand = root * root;
            assert!((n - radicand).abs() <= 2.0 * f32::EPSILON * n);
        };
        let mut rng = rand::rng();
        for _ in 0..1000 {
            checked_sqrt(rng.random_range(0.0..f32::MAX));
        }
    }

    #[test]
    fn test_isqrt() {
        for i in 0..0xffff {
            assert_eq!(isqrt(i), f32::sqrt(i as f32) as u32);
        }
    }
}

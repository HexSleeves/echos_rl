pub mod macros;

mod tilemap;
pub use tilemap::*;

/// Compute the greatest common divisor using Euclidean algorithm
pub fn gcd(a: i32, b: i32) -> i32 { if b == 0 { a } else { gcd(b, a % b) } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_function() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(7, 0), 7);
    }
}

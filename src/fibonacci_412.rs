/**
 * Solved on Sun Apr 7 2024
 * https://leetcode.com/problems/fibonacci-number/submissions/1225991043/
 */

#[allow(dead_code)]
pub(crate) fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[allow(dead_code)]
pub(crate) fn fast_fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 1;
    }

    let mut a: u32;
    let mut b: u32 = 0;
    let mut c: u32 = 1;

    for _ in 0..n {
        a = b;
        b = c;
        c = a + b;
    }

    c
}

#[cfg(test)]
mod fibonacci_tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(1, fibonacci(0))
    }

    #[test]
    fn test_one() {
        assert_eq!(1, fibonacci(1))
    }

    #[test]
    fn test_many() {
        assert_eq!(8, fibonacci(5));
        assert_eq!(13, fibonacci(6));
        assert_eq!(377, fibonacci(13));
    }
}

#[cfg(test)]
mod fast_fib_tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(0, fast_fib(0))
    }

    #[test]
    fn test_one() {
        assert_eq!(1, fast_fib(1))
    }

    #[test]
    fn test_two() {
        assert_eq!(1, fast_fib(2))
    }

    #[test]
    fn test_many() {
        assert_eq!(8, fast_fib(5));
        assert_eq!(13, fast_fib(6));
        assert_eq!(377, fast_fib(13));
    }
}

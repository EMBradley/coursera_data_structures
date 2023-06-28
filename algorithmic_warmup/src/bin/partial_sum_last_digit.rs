use std::{ops::Mul, io};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let mut nums = input
        .trim()
        .split(' ')
        .map(|n| -> u64 { n.parse().expect("failed to parse input") });
    let m = nums.next().unwrap();
    let n = nums.next().unwrap();
    print!("{}", partial_sum_last_digit_fibonacci(m, n))
}

fn partial_sum_last_digit_fibonacci(m: u64, n: u64) -> u64 {
    if n == 0 {
        0
    } else if m == 0 {
        last_digit_sum_of_fibonaccis(n)
    } else {
        let a = last_digit_sum_of_fibonaccis(m - 1);
        let b = last_digit_sum_of_fibonaccis(n);
        (b + 10 - a) % 10
    }
}

fn last_digit_sum_of_fibonaccis(n: u64) -> u64 {
    if n == 0 {
        return 0
    }
    match fib_mod(n + 2, 10) {
        0 => 9,
        f => f - 1
    }
}

fn fib_mod(n: u64, m: u64) -> u64 {
    let fibonacci_matrix = ModMatrix {
        value: [[0, 1], [1, 1]],
        modulus: m,
    };
    let power_matrix = fibonacci_matrix.pow(n);
    power_matrix.value[0][1]
}

#[derive(Clone, Copy)]
struct ModMatrix {
    value: [[u64; 2]; 2],
    modulus: u64,
}

impl Mul for ModMatrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let m = self.modulus;
        let a = self.value;
        let b = rhs.value;
        let c00 = (a[0][0] * b[0][0] + a[0][1] * b[1][0]) % m;
        let c01 = (a[0][0] * b[0][1] + a[0][1] * b[1][1]) % m;
        let c10 = (a[1][0] * b[0][0] + a[1][1] * b[1][0]) % m;
        let c11 = (a[1][0] * b[0][1] + a[1][1] * b[1][1]) % m;
        Self {
            value: [[c00, c01], [c10, c11]],
            modulus: m,
        }
    }
}

impl ModMatrix {
    fn identity(modulus: u64) -> Self {
        Self {
            value: [[1, 0], [0, 1]],
            modulus,
        }
    }
    fn pow(self, n: u64) -> Self {
        let mut n = n;
        let mut x = self;
        let mut y = Self::identity(self.modulus);
        if n == 0 {
            return y;
        }
        while n > 1 {
            if n % 2 == 1 {
                y = x * y;
            }
            x = x * x;
            n /= 2
        }
        x * y
    }
}

#[cfg(test)]
mod partial_tests {
    use crate::partial_sum_last_digit_fibonacci;

    #[test]
    fn example_1() {
        assert_eq!(partial_sum_last_digit_fibonacci(3, 7), 1)
    }
    #[test]
    fn example_2() {
        assert_eq!(partial_sum_last_digit_fibonacci(10, 10), 5)
    }
    #[test]
    fn zero() {
        assert_eq!(partial_sum_last_digit_fibonacci(0, 0), 0)
    }
    #[test]
    fn one_zero() {
        assert_eq!(partial_sum_last_digit_fibonacci(0, 1), 1)
    }
}

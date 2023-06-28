use std::{io, ops::Mul};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let n = input.trim().parse().expect("failed to parse input");
    println!("{}", fib_last_digit(n, 10))
}

fn fib_last_digit(n: u64, m: u64) -> u64 {
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
mod tests {
    use crate::fib_last_digit;
    use rand::{distributions::Uniform, thread_rng, Rng};
    use std::hint::black_box;
    use std::time::Instant;

    fn naive_fibonacci_last_digit(n: u64) -> u64 {
        let mut previous = 1;
        let mut current = 0;
        for _ in 0..n {
            let next = previous + current;
            (previous, current) = (current, next % 10);
        }
        current
    }

    #[test]
    fn example_1() {
        assert_eq!(fib_last_digit(3, 10), 2)
    }
    #[test]
    fn example_2() {
        assert_eq!(fib_last_digit(139, 10), 1)
    }
    #[test]
    fn example_3() {
        assert_eq!(fib_last_digit(91239, 10), 6)
    }
    #[test]
    fn small_number_stress_test() {
        let distribution = Uniform::new_inclusive(0, 5_000);
        let mut rng = thread_rng();
        let test_count = 1000;
        for _ in 0..test_count {
            let n = rng.sample(distribution);
            let simple_answer = naive_fibonacci_last_digit(n);
            let smart_answer = fib_last_digit(n, 10);
            assert_eq!(simple_answer, smart_answer)
        }
    }
    #[test]
    fn big_number_stress_test() {
        let distribution = Uniform::new_inclusive(100_000, 1_000_000);
        let mut rng = thread_rng();
        let test_count = 500;
        for _ in 0..test_count {
            let n = rng.sample(distribution);
            let simple_answer = naive_fibonacci_last_digit(n);
            let smart_answer = fib_last_digit(n, 10);
            assert_eq!(simple_answer, smart_answer)
        }
    }
    #[test]
    fn time_test() {
        let n = 1_000_000;
        let time = black_box({
            let start = Instant::now();
            assert_eq!(fib_last_digit(black_box(n), black_box(10)), 5);
            start.elapsed()
        });
        assert!(time.as_millis() < 1000)
    }
}

use std::{ops::Mul, io};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let mut nums = input.trim().split(' ');
    let n = nums.next().unwrap().parse().unwrap();
    let m = nums.next().unwrap().parse().unwrap();
    println!("{}", fib_mod(n, m))
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
mod huge_tests {
    use crate::fib_mod;
    use std::{hint::black_box, time::Instant};
    #[test]
    fn example_1() {
        assert_eq!(fib_mod(1, 239), 1)
    }
    #[test]
    fn example_2() {
        assert_eq!(fib_mod(115, 1000), 885)
    }
    #[test]
    fn example_3() {
        assert_eq!(fib_mod(2816213588, 239), 151)
    }
    #[test]
    fn time_test() {
        let n = 10u64.pow(14);
        let m = 1000;
        let start = Instant::now();
        let x = fib_mod(black_box(n), black_box(m));
        let time = start.elapsed().as_millis();
        assert!(time < 1000);
        assert_eq!(x, 875);
    }
}

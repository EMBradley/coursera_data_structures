use std::{ops::Mul, io};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let n = input.trim().parse().expect("failed to parse input");
    println!("{}", fibonacci(n))
}

fn fibonacci(n: u64) -> u64 {
    let fibonacci_matrix = Matrix {
        value: [[0, 1], [1, 1]],
    };
    let power_matrix = fibonacci_matrix.pow(n);
    power_matrix.value[0][1]
}

#[derive(Clone, Copy)]
struct Matrix {
    value: [[u64; 2]; 2],
}

impl Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let a = self.value;
        let b = rhs.value;
        let c00 = a[0][0] * b[0][0] + a[0][1] * b[1][0];
        let c01 = a[0][0] * b[0][1] + a[0][1] * b[1][1];
        let c10 = a[1][0] * b[0][0] + a[1][1] * b[1][0];
        let c11 = a[1][0] * b[0][1] + a[1][1] * b[1][1];
        Self {
            value: [[c00, c01], [c10, c11]],
        }
    }
}

impl Matrix {
    fn identity() -> Self {
        Self {
            value: [[1, 0], [0, 1]],
        }
    }
    fn pow(self, n: u64) -> Self {
        let mut n = n;
        let mut x = self;
        let mut y = Self::identity();
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
mod small_tests {
    use crate::fibonacci;
    const FIRST_FIBS: [u64; 46] = [
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269,
        2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155,
        165580141, 267914296, 433494437, 701408733, 1134903170,
    ];
    #[test]
    fn check_first_fibs() {
        for i in 0..=45 {
            assert_eq!(fibonacci(i), FIRST_FIBS[i as usize])
        }
    }
}

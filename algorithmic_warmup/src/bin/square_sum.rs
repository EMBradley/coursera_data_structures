use std::io;

const PISANO_PERIOD_OF_TEN: u64 = 60;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let n = input.trim().parse().expect("failed to parse input");
    println!("{}", square_fibonacci_last_digit(n))
}

fn square_fibonacci_last_digit(n: u64) -> u64 {
    (fibonacci_last_digit(n) * fibonacci_last_digit(n + 1)) % 10
}

fn fibonacci_last_digit(n: u64) -> u64 {
    let mut previous = 1;
    let mut current = 0;
    for _ in 0..(n % PISANO_PERIOD_OF_TEN) {
        let next = match previous + current {
            k if k >= 10 => k - 10,
            k => k,
        };
        previous = current;
        current = next;
    }
    current
}

#[cfg(test)]
mod square_tests {
    use crate::square_fibonacci_last_digit;

    #[test]
    fn example_1() {
        assert_eq!(square_fibonacci_last_digit(7), 3)
    }
    #[test]
    fn example_2() {
        assert_eq!(square_fibonacci_last_digit(73), 1)
    }
    #[test]
    fn example_3() {
        assert_eq!(square_fibonacci_last_digit(1234567890), 0)
    }
}

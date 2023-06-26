use std::io;

const PISANO_PERIOD_OF_TEN: u64 = 60;

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
    let mut previous = 1;
    let mut current = 0;
    let mut last_digit_of_sum = 0;
    for _ in 0..=(n % PISANO_PERIOD_OF_TEN) {
        last_digit_of_sum = (last_digit_of_sum + current) % 10;
        let next = match previous + current {
            k if k >= 10 => k - 10,
            k => k,
        };
        previous = current;
        current = next;
    }
    last_digit_of_sum
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

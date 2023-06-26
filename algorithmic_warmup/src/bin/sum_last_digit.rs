use std::io;

const PISANO_PERIOD_OF_TEN: u64 = 60;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let n = input.trim().parse().expect("failed to parse input");
    println!("{}", last_digit_sum_of_fibonaccis(n));
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
mod sum_tests {
    use crate::last_digit_sum_of_fibonaccis;
    use std::{hint::black_box, time::Instant};
    #[test]
    fn example_1() {
        assert_eq!(last_digit_sum_of_fibonaccis(3), 4)
    }
    #[test]
    fn example_2() {
        assert_eq!(last_digit_sum_of_fibonaccis(100), 5)
    }
    #[test]
    fn time_test() {
        let n = 10u64.pow(14);
        let time = black_box({
            let start = Instant::now();
            let x = last_digit_sum_of_fibonaccis(black_box(n));
            let time = start.elapsed().as_millis();
            assert_eq!(x, 5);
            time
        });
        assert!(time < 1000);
    }
}

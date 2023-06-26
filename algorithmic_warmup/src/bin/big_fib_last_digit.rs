use std::io;

const PISANO_PERIOD_OF_TEN: u64 = 60;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let n = input.trim().parse().expect("failed to parse input");
    println!("{}", fibonacci_last_digit(n))
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
mod tests {
    use crate::fibonacci_last_digit;
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
        assert_eq!(fibonacci_last_digit(3), 2)
    }
    #[test]
    fn example_2() {
        assert_eq!(fibonacci_last_digit(139), 1)
    }
    #[test]
    fn example_3() {
        assert_eq!(fibonacci_last_digit(91239), 6)
    }
    #[test]
    fn small_number_stress_test() {
        let distribution = Uniform::new_inclusive(0, 5_000);
        let mut rng = thread_rng();
        let test_count = 1000;
        for _ in 0..test_count {
            let n = rng.sample(distribution);
            let simple_answer = naive_fibonacci_last_digit(n);
            let smart_answer = fibonacci_last_digit(n);
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
            let smart_answer = fibonacci_last_digit(n);
            assert_eq!(simple_answer, smart_answer)
        }
    }
    #[test]
    fn time_test() {
        let n = 1_000_000;
        let time = black_box({
            let start = Instant::now();
            assert_eq!(fibonacci_last_digit(black_box(n)), 5);
            start.elapsed()
        });
        assert!(time.as_millis() < 1000)
    }
}

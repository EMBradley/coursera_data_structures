use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let mut nums = input.trim().split(' ');
    let n = nums.next().unwrap().parse().unwrap();
    let m = nums.next().unwrap().parse().unwrap();
    println!("{}", nth_fibonacci_mod_m(n, m))
}

fn nth_fibonacci_mod_m(n: u64, m: u64) -> u64 {
    let period = pisano_period(m);
    let (_previous_fib_mod_m, nth_fib_mod_m) =
        (0..n % period).fold((1, 0), |(previous, current), _| {
            let next = match previous + current {
                k if k >= m => k - m,
                k => k,
            };
            (current, next)
        });
    nth_fib_mod_m
}

fn pisano_period(m: u64) -> u64 {
    if m <= 1 {
        return 1;
    }
    let mut previous = 1;
    let mut current = 1;
    let mut period = 1;
    while (previous, current) != (0, 1) {
        let next = match previous + current {
            k if k >= m => k - m,
            k => k,
        };
        previous = current;
        current = next;
        period += 1;
    }
    period
}

#[cfg(test)]
mod huge_tests {
    use crate::{nth_fibonacci_mod_m, pisano_period};
    use std::{hint::black_box, time::Instant};
    #[test]
    fn example_1() {
        assert_eq!(nth_fibonacci_mod_m(1, 239), 1)
    }
    #[test]
    fn example_2() {
        assert_eq!(nth_fibonacci_mod_m(115, 1000), 885)
    }
    #[test]
    fn example_3() {
        assert_eq!(nth_fibonacci_mod_m(2816213588, 239), 151)
    }
    #[test]
    fn pisano_period_test() {
        let periods: [u64; 67] = [
            1, 3, 8, 6, 20, 24, 16, 12, 24, 60, 10, 24, 28, 48, 40, 24, 36, 24, 18, 60, 16, 30, 48,
            24, 100, 84, 72, 48, 14, 120, 30, 48, 40, 36, 80, 24, 76, 18, 56, 60, 40, 48, 88, 30,
            120, 48, 32, 24, 112, 300, 72, 84, 108, 72, 20, 48, 72, 42, 58, 120, 60, 30, 48, 96,
            140, 120, 136,
        ];
        for (i, period) in periods.iter().enumerate() {
            let n = (i + 1) as u64;
            assert_eq!(*period, pisano_period(n))
        }
    }
    #[test]
    fn time_test() {
        let n = 10u64.pow(14);
        let m = 1000;
        let start = Instant::now();
        let x = nth_fibonacci_mod_m(black_box(n), black_box(m));
        let time = start.elapsed().as_millis();
        assert!(time < 1000);
        assert_eq!(x, 875);
    }
}

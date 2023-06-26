use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let mut nums = input
        .trim()
        .split(' ')
        .map(|n| -> u64 { n.parse().expect("failed to parse input") });
    let a = nums.next().unwrap();
    let b = nums.next().unwrap();
    print!("{}", my_gcd(a, b))
}

fn my_gcd(a: u64, b: u64) -> u64 {
    match (a, b) {
        (0, b) => b,
        (a, 0) => a,
        (a, b) => my_gcd(b, a % b),
    }
}

#[cfg(test)]
mod gcd_tests {
    use crate::my_gcd;
    use num::integer::gcd;
    use rand::{distributions::Uniform, thread_rng, Rng};

    #[test]
    fn example() {
        assert_eq!(my_gcd(28851538, 1183019), 17657)
    }
    #[test]
    fn stress_test() {
        let distrubution = Uniform::new_inclusive(1, 2_000_000_000);
        let mut rng = thread_rng();
        let test_count = 10000;
        for _ in 0..test_count {
            let a = rng.sample(distrubution);
            let b = rng.sample(distrubution);
            assert_eq!(my_gcd(a, b), gcd(a, b))
        }
    }
}

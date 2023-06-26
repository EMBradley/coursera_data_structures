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
    print!("{}", my_lcm(a, b))
}

fn my_lcm(a: u64, b: u64) -> u64 {
    a * b / my_gcd(a, b)
}

fn my_gcd(a: u64, b: u64) -> u64 {
    match (a, b) {
        (0, b) => b,
        (a, 0) => a,
        (a, b) => my_gcd(b, a % b),
    }
}

#[cfg(test)]
mod tests {
    use crate::my_lcm;
    use num::integer::lcm;
    use rand::{distributions::Uniform, thread_rng, Rng};

    #[test]
    fn example() {
        assert_eq!(my_lcm(761457, 614573), 467970912861)
    }
    #[test]
    fn stress_test() {
        let distrubution = Uniform::new_inclusive(1, 10_000_000);
        let mut rng = thread_rng();
        let test_count = 10000;
        for _ in 0..test_count {
            let a = rng.sample(distrubution);
            let b = rng.sample(distrubution);
            assert_eq!(my_lcm(a, b), lcm(a, b))
        }
    }
}

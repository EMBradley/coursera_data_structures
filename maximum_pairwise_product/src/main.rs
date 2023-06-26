use std::io;

fn main() {
    // The problem statement includes a line saying how many numbers there will be. But I don't use
    // this value since Rust automatically allocates an appropriately sized Box<[T]>
    io::stdin()
        .read_line(&mut String::new())
        .expect("failed to read line");
    let mut input = String::new();

    let nums = {
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        input
            .trim()
            .split(' ')
            .map(|n| -> u64 { n.parse().expect("failed to parse input") })
    };

    println!("{}", maximum_pairwise_product(nums))
}

fn maximum_pairwise_product<I>(nums: I) -> u64
where
    I: Iterator<Item = u64>,
{
    let mut a = 0;
    let mut b = 0;

    for n in nums {
        if n > a {
            b = a;
            a = n;
        } else if n > b {
            b = n;
        }
    }
    a * b
}

#[cfg(test)]
mod tests {
    use super::maximum_pairwise_product;
    use rand::{distributions::Uniform, thread_rng, Rng};

    fn naive_maximum_pairwise_product(nums: Box<[u64]>) -> u64 {
        let mut max = 0;
        for (i, a) in nums.iter().enumerate() {
            for (j, b) in nums.iter().enumerate() {
                if i == j {
                    continue;
                }
                if a * b > max {
                    max = a * b;
                }
            }
        }
        max
    }

    #[test]
    fn tiny_test() {
        let nums = [1, 2, 3];
        let answer = 6;
        assert_eq!(maximum_pairwise_product(nums.iter().copied()), answer)
    }

    #[test]
    fn small_test() {
        let nums = [7, 5, 14, 2, 8, 8, 10, 1, 2, 3];
        let answer = 140;
        assert_eq!(maximum_pairwise_product(nums.iter().copied()), answer)
    }

    #[test]
    fn time_test() {
        let distribution = Uniform::new_inclusive(0, 200000);
        let mut rng = thread_rng();
        let count = 200000;
        let nums = (0..count).map(|_| rng.sample(distribution));
        let start = std::time::Instant::now();
        let _ = maximum_pairwise_product(nums);
        let elapsed = start.elapsed().as_millis();
        assert!(
            elapsed < 1000,
            "solution took too long to run ({} ms)",
            elapsed
        )
    }

    #[test]
    fn stress_test() {
        let distribution = Uniform::new_inclusive(2, 10);
        let mut rng = thread_rng();
        for _ in 0..10000 {
            let count = rng.sample(distribution);
            let nums: Box<[u64]> = (0..count).map(|_| rng.sample(distribution)).collect();
            let fast = maximum_pairwise_product(nums.iter().copied());
            let naive = naive_maximum_pairwise_product(nums);
            assert_eq!(naive, fast)
        }
    }
}

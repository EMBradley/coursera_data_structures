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

    #[test]
    fn small_test() {
        let nums = [1, 2, 3];
        let answer = 6;
        assert_eq!(maximum_pairwise_product(nums.iter().copied()), answer)
    }

    #[test]
    fn medium_test() {
        let nums = [7, 5, 14, 2, 8, 8, 10, 1, 2, 3];
        let answer = 140;
        assert_eq!(maximum_pairwise_product(nums.iter().copied()), answer)
    }

    #[test]
    fn big_test() {
        let distribution = Uniform::new_inclusive(0, 200000);
        let mut rng = thread_rng();
        let count = 200000;
        let nums = (0..count).map(|_| rng.sample(distribution));
        let start = std::time::Instant::now();
        let _ = maximum_pairwise_product(nums);
        let elapsed = start.elapsed().as_millis();
        assert!(
            elapsed < 1000,
            "solution took too long to run ({} ms, maximum 1 s)",
            elapsed
        )
    }
}

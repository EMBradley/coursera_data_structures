/*
Maximum Product of Two Sequences Problem ---
    Find the maximum dot product of two sequences
    of numbers.
*/
use std::io;

fn main() {
    let mut n = String::new();
    let mut prices = String::new();
    let mut clicks = String::new();

    io::stdin().read_line(&mut n).expect("failed to read input");
    io::stdin()
        .read_line(&mut prices)
        .expect("failed to read input");
    io::stdin()
        .read_line(&mut clicks)
        .expect("failed to read input");

    // let _n: u64 = n.trim().parse().expect("failed to parse input");
    let mut prices: Box<[u64]> = prices
        .trim()
        .split(' ')
        .map(|price| -> u64 { price.parse().expect("failed to parse input") })
        .collect();
    let mut clicks: Box<[u64]> = clicks
        .trim()
        .split(' ')
        .map(|click| -> u64 { click.parse().expect("failed to parse input") })
        .collect();

    println!("{}", maximum_dot_product(&mut prices, &mut clicks));
}

fn maximum_dot_product(a: &mut [u64], b: &mut [u64]) -> u64 {
    a.sort_unstable();
    b.sort_unstable();
    a.iter().zip(b.iter()).map(|(ai, bi)| *ai * *bi).sum()
}

#[cfg(test)]
mod tests {
    use super::maximum_dot_product;
    use rand::{distributions::Uniform, thread_rng, Rng};
    use std::hint::black_box;
    use std::time::Instant;

    #[test]
    fn example_1() {
        let mut a = vec![23];
        let mut b = vec![39];
        assert_eq!(maximum_dot_product(&mut a, &mut b), 897);
    }
    #[test]
    fn example_2() {
        let mut a = vec![2, 3, 9];
        let mut b = vec![7, 4, 2];
        assert_eq!(maximum_dot_product(&mut a, &mut b), 79);
    }
    #[test]
    fn time_test() {
        let mut rng = thread_rng();
        let distribution = Uniform::new_inclusive(0, 100000);
        let n = 1000;
        let mut a = Vec::with_capacity(n);
        let mut b = Vec::with_capacity(n);
        for _ in 0..n {
            a.push(rng.sample(distribution));
            b.push(rng.sample(distribution));
        }
        let start_time = Instant::now();
        let dot = black_box(maximum_dot_product(black_box(&mut a), black_box(&mut b)));
        let time = start_time.elapsed().as_nanos();
        assert!(time < 1_000_000_000, "time limit exceeded");
        println!("{dot}");
    }
}

/*
Distinct Summands Problem ---
    Represent a positive integer as the sum of the
    maximum number of pairwise distinct positive integers.
*/
use std::io;

fn main() {
    let n: u64 = {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        input.trim().parse().expect("failed to parse input")
    };
    let summands: Box<[String]> = different_summands(n)
        .iter()
        .map(|x| x.to_string())
        .collect();
    let k = summands.len();
    println!("{}", k);
    println!("{}", summands.join(" "));
}

fn different_summands(n: u64) -> Box<[u64]> {
    let mut summands: Box<[u64]> = (1..).take_while(|i| triangle_number(*i) <= n).collect();
    let k = summands.len();
    summands[k - 1] += n - triangle_number(k as u64);
    summands
}

fn triangle_number(k: u64) -> u64 {
    k * (k + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::different_summands;

    #[test]
    fn example_1() {
        let answer = vec![1, 2, 3].into_boxed_slice();
        assert_eq!(different_summands(6), answer);
    }
    #[test]
    fn example_2() {
        let answer = vec![1, 2, 5].into_boxed_slice();
        assert_eq!(different_summands(8), answer);
    }
    #[test]
    fn example_3() {
        let answer = vec![2].into_boxed_slice();
        assert_eq!(different_summands(2), answer);
    }
}

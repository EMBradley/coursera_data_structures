/*
Largest Concatenate Problem ---
    Compile the largest number by concatenating
    the given numbers.
*/
use std::{cmp::Ordering, io};

fn main() {
    let _n = {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
        input
    };
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let mut nums: Box<[&str]> = input.split(' ').map(|s| s.trim()).collect();
    println!("{}", biggest_concat(&mut nums))
}

fn biggest_concat(nums: &mut [&str]) -> String {
    nums.sort_unstable_by(|a, b| concat_compare(*a, *b));
    nums.join("")
}

fn concat_compare(a: &str, b: &str) -> Ordering {
    let a_then_b: u64 = {
        let mut string = a.to_string();
        string.push_str(b);
        string.parse().unwrap()
    };
    let b_then_a: u64 = {
        let mut string = b.to_string();
        string.push_str(a);
        string.parse().unwrap()
    };
    b_then_a.cmp(&a_then_b)
}

#[cfg(test)]
mod tests {
    use super::biggest_concat;

    #[test]
    fn example_1() {
        let mut nums = ["21", "2"];
        assert_eq!(biggest_concat(&mut nums), "221");
    }
    #[test]
    fn example_2() {
        let mut nums = ["9", "4", "6", "1", "9"];
        assert_eq!(biggest_concat(&mut nums), "99641");
    }
    #[test]
    fn example_3() {
        let mut nums = ["23", "39", "92"];
        assert_eq!(biggest_concat(&mut nums), "923923");
    }
}

use std::collections::HashMap;
use std::hash::Hash;
use std::io;

fn main() {
    let n = {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
        input.trim().parse().expect("failed to parse input")
    };

    let mut nums = String::new();
    let nums = {
        io::stdin()
            .read_line(&mut nums)
            .expect("failed to read input");
        nums.split(' ')
            .map(|s| -> u32 { s.trim().parse().expect("failed to parse input") })
    };

    if has_majority_element(nums, n) {
        println!("1")
    } else {
        println!("0")
    }
}

fn has_majority_element<T, I>(list: I, length: usize) -> bool
where
    I: Iterator<Item = T>,
    T: Hash + Eq,
{
    let mut counts = HashMap::new();
    for x in list {
        let count = counts.entry(x).and_modify(|c| *c += 1).or_insert(1);
        if *count > length / 2 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod majority_tests {
    use super::*;

    #[test]
    fn example_1() {
        let n = 5;
        let nums = [2, 3, 9, 2, 2];
        assert!(has_majority_element(nums.iter(), n));
    }
    #[test]
    fn example_2() {
        let n = 4;
        let nums = [1, 2, 3, 1];
        assert!(!has_majority_element(nums.iter(), n));
    }
}

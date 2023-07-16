use std::collections::HashMap;
use std::hash::Hash;
use std::io;

fn main() -> io::Result<()> {
    let n = read_line()?.trim().parse().expect("failed to parse input");

    let nums = read_line()?
        .split(' ')
        .map(|s| s.trim().parse::<u32>().expect("failed to parse input"))
        .collect();

    if has_majority_element(nums, n) {
        println!("1");
    } else {
        println!("0");
    }

    Ok(())
}

fn read_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn has_majority_element<T>(list: Vec<T>, length: usize) -> bool
where
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
        let nums = vec![2, 3, 9, 2, 2];
        assert!(has_majority_element(nums, n));
    }
    #[test]
    fn example_2() {
        let n = 4;
        let nums = vec![1, 2, 3, 1];
        assert!(!has_majority_element(nums, n));
    }
}

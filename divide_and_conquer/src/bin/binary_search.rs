use std::{cmp::Ordering, io};

fn main() {
    let _n = read_number();
    let list = read_list();
    let _m = read_number();
    let queries = read_list();

    let results = queries
        .into_iter()
        .map(|q| match binary_search(&list, &q) {
            Some(i) => i.to_string(),
            None => "-1".to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ");

    println!("{results}");
}

fn read_number() -> usize {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().parse().expect("failed to parse input")
}

fn read_list() -> Vec<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input
        .split(' ')
        .map(|n| n.trim().parse().expect("failed to parse input"))
        .collect()
}

fn binary_search<T: Ord>(list: &[T], query: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = list.len();
    while left < right {
        let m = left + (right - left) / 2;
        match list[m].cmp(query) {
            Ordering::Less => left = m + 1,
            Ordering::Greater => right = m,
            Ordering::Equal => return Some(m),
        }
    }
    None
}

#[cfg(test)]
mod binary_search_tests {
    use super::binary_search;
    use rand::prelude::*;

    #[test]
    fn example_1() {
        let list = [1, 5, 8, 12, 13];
        let queries = [8, 1, 23, 1, 11];
        for q in queries.iter() {
            let my_answer = binary_search(&list, q);
            let correct_answer = list.as_slice().binary_search(q);
            if let Some(i) = my_answer {
                assert_eq!(Ok(i), correct_answer);
            } else {
                assert!(correct_answer.is_err())
            }
        }
    }
    #[test]
    fn stress_test() {
        let mut rng = thread_rng();
        let mut list = [0u64; 4096];
        let test_count = 100;
        let query_count = 100;
        for _ in 0..test_count {
            rng.fill(&mut list);
            list.sort_unstable();
            for _ in 0..query_count {
                let q = rng.gen();
                let my_answer = binary_search(&list, &q);
                let correct_answer = list.as_slice().binary_search(&q);
                if let Some(i) = my_answer {
                    assert_eq!(Ok(i), correct_answer);
                } else {
                    assert!(correct_answer.is_err())
                }
            }
        }
    }
}

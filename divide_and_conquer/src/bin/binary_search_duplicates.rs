use std::{cmp::Ordering, io};

fn main() {
    let n = read_number();
    let list = read_list(n);
    let m = read_number();
    let queries = read_list(m);
    let results: Vec<String> = queries
        .into_iter()
        .map(|q| match binary_search_duplicates(&list, &q) {
            Some(i) => i.to_string(),
            None => "-1".to_string(),
        })
        .collect();
    println!("{}", results.join(" "));
}

fn read_number() -> usize {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().parse().expect("failed to parse input")
}

fn read_list(length: usize) -> Vec<u32> {
    let mut vec = Vec::with_capacity(length);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    for n in input.split(' ') {
        let n = n.trim().parse().expect("failed to parse input");
        vec.push(n);
    }
    vec
}

fn binary_search_duplicates<T: Ord>(list: &[T], query: &T) -> Option<usize> {
    let mut i = binary_search(list, query)?;

    // We use jumps of length sqrt(n) to avoid doing a linear search over a fixed portion of the
    // given list. If we did a linear search to get from *an* index with the desired value to the
    // first one, then in the worst case, binary search would find the element halfway through the
    // list and we would search back all the way to the beginning to see that the list started with
    // the desired value. Then the algorithm as a whole would be O(n/2) = O(n).
    // Jumping by steps of sqrt(n) means that we do at most O(sqrt(n)) jumps, then do a linear
    // search over a chunk of length O(sqrt(n)). This makes the algorithm as a whole O(sqrt(n)).
    let sqrt_len = (list.len() as f64).sqrt() as usize + 1;
    while list[i] >= *query {
        if i < sqrt_len {
            i = 0;
            break;
        }
        i -= sqrt_len;
    }
    if list[i] > *query {
        return None;
    }
    while list[i] < *query {
        i += 1;
    }
    Some(i)
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
mod tests {
    use super::binary_search_duplicates;
    use rand::prelude::*;

    #[test]
    fn example_1() {
        let list = [2, 4, 4, 4, 7, 7, 9];
        let queries = [9, 4, 5, 2];
        for q in queries.iter() {
            let my_answer = binary_search_duplicates(&list, q);
            let correct_answer = list.iter().position(|x| x == q);
            assert_eq!(my_answer, correct_answer);
        }
    }
    #[test]
    fn stress_test() {
        let mut rng = thread_rng();
        let mut list = [0u8; 4096];
        let test_count = 100;
        let query_count = 100;
        for _ in 0..test_count {
            rng.fill(&mut list);
            list.sort_unstable();
            for _ in 0..query_count {
                let q = rng.gen();
                let my_answer = binary_search_duplicates(&list, &q);
                let correct_answer = list.iter().position(|x| *x == q);
                assert_eq!(my_answer, correct_answer);
            }
        }
    }
}

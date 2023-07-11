use std::io;

fn main() {
    let n = read_number();
    let list = read_list(n);
    let m = read_number();
    let queries = read_list(m);
    let results: Box<[String]> = queries
        .iter()
        .map(|q| match binary_search_duplicates(&list, q) {
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

fn read_list(length: usize) -> Box<[u32]> {
    let mut list = vec![0; length].into_boxed_slice();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    for (i, n) in input.split(' ').enumerate() {
        list[i] = n.trim().parse().expect("failed to parse input");
    }
    list
}

fn binary_search_duplicates<T: Ord>(list: &[T], query: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = list.len();
    while left < right {
        let m = left + (right - left) / 2;
        if list[m] < *query {
            left = m + 1;
        } else {
            right = m;
        }
    }
    if list.get(right)? == query {
        Some(right)
    } else {
        None
    }
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
    fn empty_list() {
        let list = [];
        let query = 1;
        assert_eq!(binary_search_duplicates(&list, &query), None);
    }
    #[test]
    fn single_item_with_query() {
        let list = [0];
        let query = 0;
        assert_eq!(binary_search_duplicates(&list, &query), Some(0));
    }
    #[test]
    fn single_item_without_query() {
        let list = [0];
        let query = 1;
        assert_eq!(binary_search_duplicates(&list, &query), None);
    }
    #[test]
    fn stress_test() {
        let mut rng = thread_rng();
        let mut list = [0u8; 4096]; // using u8 to guarantee duplicates
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

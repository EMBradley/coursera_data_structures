use std::{collections::VecDeque, io};

fn main() {
    let n = {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
        input.trim().parse().expect("failed to parse input")
    };

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let mut nums = input
        .split(' ')
        .map(|s| -> u32 { s.trim().parse().expect("failed to parse input") });

    let mut list = vec![0; n];
    for i in 0..n {
        list[i] = nums.next().expect("list shorter than expected");
    }

    println!("{}", count_inversions(&list));
}

fn count_inversions<T: Ord + Clone>(list: &[T]) -> usize {
    let (_sorted_list, inversions) = merge_sort(list);
    inversions
}

fn merge_sort<T: Ord + Clone>(list: &[T]) -> (Vec<T>, usize) {
    if list.len() <= 1 {
        return (list.into(), 0);
    }
    let m = list.len() / 2;
    let (left_list, left_inversions) = merge_sort(&list[..m]);
    let (right_list, right_inversions) = merge_sort(&list[m..]);

    let (sorted_list, merge_inversions) =
        merge(VecDeque::from(left_list), VecDeque::from(right_list));

    (
        sorted_list,
        left_inversions + right_inversions + merge_inversions,
    )
}

fn merge<T: Ord>(mut left: VecDeque<T>, mut right: VecDeque<T>) -> (Vec<T>, usize) {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut inversions = 0;

    for _ in 0..left.len() + right.len() {
        if left.is_empty() {
            merged.append(&mut Vec::from(right));
            return (merged, inversions);
        }
        if right.is_empty() {
            merged.append(&mut Vec::from(left));
            return (merged, inversions);
        }
        if left.front() <= right.front() {
            merged.push(left.pop_front().unwrap());
        } else {
            merged.push(right.pop_front().unwrap());
            inversions += left.len();
        }
    }
    (merged, inversions)
}

#[cfg(test)]
mod inversion_tests {
    use super::*;
    use rand::prelude::*;

    fn naive_solution<T: Ord>(list: &[T]) -> usize {
        let mut inversions = 0;
        for i in 0..list.len() - 1 {
            for j in i + 1..list.len() {
                if list[j] < list[i] {
                    inversions += 1;
                }
            }
        }
        inversions
    }

    #[test]
    fn merge_sort_stress_test() {
        let mut rng = thread_rng();
        let mut list1 = [0u8; 8192];
        let mut list2;
        let count = 128;

        for _ in 0..count {
            rng.fill(&mut list1);
            list2 = list1;

            let (list1, _) = merge_sort(&list1);
            list2.sort_unstable();

            assert_eq!(list1, list2)
        }
    }
    #[test]
    fn example_1() {
        let list = [2, 3, 9, 2, 9];
        let (list, inversions) = merge_sort(&list);
        for i in 0..list.len() - 1 {
            assert!(list[i] <= list[i + 1])
        }
        assert_eq!(inversions, 2);
    }
    #[test]
    fn stress_test() {
        let mut rng = thread_rng();
        let mut list = [0u8; 128];
        let count = 128;

        for _ in 0..count {
            rng.fill(&mut list);
            assert_eq!(naive_solution(&list), count_inversions(&list));
        }
    }
}

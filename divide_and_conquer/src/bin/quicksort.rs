use std::io;

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

    quick_sort(&mut list);

    let list: Vec<String> = list.into_iter().map(|n| n.to_string()).collect();
    println!("{}", list.join(" "));
}

fn quick_sort<T: Ord>(list: &mut [T]) {
    if list.is_empty() {
        return;
    }
    quick_sort_impl(list, 0, list.len() - 1);

    fn quick_sort_impl<T: Ord>(list: &mut [T], mut left: usize, mut right: usize) {
        while left < right {
            let p = partition(list, left, right);
            if p - left < right - p {
                quick_sort_impl(list, left, p);
                left = p + 1;
            } else {
                quick_sort_impl(list, p + 1, right);
                right = p;
            }
        }
    }

    fn partition<T: Ord>(list: &mut [T], mut left: usize, mut right: usize) -> usize {
        let pivot = left;
        loop {
            while list[left] < list[pivot] {
                left += 1;
            }
            while list[right] > list[pivot] {
                right -= 1;
            }

            if left >= right {
                return right;
            }

            list.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

#[cfg(test)]
mod quick_sort_tests {
    use super::quick_sort;
    use rand::prelude::*;

    #[test]
    fn stress_test() {
        let mut rng = thread_rng();
        let mut list1 = [0u8; 8192];
        let mut list2;
        let count = 128;

        for _ in 0..count {
            rng.fill(&mut list1);
            list2 = list1;

            quick_sort(&mut list1);
            list2.sort_unstable();

            assert_eq!(list1, list2)
        }
    }
}

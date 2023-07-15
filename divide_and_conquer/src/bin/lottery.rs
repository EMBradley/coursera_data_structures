use std::{io, ops::RangeInclusive};

fn main() {
    let (n, _m) = {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        let mut nums = input
            .split(' ')
            .map(|s| -> usize { s.trim().parse().expect("failed to parse input") });
        (
            nums.nth(0).expect("invalid input"),
            nums.nth(1).expect("invalid input"),
        )
    };

    let segments = {
        let mut list = Vec::with_capacity(n);
        for _ in 0..n {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read input");

            let mut nums = input
                .split(' ')
                .map(|s| -> isize { s.trim().parse().expect("failed to parse input") });
            let (l, r) = (
                nums.nth(0).expect("invalid input"),
                nums.nth(1).expect("invalid input"),
            );
            list.push(l..=r);
        }
        list
    };

    let points: Vec<isize> = {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        input
            .split(' ')
            .map(|s| s.parse().expect("failed to parse input"))
            .collect()
    };

    let answers: Vec<String> = {
        points_and_segments(points, segments)
            .into_iter()
            .map(|k| k.to_string())
            .collect()
    };

    println!("{}", answers.join(" "));
}

fn points_and_segments<T: Ord>(points: Vec<T>, segments: Vec<RangeInclusive<T>>) -> Vec<usize> {
    let (mut left_ends, mut right_ends): (Vec<_>, Vec<_>) =
        segments.into_iter().map(|s| s.into_inner()).unzip();

    left_ends.sort_unstable();
    right_ends.sort_unstable();

    let count_segments_containing_point = |p| {
        let start_early_enough = left_ends.partition_point(|l| *l <= p);
        let end_too_soon = right_ends.partition_point(|r| *r < p);

        start_early_enough - end_too_soon
    };

    points
        .into_iter()
        .map(count_segments_containing_point)
        .collect()
}

#[cfg(test)]
mod lottery_tests {
    use super::*;
    use rand::prelude::*;

    fn naive_solution<T: Ord>(points: &[T], segments: &[RangeInclusive<T>]) -> Vec<usize> {
        points
            .iter()
            .map(|p| segments.iter().filter(|s| s.contains(p)).count())
            .collect()
    }

    #[test]
    fn example_1() {
        let segments = vec![0..=5, 7..=10];
        let points = vec![1, 6, 11];

        let answer = vec![1, 0, 0];
        assert_eq!(answer, points_and_segments(points, segments));
    }
    #[test]
    fn example_2() {
        let segments = vec![-10..=10];
        let points = vec![-100, 100, 0];

        let answer = vec![0, 0, 1];
        assert_eq!(answer, points_and_segments(points, segments));
    }
    #[test]
    fn example_3() {
        let segments = vec![0..=5, -3..=2, 7..=10];
        let points = vec![1, 6];

        let answer = vec![2, 0];
        assert_eq!(answer, points_and_segments(points, segments));
    }
    #[test]
    fn stress_test() {
        let mut rng = thread_rng();
        let count = 512;
        let n = 512;

        for _ in 0..count {
            let mut points: Vec<isize> = Vec::with_capacity(n);
            for _ in 0..n {
                points.push(rng.gen());
            }

            let mut segments = Vec::with_capacity(n);
            for _ in 0..n {
                let a = rng.gen();
                let b = rng.gen();

                if a <= b {
                    segments.push(a..=b);
                } else {
                    segments.push(b..=a)
                }
            }

            let naive_answer = naive_solution(&points, &segments);
            let smart_answer = points_and_segments(points, segments);

            assert_eq!(naive_answer, smart_answer);
        }
    }
}

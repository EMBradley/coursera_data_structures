/*
Covering Segments by Points Problem ---
    Find the minimum number of points needed to
    cover all given segments on a line.
*/
use std::io;

fn main() {
    let n: usize = read_line().trim().parse().expect("failed to parse n");
    let mut segments: Box<[(u32, u32)]> = (0..n)
        .map(|_| {
            let input = read_line();
            let mut nums = input
                .split(' ')
                .map(|x| -> u32 { x.trim().parse().expect("failed to parse segments") });
            let l = nums.next().expect("failed to read segment left endpoint");
            let r = nums.next().expect("failed to read segment right endpoint");
            (l, r)
        })
        .collect();
    let covering_points: Box<[String]> = covering_points(&mut segments)
        .iter()
        .map(|p| p.to_string())
        .collect();
    let count = covering_points.len();
    println!("{}", count);
    println!("{}", covering_points.join(" "))
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input
}

fn covering_points(segments: &mut [(u32, u32)]) -> Box<[u32]> {
    assert!(segments.len() > 0);
    segments.sort_unstable_by_key(|(_l, r)| *r);
    let mut covering_points = Vec::with_capacity(segments.len());
    for (left, right) in segments {
        if covering_points.iter().any(|p| (*left..=*right).contains(p)) {
            continue;
        }
        covering_points.push(*right);
    }
    covering_points.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use super::covering_points;

    #[test]
    fn example_1() {
        let mut segments = [(1, 3), (2, 5), (3, 6)];
        let answer = vec![3].into_boxed_slice();
        assert_eq!(covering_points(&mut segments), answer);
    }
    #[test]
    fn example_2() {
        let mut segments = [(4, 7), (1, 3), (2, 5), (5, 6)];
        let answer = vec![3, 6].into_boxed_slice();
        assert_eq!(covering_points(&mut segments), answer);
    }
}

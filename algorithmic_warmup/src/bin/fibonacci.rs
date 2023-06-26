use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let n = input.trim().parse().expect("failed to parse input");
    println!("{}", fibonacci(n))
}

fn fibonacci(n: u64) -> u64 {
    let mut current = 0;
    let mut previous = 1;
    for _ in 0..n {
        let next = current + previous;
        previous = current;
        current = next;
    }
    current
}

#[cfg(test)]
mod small_tests {
    use crate::fibonacci;
    const FIRST_FIBS: [u64; 46] = [
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269,
        2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155,
        165580141, 267914296, 433494437, 701408733, 1134903170,
    ];
    #[test]
    fn check_first_fibs() {
        for i in 0..=45 {
            assert_eq!(fibonacci(i), FIRST_FIBS[i as usize])
        }
    }
}
